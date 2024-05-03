// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// ----- Modules -----

#![no_std]

// ----- Crates -----

use heapless::{String, Vec};
pub use hid_io_protocol::commands::*;
pub use hid_io_protocol::*;
use kll_core::TriggerEvent;
use pkg_version::*;

#[cfg(feature = "defmt")]
use defmt::trace;
#[cfg(not(feature = "defmt"))]
use log::trace;

// ----- Sizes -----

pub const MESSAGE_LEN: usize = 256;

// ----- General Structs -----

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HidIoHostInfo {
    pub major_version: u16,
    pub minor_version: u16,
    pub patch_version: u16,
    pub os: u8,
    pub os_version: String<256>,
    pub host_software_name: String<256>,
}

// ----- Enums -----

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HidIoEvent {
    TriggerEvent(TriggerEvent),
}

// ----- Command Interface -----

pub struct CommandInterface<
    KINTF: KiibohdCommandInterface<H>,
    const TX: usize,
    const RX: usize,
    const N: usize,
    const H: usize,
    const S: usize,
    const ID: usize,
> {
    ids: Vec<HidIoCommandId, ID>,
    pub rx_bytebuf: buffer::Buffer<RX, N>,
    rx_packetbuf: HidIoPacketBuffer<H>,
    pub tx_bytebuf: buffer::Buffer<TX, N>,
    serial_buf: Vec<u8, S>,
    hostinfo: HidIoHostInfo,
    term_out_buffer: String<H>,
    interface: KINTF,
}

impl<
        KINTF: KiibohdCommandInterface<H>,
        const TX: usize,
        const RX: usize,
        const N: usize,
        const H: usize,
        const S: usize,
        const ID: usize,
    > CommandInterface<KINTF, TX, RX, N, H, S, ID>
{
    pub fn new(
        ids: &[HidIoCommandId],
        interface: KINTF,
    ) -> Result<CommandInterface<KINTF, TX, RX, N, H, S, ID>, CommandError> {
        // Make sure we have a large enough id vec
        let ids = match Vec::from_slice(ids) {
            Ok(ids) => ids,
            Err(_) => {
                return Err(CommandError::IdVecTooSmall);
            }
        };

        let tx_bytebuf = buffer::Buffer::new();
        let rx_bytebuf = buffer::Buffer::new();
        let rx_packetbuf = HidIoPacketBuffer::new();
        let serial_buf = Vec::new();
        let term_out_buffer = String::new();
        let hostinfo = HidIoHostInfo {
            major_version: 0,
            minor_version: 0,
            patch_version: 0,
            os: 0,
            os_version: String::new(),
            host_software_name: String::new(),
        };

        Ok(CommandInterface {
            ids,
            rx_bytebuf,
            rx_packetbuf,
            tx_bytebuf,
            serial_buf,
            hostinfo,
            term_out_buffer,
            interface,
        })
    }

    pub fn host_info_cached(&self) -> &HidIoHostInfo {
        &self.hostinfo
    }

    /// Reference to the customized interface
    /// The interface will likely have custom datastructures
    /// that you want access to (e.g. manufacturing toggles)
    pub fn interface(&self) -> &KINTF {
        &self.interface
    }

    /// Mut reference to the customized interface
    /// The interface will likely have custom datastructures
    /// that you want access to (e.g. manufacturing toggles)
    pub fn mut_interface(&mut self) -> &mut KINTF {
        &mut self.interface
    }

    /// Decode rx_bytebuf into a HidIoPacketBuffer
    /// Returns true if buffer ready, false if not
    pub fn rx_packetbuffer_decode(&mut self) -> Result<bool, CommandError> {
        loop {
            // Retrieve vec chunk
            if let Some(buf) = self.rx_bytebuf.dequeue() {
                trace!("rx_packetbuffer_decode: {:?}", buf);
                // Decode chunk
                match self.rx_packetbuf.decode_packet(&buf) {
                    Ok(_recv) => {
                        // Only handle buffer if ready
                        if self.rx_packetbuf.done {
                            trace!("rx_packetbuf: {:?}", self.rx_packetbuf);
                            // Handle sync packet type
                            match self.rx_packetbuf.ptype {
                                HidIoPacketType::Sync => {
                                    self.interface.hidio_sync_packet();
                                    self.rx_packetbuf.clear();
                                }
                                _ => {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        return Err(CommandError::PacketDecodeError(e));
                    }
                }
            } else {
                return Ok(false);
            }
        }
    }

    /// Process rx buffer until empty
    /// Handles flushing tx->rx, decoding, then processing buffers
    /// Returns the number of buffers processed
    pub fn process_rx(&mut self, count: u8) -> Result<u8, CommandError> {
        // Decode bytes into buffer
        let mut cur = 0;
        while (count == 0 || cur < count) && self.rx_packetbuffer_decode()? {
            // Process rx buffer
            let ret = self.rx_message_handling(self.rx_packetbuf.clone());

            // Clear buffer
            self.rx_packetbuf.clear();
            cur += 1;

            // We need to clear the failed packet before returning the error
            match ret {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(cur)
    }

    /// Flush the term buffer
    pub fn term_buffer_flush(&mut self) -> Result<(), CommandError> {
        // Send the buffer
        if self.term_out_buffer.len() > 0 {
            let output = self.term_out_buffer.clone();
            self.h0034_terminalout(h0034::Cmd { output }, true)?;
            self.term_out_buffer.clear();
        }
        Ok(())
    }

    /// Process incoming events through HID-IO
    /// This is the preferred mechanism to interact with HID-IO (if possible for your situation)
    pub fn process_event(&mut self, event: HidIoEvent) -> Result<(), CommandError> {
        trace!("process_event: {:?}", event);
        // TODO - Event handler
        match event {
            HidIoEvent::TriggerEvent(_event) => {}
        }
        Ok(())
    }
}

/// CommandInterface for Commands
/// TX - tx byte buffer size (in multiples of N)
/// RX - tx byte buffer size (in multiples of N)
/// N - Max payload length (HidIoPacketBuffer), used for default values
/// H - Max data payload length (HidIoPacketBuffer)
/// S - Serialization buffer size
/// ID - Max number of HidIoCommandIds
impl<
        KINTF: KiibohdCommandInterface<H>,
        const TX: usize,
        const RX: usize,
        const N: usize,
        const H: usize,
        const S: usize,
        const ID: usize,
    > Commands<H, { MESSAGE_LEN - 1 }, { MESSAGE_LEN - 2 }, { MESSAGE_LEN - 4 }, ID>
    for CommandInterface<KINTF, TX, RX, N, H, S, ID>
{
    fn default_packet_chunk(&self) -> u32 {
        N as u32
    }

    fn tx_packetbuffer_send(&mut self, buf: &mut HidIoPacketBuffer<H>) -> Result<(), CommandError> {
        let size = buf.serialized_len() as usize;
        trace!("tx_packetbuffer_send: {:?} serialized_len({:?})", buf, size);
        if self.serial_buf.resize_default(size).is_err() {
            return Err(CommandError::SerializationVecTooSmall);
        }
        match buf.serialize_buffer(&mut self.serial_buf) {
            Ok(data) => data,
            Err(err) => {
                return Err(CommandError::SerializationFailed(err));
            }
        };

        // Add serialized data to buffer
        // May need to enqueue multiple packets depending how much
        // was serialized
        let data = &self.serial_buf;
        for pos in (0..data.len()).step_by(N) {
            let len = core::cmp::min(N, data.len() - pos);
            match self
                .tx_bytebuf
                .enqueue(match Vec::from_slice(&data[pos..len + pos]) {
                    Ok(vec) => vec,
                    Err(_) => {
                        return Err(CommandError::TxBufferVecTooSmall);
                    }
                }) {
                Ok(_) => {}
                Err(_) => {
                    return Err(CommandError::TxBufferSendFailed);
                }
            }
        }
        Ok(())
    }
    fn supported_id(&self, id: HidIoCommandId) -> bool {
        self.ids.iter().any(|&i| i == id)
    }

    fn h0000_supported_ids_cmd(&mut self, _data: h0000::Cmd) -> Result<h0000::Ack<ID>, h0000::Nak> {
        // Build id list to send back
        Ok(h0000::Ack::<ID> {
            ids: self.ids.clone(),
        })
    }

    /// Uses the CommandInterface to send data directly
    fn h0001_info_cmd(
        &mut self,
        data: h0001::Cmd,
    ) -> Result<h0001::Ack<{ MESSAGE_LEN - 1 }>, h0001::Nak> {
        use h0001::*;

        let property = data.property;
        let os = OsType::Unknown;
        let mut number = 0;
        let mut string = String::new();

        trace!("h0001_info_cmd: {:?}", data);
        match property {
            Property::MajorVersion => {
                number = pkg_version_major!();
            }
            Property::MinorVersion => {
                number = pkg_version_minor!();
            }
            Property::PatchVersion => {
                number = pkg_version_patch!();
            }
            Property::DeviceName => {
                if let Some(conf) = self.interface.h0001_device_name() {
                    string.clear();
                    if string.push_str(conf).is_err() {
                        return Err(Nak { property });
                    }
                }
            }
            Property::DeviceSerialNumber => {
                if let Some(conf) = self.interface.h0001_device_serial_number() {
                    string.clear();
                    if string.push_str(conf).is_err() {
                        return Err(Nak { property });
                    }
                }
            }
            Property::DeviceVersion => {
                if let Some(conf) = self.interface.h0001_device_version() {
                    string.clear();
                    if string.push_str(conf).is_err() {
                        return Err(Nak { property });
                    }
                }
            }
            Property::DeviceMcu => {
                if let Some(conf) = self.interface.h0001_device_mcu() {
                    string.clear();
                    if string.push_str(conf).is_err() {
                        return Err(Nak { property });
                    }
                }
            }
            Property::FirmwareName => {
                if let Some(conf) = self.interface.h0001_firmware_name() {
                    string.clear();
                    if string.push_str(conf).is_err() {
                        return Err(Nak { property });
                    }
                }
            }
            Property::FirmwareVersion => {
                if let Some(conf) = self.interface.h0001_firmware_version() {
                    string.clear();
                    if string.push_str(conf).is_err() {
                        return Err(Nak { property });
                    }
                }
            }
            Property::DeviceVendor => {
                if let Some(conf) = self.interface.h0001_device_vendor() {
                    string.clear();
                    if string.push_str(conf).is_err() {
                        return Err(Nak { property });
                    }
                }
            }
            _ => {
                return Err(Nak { property });
            }
        }

        Ok(Ack {
            property,
            os,
            number,
            string,
        })
    }
    /// Uses the CommandInterface to store data rather than issue
    /// a callback
    fn h0001_info_ack(
        &mut self,
        data: h0001::Ack<{ MESSAGE_LEN - 1 }>,
    ) -> Result<(), CommandError> {
        use h0001::*;

        trace!("h0001_info_ack: {:?}", data);
        match data.property {
            Property::MajorVersion => {
                self.hostinfo.major_version = data.number;
            }
            Property::MinorVersion => {
                self.hostinfo.minor_version = data.number;
            }
            Property::PatchVersion => {
                self.hostinfo.patch_version = data.number;
            }
            Property::OsType => {
                self.hostinfo.os = data.os as u8;
            }
            Property::OsVersion => {
                self.hostinfo.os_version = String::from(data.string.as_str());
            }
            Property::HostSoftwareName => {
                self.hostinfo.host_software_name = String::from(data.string.as_str());
            }
            _ => {
                return Err(CommandError::InvalidProperty8(data.property as u8));
            }
        }

        Ok(())
    }

    fn h0002_test_cmd(&mut self, data: h0002::Cmd<H>) -> Result<h0002::Ack<H>, h0002::Nak> {
        Ok(h0002::Ack { data: data.data })
    }

    fn h0016_flashmode_cmd(&mut self, data: h0016::Cmd) -> Result<h0016::Ack, h0016::Nak> {
        self.interface.h0016_flashmode_cmd(data)
    }

    fn h001a_sleepmode_cmd(&mut self, data: h001a::Cmd) -> Result<h001a::Ack, h001a::Nak> {
        self.interface.h001a_sleepmode_cmd(data)
    }

    fn h0021_pixelsetting_cmd(&mut self, data: h0021::Cmd) -> Result<h0021::Ack, h0021::Nak> {
        self.interface.h0021_pixelsetting_cmd(data)
    }

    fn h0021_pixelsetting_nacmd(&mut self, data: h0021::Cmd) -> Result<(), CommandError> {
        if self.interface.h0021_pixelsetting_cmd(data).is_ok() {
            Ok(())
        } else {
            Err(CommandError::CallbackFailed)
        }
    }

    fn h0026_directset_cmd(
        &mut self,
        data: h0026::Cmd<{ MESSAGE_LEN - 2 }>,
    ) -> Result<h0026::Ack, h0026::Nak> {
        self.interface.h0026_directset_cmd(data)
    }

    fn h0026_directset_nacmd(
        &mut self,
        data: h0026::Cmd<{ MESSAGE_LEN - 2 }>,
    ) -> Result<(), CommandError> {
        if self.interface.h0026_directset_cmd(data).is_ok() {
            Ok(())
        } else {
            Err(CommandError::CallbackFailed)
        }
    }

    fn h0031_terminalcmd_cmd(&mut self, data: h0031::Cmd<H>) -> Result<h0031::Ack, h0031::Nak> {
        if self.interface.h0031_terminalinput(data) {
            Ok(h0031::Ack {})
        } else {
            Err(h0031::Nak {})
        }
    }
    fn h0031_terminalcmd_nacmd(&mut self, data: h0031::Cmd<H>) -> Result<(), CommandError> {
        if self.interface.h0031_terminalinput(data) {
            Ok(())
        } else {
            Err(CommandError::CallbackFailed)
        }
    }

    fn h0050_manufacturing_cmd(&mut self, data: h0050::Cmd) -> Result<h0050::Ack, h0050::Nak> {
        self.interface.h0050_manufacturing_cmd(data)
    }

    fn h0051_manufacturingres_ack(&mut self, _data: h0051::Ack) -> Result<(), CommandError> {
        Ok(())
    }

    fn h0061_layerset_cmd(&mut self, data: h0061::Cmd) -> Result<h0061::Ack, h0061::Nak> {
        self.interface.h0061_layerset_cmd(data)
    }
}

// ----- Traits -----

/// Kiibohd Command Interface
/// Simplified CommandInterface used to receive HID-IO callbacks.
pub trait KiibohdCommandInterface<const H: usize> {
    /// HID-IO Sync Packet received
    /// TODO: Is this necessary anymore, or can timeouts be handled here?
    /// Callback
    fn hidio_sync_packet(&self) {}

    /// Returns the device name (e.g. Keystone TKL)a
    /// Callback
    fn h0001_device_name(&self) -> Option<&str>;

    /// Returns the device serial number
    /// Callback
    fn h0001_device_serial_number(&self) -> Option<&str> {
        None
    }

    /// Returns the device version
    /// Callback
    fn h0001_device_version(&self) -> Option<&str> {
        None
    }

    /// Returns device MCU name
    /// Callback
    fn h0001_device_mcu(&self) -> Option<&str> {
        None
    }

    /// Returns name of firmware (e.g. kiibohd)
    /// Callback
    fn h0001_firmware_name(&self) -> Option<&str>;

    /// Returns version of firmware
    /// Callback
    fn h0001_firmware_version(&self) -> Option<&str> {
        None
    }

    /// Returns device vendor name (e.g. Input Club)
    /// Callback
    fn h0001_device_vendor(&self) -> Option<&str> {
        None
    }

    /// Schedule flash mode (jump to bootloader)
    /// Ideally, this function should return a response, push USB buffer, then initiate flash mode
    /// (so that the HID-IO host gets confirmation that we're going to enter flash mode)
    /// However, if that is not possible, it is ok to immediately enter flash mode.
    /// Callback
    fn h0016_flashmode_cmd(&mut self, _data: h0016::Cmd) -> Result<h0016::Ack, h0016::Nak> {
        Err(h0016::Nak {
            error: h0016::Error::NotSupported,
        })
    }

    /// Schedule sleep mode
    /// Ideally, this function should return a response, push USB buffer, then initiate sleep mode
    /// (so that the HID-IO host gets confirmation that we're going to enter sleep mode)
    /// However, if that is not possible, it is ok to immediately enter sleep mode.
    /// It is possible the device is not ready for sleep, send the appropriate error flag in this
    /// case.
    /// Callback
    fn h001a_sleepmode_cmd(&mut self, _data: h001a::Cmd) -> Result<h001a::Ack, h001a::Nak> {
        Err(h001a::Nak {
            error: h001a::Error::NotSupported,
        })
    }

    /// General Pixel/LED settings for HID-IO
    fn h0021_pixelsetting_cmd(&mut self, _data: h0021::Cmd) -> Result<h0021::Ack, h0021::Nak> {
        Err(h0021::Nak {})
    }

    /// Raw pixel/LED data setting
    /// This is device/configuration specific, but is also the most efficient way to set pixels/LEDs.
    fn h0026_directset_cmd(
        &mut self,
        _data: h0026::Cmd<{ MESSAGE_LEN - 2 }>,
    ) -> Result<h0026::Ack, h0026::Nak> {
        Err(h0026::Nak {})
    }

    /// Logging callback
    /// Input received from host
    /// Return false if not able to log (buffer full, or disabled)
    /// Callback
    fn h0031_terminalinput(&mut self, _data: h0031::Cmd<H>) -> bool {
        false
    }

    /// Manufacturing command
    /// Input manufacturing command coming from the host
    /// Callback
    fn h0050_manufacturing_cmd(&mut self, _data: h0050::Cmd) -> Result<h0050::Ack, h0050::Nak> {
        Err(h0050::Nak {})
    }

    /// Layer set callback
    /// Input received from host
    /// Callback
    fn h0061_layerset_cmd(&mut self, _data: h0061::Cmd) -> Result<h0061::Ack, h0061::Nak> {
        Err(h0061::Nak {})
    }
}
