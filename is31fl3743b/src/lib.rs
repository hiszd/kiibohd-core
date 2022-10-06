// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// See <https://www.lumissil.com/assets/pdf/core/IS31FL3743B_DS.pdf> for Chip details.

#![no_std]

use heapless::spsc::Queue;

const ISSI_CONFIG_PAGE: u8 = 0x52;
const ISSI_SCALE_PAGE: u8 = 0x51;
const ISSI_PWM_PAGE: u8 = 0x50;
/// Both the LED Scaling and PWM register pages have the same length
/// See Table 2 (pg 11): <https://www.lumissil.com/assets/pdf/core/IS31FL3743B_DS.pdf>
pub const ISSI_PAGE_LEN: usize = 0xC6;
/// Both the LED Scaling and PWM registers start from 0x01, not 0x00
/// See Table 2 (pg 11): <https://www.lumissil.com/assets/pdf/core/IS31FL3743B_DS.pdf>
const ISSI_PAGE_START: u8 = 0x01;
const ISSI_OPEN_REG_LEN: usize = 0x20;
const ISSI_OPEN_REG_START: u8 = 0x03;

#[derive(Clone, Copy, Debug, PartialEq, Eq, defmt::Format)]
pub enum IssiError {
    OpenDetectNotReady,
    PdcBufferTooSmall(usize, usize),
    FuncQueueEmpty,
    FuncQueueFull,
    ShortDetectNotReady,
    UnhandledFunction(Function),
}

pub struct IssiBuf<const CHIPS: usize> {
    pwm: [[u8; ISSI_PAGE_LEN as usize]; CHIPS],
    scaling: [[u8; ISSI_PAGE_LEN as usize]; CHIPS],
}

impl<const CHIPS: usize> IssiBuf<CHIPS> {
    pub fn new() -> Self {
        Self {
            pwm: [[0; ISSI_PAGE_LEN]; CHIPS],
            scaling: [[0; ISSI_PAGE_LEN]; CHIPS],
        }
    }
}

impl<const CHIPS: usize> Default for IssiBuf<CHIPS> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, defmt::Format)]
pub enum Function {
    /// Set Brightness
    Brightness,
    /// Detect channel open circuits (read)
    OpenCircuitDetectRead,
    /// Detect channel open circuits (setup)
    OpenCircuitDetectSetup,
    /// Adjust PWM to each channel
    Pwm,
    /// Re-initializes ISSI Chips
    Reset,
    /// Applies current scaling to each channel
    Scaling,
    /// Detect channel short circuits (read)
    ShortCircuitDetectRead,
    /// Detect channel short circuits (setup)
    ShortCircuitDetectSetup,
    /// Software Shutdown
    SoftwareShutdown,
    /// Unknown function
    Unknown,
}

const fn atsam4_cs_to_pcs(cs: u8) -> u8 {
    match cs {
        0 => 0b0000, // xxx0 => NPCS[3:0] = 1110
        1 => 0b0001, // xx01 => NPCS[3:0] = 1101
        2 => 0b0011, // x011 => NPCS[3:0] = 1011
        3 => 0b0111, // 0111 => NPCS[3:0] = 0111
        _ => 0b1111, // Forbidden
    }
}

const fn atsam4_pcs_to_cs(pcs: u8) -> u8 {
    if pcs & 0b0001 == 0 {
        0
    } else if pcs & 0b0010 == 0 {
        1
    } else if pcs & 0b0100 == 0 {
        2
    } else if pcs & 0b1000 == 0 {
        3
    } else {
        0xFF // Forbidden
    }
}

/// Builds a 32-bit PDC-Ready Variable Peripheral Selection SPI word for ATSAM4S
/// Used to generate constant buffers easily
const fn atsam4_var_spi(data: u8, cs: u8, lastxfer: bool) -> u32 {
    (data as u32) | ((atsam4_cs_to_pcs(cs) as u32) << 16) | ((lastxfer as u32) << 24)
}

/// Converts a 32-bit PDC-Ready Variable Peripheral Selection SPI word to a tuple of data, cs, and lastxfer
/// (data, cs, lastxfer)
const fn atsam4_var_data(spi: u32) -> (u8, u8, bool) {
    let data = spi & 0xFF;
    let cs = atsam4_pcs_to_cs(((spi >> 16) & 0xF) as u8);
    let lastxfer = (spi >> 24) & 1 == 1;
    (data as u8, cs, lastxfer)
}

/// Copies the sync expression to the buffer and returns the new position
/// atsam4_reg_sync!(tx_buf, position, [cs0, cs1,...], page, register, value)
macro_rules! atsam4_reg_sync {
    (
        $Buf:ident, $Pos:expr, $Chips:expr, $Page:expr, $Reg:expr, $Value:expr
    ) => {{
        let mut pos = $Pos;
        for cs in $Chips {
            let buf = [
                atsam4_var_spi($Page, *cs, false),
                atsam4_var_spi($Reg, *cs, false),
                atsam4_var_spi($Value, *cs, true),
            ];
            $Buf[pos..pos + 3].copy_from_slice(&buf);
            pos += 3;
        }

        pos
    }};
}

/// atsam4 specific implementation of Is31fl3743b (variable cs mode)
///
/// ```ignore
/// use is31fl3743b::Is31fl3743bAtsam4Dma;
///
/// const ISSI_DRIVER_CHIPS: usize = 2;
/// const ISSI_DRIVER_QUEUE_SIZE: usize = 5;
/// const ISSI_DRIVER_CS_LAYOUT: [u8; ISSI_DRIVER_CHIPS] = [0, 1];
/// // Must be 256 or less, or a power of 2; e.g. 512 due limitations with embedded-dma
/// // Actual value should be -> ISSI_DRIVER_CHIPS * 198 (e.g. 396);
/// // Size is determined by the largest SPI tx transaction
/// const SPI_TX_BUF_SIZE: usize = 512;
/// // Size is determined by the largest SPI rx transaction
/// const SPI_RX_BUF_SIZE: usize = (32 + 2) * ISSI_DRIVER_CHIPS;
///
/// #[init(local = [spi_tx_buf: [u32; SPI_TX_BUF_SIZE] = [0; SPI_TX_BUF_SIZE], spi_rx_buf: [u32; SPI_RX_BUF_SIZE] = [0; SPI_RX_BUF_SIZE],])]
/// fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
///     // ... Continued ...
///
///     let wdrbt = false; // Wait data read before transfer enabled
///     let llb = false; // Local loopback
///                  // Cycles to delay between consecutive transfers
///     let dlybct = 0; // No delay
///     let mut spi = SpiMaster::<u32>::new(
///         cx.device.SPI,
///         clocks.peripheral_clocks.spi.into_enabled_clock(),
///         pins.spi_miso,
///         pins.spi_mosi,
///         pins.spi_sck,
///         atsam4_hal::spi::PeripheralSelectMode::Variable,
///         wdrbt,
///         llb,
///         dlybct,
///     );
///
///     // Setup each CS channel
///     let mode = atsam4_hal::spi::spi::MODE_3;
///     let csa = atsam4_hal::spi::ChipSelectActive::ActiveAfterTransfer;
///     let bits = atsam4_hal::spi::BitWidth::Width8Bit;
///     let baud = atsam4_hal::spi::Hertz(12_000_000_u32);
///     // Cycles to delay from CS to first valid SPCK
///     let dlybs = 0; // Half an SPCK clock period
///     let cs_settings =
///     atsam4_hal::spi::ChipSelectSettings::new(mode, csa, bits, baud, dlybs, dlybct);
///     for i in 0..ISSI_DRIVER_CHIPS {
///         spi.cs_setup(i, cs_settings.clone()).unwrap();
///     }
///     spi.enable_txbufe_interrupt();
///
///     // Setup SPI with pdc
///     let spi = spi.with_pdc_rxtx();
///
///     // Setup ISSI LED Driver
///     let issi_default_brightness = 255; // TODO compile-time configuration + flash default
///     let issi_default_enable = true; // TODO compile-time configuration + flash default
///     let mut issi = Is31fl3743bAtsam4Dma::<ISSI_DRIVER_CHIPS, ISSI_DRIVER_QUEUE_SIZE>::new(
///         ISSI_DRIVER_CS_LAYOUT,
///         issi_default_brightness,
///         issi_default_enable,
///     );
///
///     for chip in issi.pwm_page_buf() {
///         chip.iter_mut().for_each(|e| *e = 255);
///     }
///     for chip in issi.scaling_page_buf() {
///         chip.iter_mut().for_each(|e| *e = 100);
///     }
///
///     // Start ISSI LED Driver initialization
///     issi.reset().unwrap(); // Queue reset DMA transaction
///     issi.scaling().unwrap(); // Queue scaling default
///     issi.pwm().unwrap(); // Queue pwm default
///     let (rx_len, tx_len) = issi.tx_function(cx.local.spi_tx_buf).unwrap();
///     let spi_rxtx = spi.read_write_len(cx.local.spi_rx_buf, rx_len, cx.local.spi_tx_buf, tx_len);
///
///     // ... Continued ...
/// }
///
/// // SPI Interrupt
/// #[task(binds = SPI, priority = 12, shared = [issi, spi, spi_rxtx])]
/// fn spi(mut cx: spi::Context) {
///     let mut issi = cx.shared.issi;
///     let mut spi_rxtx = cx.shared.spi_rxtx;
///
///     spi_rxtx.lock(|spi_rxtx| {
///         // Retrieve DMA buffer
///         if let Some(spi_buf) = spi_rxtx.take() {
///             let ((rx_buf, tx_buf), spi) = spi_buf.wait();
///
///             issi.lock(|issi| {
///                 // Process Rx buffer if applicable
///                 issi.rx_function(rx_buf).unwrap();
///
///                 // Prepare the next DMA transaction
///                 if let Ok((rx_len, tx_len)) = issi.tx_function(tx_buf) {
///                     spi_rxtx.replace(spi.read_write_len(rx_buf, rx_len, tx_buf, tx_len));
///                 } else {
///                     // Disable PDC
///                     let mut spi = spi.revert();
///                     spi.disable_txbufe_interrupt();
///
///                     // No more transactions ready, park spi peripheral and buffers
///                     cx.shared.spi.lock(|spi_periph| {
///                         spi_periph.replace((spi, rx_buf, tx_buf));
///                     });
///                 }
///             });
///         }
///     });
/// }
/// ```
pub struct Is31fl3743bAtsam4Dma<const CHIPS: usize, const QUEUE_SIZE: usize> {
    /// Default LED brightness, used during initialization
    initial_global_brightness: u8,
    /// Currently set global LED brightness, used to handle increments
    current_global_brightness: u8,
    /// Chip enable flag (used to power down the chips; often used for powersaving)
    enable: bool,
    /// List of chip selects
    cs: [u8; CHIPS],
    /// Queue producer for PDC functions
    func_queue: Queue<Function, QUEUE_SIZE>,
    /// Buffer used to copy the incoming buffer data to send to the ISSI chips
    /// Contains data for both the PWM and Scaling pages
    page_buf: IssiBuf<CHIPS>,
    /// Short detect buffer is ready
    short_detect_ready: bool,
    /// Short detect buffer
    short_detect: [[u8; ISSI_OPEN_REG_LEN]; CHIPS],
    /// Open detect buffer is ready
    open_detect_ready: bool,
    /// Open detect buffer
    open_detect: [[u8; ISSI_OPEN_REG_LEN]; CHIPS],
    /// Holds most recent rx_len
    last_rx_len: usize,
}

impl<const CHIPS: usize, const QUEUE_SIZE: usize> Is31fl3743bAtsam4Dma<CHIPS, QUEUE_SIZE>
{
    pub fn new(cs: [u8; CHIPS], initial_global_brightness: u8, enable: bool) -> Self {
        Self {
            initial_global_brightness,
            current_global_brightness: initial_global_brightness,
            enable,
            cs,
            func_queue: Queue::new(),
            page_buf: IssiBuf::new(),
            short_detect_ready: false,
            short_detect: [[0; ISSI_OPEN_REG_LEN]; CHIPS],
            open_detect_ready: false,
            open_detect: [[0; ISSI_OPEN_REG_LEN]; CHIPS],
            last_rx_len: 0,
        }
    }

    /// Access pwm page buffer
    pub fn pwm_page_buf(&mut self) -> &mut [[u8; ISSI_PAGE_LEN]; CHIPS] {
        &mut self.page_buf.pwm
    }

    /// Access scaling page buffer
    pub fn scaling_page_buf(&mut self) -> &mut [[u8; ISSI_PAGE_LEN]; CHIPS] {
        &mut self.page_buf.scaling
    }

    /// Called to process DMA data buffer (after interrupt)
    pub fn rx_function(&mut self, rx_buf: &[u32]) -> Result<(), IssiError>
    {
        // Dequeue function as we're finished with it
        let func = if let Some(func) = self.func_queue.dequeue() {
            func
        } else {
            return Err(IssiError::FuncQueueEmpty);
        };
        defmt::trace!("rx_function: {:?} - {:X}", func, rx_buf);

        match func {
            Function::Brightness => self.brightness_set_rx(rx_buf),
            Function::OpenCircuitDetectRead => self.open_circuit_detect_read_rx(rx_buf),
            Function::OpenCircuitDetectSetup => self.open_circuit_detect_setup_rx(rx_buf),
            Function::Pwm => self.pwm_rx(rx_buf),
            Function::Reset => self.reset_rx(rx_buf),
            Function::Scaling => self.scaling_rx(rx_buf),
            Function::ShortCircuitDetectRead => self.short_circuit_detect_read_rx(rx_buf),
            Function::ShortCircuitDetectSetup => self.short_circuit_detect_setup_rx(rx_buf),
            Function::SoftwareShutdown => self.software_shutdown_rx(rx_buf),
            _ => Err(IssiError::UnhandledFunction(func)),
        }
    }

    /// Called to prepare Tx buffer before initiating DMA
    /// (rx_len, tx_len)
    pub fn tx_function(&mut self, tx_buf: &mut [u32]) -> Result<(usize, usize), IssiError> {
        // Don't dequeue as we'll need to refer back after the DMA transaction is finished
        let func = if let Some(func) = self.func_queue.peek() {
            func
        } else {
            return Err(IssiError::FuncQueueEmpty);
        };
        defmt::trace!("tx_function: {:?}", func);

        match func {
            Function::Brightness => self.brightness_set_tx(tx_buf),
            Function::OpenCircuitDetectRead => self.openshort_circuit_detect_read_tx(tx_buf),
            Function::OpenCircuitDetectSetup => self.open_circuit_detect_setup_tx(tx_buf),
            Function::Pwm => self.pwm_tx(tx_buf),
            Function::Reset => self.reset_tx(tx_buf),
            Function::Scaling => self.scaling_tx(tx_buf),
            Function::ShortCircuitDetectRead => self.openshort_circuit_detect_read_tx(tx_buf),
            Function::ShortCircuitDetectSetup => self.short_circuit_detect_setup_tx(tx_buf),
            Function::SoftwareShutdown => self.software_shutdown_tx(tx_buf),
            _ => Err(IssiError::UnhandledFunction(*func)),
        }
    }

    /// Triggers chip reset sequence
    pub fn reset(&mut self) -> Result<(), IssiError> {
        if self.func_queue.enqueue(Function::Reset).is_ok() {
            defmt::trace!("reset");
            Ok(())
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    fn reset_rx(&mut self, _rx_buf: &[u32]) -> Result<(), IssiError> {
        Ok(())
    }

    fn reset_tx(&mut self, tx_buf: &mut [u32]) -> Result<(usize, usize), IssiError> {
        let chips = &self.cs;
        let (last, chips_except_last) = self.cs.split_last().unwrap();
        let last = [last];
        let pos = 0;

        // Clear LED pages
        // Call reset to clear all register (on all chips)
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x2F, 0xAE);

        // Reset the global brightness
        self.current_global_brightness = self.initial_global_brightness;
        let pos = atsam4_reg_sync!(
            tx_buf,
            pos,
            chips,
            ISSI_CONFIG_PAGE,
            0x01,
            self.current_global_brightness
        );

        // Enable pull-up and pull-down anti-ghosting registers
        // TODO: Make configurable
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x02, 0x33);

        // Set temperature roll-off
        // TODO: Make configurable
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x24, 0x00);

        // Follower/slave sync
        // TODO: Make spread specture configurable
        let pos = if chips.len() > 1 {
            atsam4_reg_sync!(tx_buf, pos, chips_except_last, ISSI_CONFIG_PAGE, 0x25, 0x80)
        } else {
            pos
        };

        // Setup ISSI sync and spread spectrum function
        // XXX (HaaTa); The last chip is used as it is the last chip all of the frame data is sent to
        // This is imporant as it may take more time to send the packet than the ISSI chip can handle
        // between frames.
        // TODO: Make spread specture configurable
        let pos = atsam4_reg_sync!(tx_buf, pos, last, ISSI_CONFIG_PAGE, 0x25, 0xC0);

        // Disable software shutdown (if LEDs are enabled)
        let pos = if self.enable {
            atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x00, 0x09)
        } else {
            pos
        };

        defmt::info!("Reset Buf: {:?}", tx_buf);

        // Size of buffer PDC/DMA has sets to send
        self.last_rx_len = 0;
        Ok((0, pos))
    }

    pub fn scaling(&mut self) -> Result<(), IssiError> {
        if self.func_queue.enqueue(Function::Scaling).is_ok() {
            defmt::trace!("scaling");
            Ok(())
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    fn scaling_rx(&mut self, _rx_buf: &[u32]) -> Result<(), IssiError> {
        Ok(())
    }

    fn scaling_tx(&mut self, tx_buf: &mut [u32]) -> Result<(usize, usize), IssiError> {
        // Copy each byte from the shared buffer into the DMA/PDC buffer
        // The DMA format encodes the CS and last byte in a transaction
        let mut pos = 0;
        for (chip, chip_buf) in self.page_buf.scaling.into_iter().enumerate() {
            let cs = self.cs[chip];

            // Setup scaling page
            tx_buf[pos] = atsam4_var_spi(ISSI_SCALE_PAGE, cs, false);
            pos += 1;

            // First register (always 0x01)
            tx_buf[pos] = atsam4_var_spi(ISSI_PAGE_START, cs, false);
            pos += 1;

            // Handle most of the bytes
            let mut bytes = chip_buf.chunks_exact(chip_buf.len() - 1);
            for byte in bytes.next().unwrap() {
                tx_buf[pos] = atsam4_var_spi(*byte, cs, false);
                pos += 1;
            }
            // Final byte (lastxfer)
            tx_buf[pos] = atsam4_var_spi(bytes.remainder()[0], cs, true);
            pos += 1;
        }

        // Returns the total size of the DMA buffer to transfer
        self.last_rx_len = 0;
        Ok((0, pos))
    }

    pub fn pwm(&mut self) -> Result<(), IssiError> {
        if self.func_queue.enqueue(Function::Pwm).is_ok() {
            defmt::trace!("pwm");
            Ok(())
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    fn pwm_rx(&mut self, _rx_buf: &[u32]) -> Result<(), IssiError> {
        Ok(())
    }

    fn pwm_tx(&mut self, tx_buf: &mut [u32]) -> Result<(usize, usize), IssiError> {
        // Copy each byte from the shared buffer into the DMA/PDC buffer
        // The DMA format encodes the CS and last byte in a transaction
        let mut pos = 0;
        for (chip, chip_buf) in self.page_buf.pwm.into_iter().enumerate() {
            let cs = self.cs[chip];

            // Setup pwm page
            tx_buf[pos] = atsam4_var_spi(ISSI_PWM_PAGE, cs, false);
            pos += 1;

            // First register (always 0x01)
            tx_buf[pos] = atsam4_var_spi(ISSI_PAGE_START, cs, false);
            pos += 1;

            // Handle most of the bytes
            let mut bytes = chip_buf.chunks_exact(chip_buf.len() - 1);
            for byte in bytes.next().unwrap() {
                tx_buf[pos] = atsam4_var_spi(*byte, cs, false);
                pos += 1;
            }
            // Final byte (lastxfer)
            tx_buf[pos] = atsam4_var_spi(bytes.remainder()[0], cs, true);
            pos += 1;
        }

        // Returns the total size of the DMA buffer to transfer
        self.last_rx_len = 0;
        Ok((0, pos))
    }

    /// Enable LEDs on next process loop
    /// (Software Shutdown)
    pub fn enable(&mut self) -> Result<(), IssiError> {
        self.enable = true;
        self.software_shutdown()
    }

    /// Disable LEDs on next process loop
    /// (Software Shutdown)
    pub fn disable(&mut self) -> Result<(), IssiError> {
        self.enable = false;
        self.software_shutdown()
    }

    /// Toggle LEDs on next process loop
    /// (Software Shutdown)
    pub fn toggle(&mut self) -> Result<bool, IssiError> {
        self.enable = !self.enable;
        self.software_shutdown()?;
        Ok(self.enable)
    }

    /// LED status
    pub fn enabled(&self) -> bool {
        self.enable
    }

    fn software_shutdown(&mut self) -> Result<(), IssiError> {
        if self.func_queue.enqueue(Function::SoftwareShutdown).is_ok() {
            defmt::trace!("software_shutdown: {:?}", self.enable);
            Ok(())
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    fn software_shutdown_rx(&mut self, _rx_buf: &[u32]) -> Result<(), IssiError> {
        Ok(())
    }

    fn software_shutdown_tx(&mut self, tx_buf: &mut [u32]) -> Result<(usize, usize), IssiError> {
        let pos = if self.enable {
            // Disable software shutdown
            atsam4_reg_sync!(tx_buf, 0, &self.cs, ISSI_CONFIG_PAGE, 0x00, 0x09)
        } else {
            // Enable software shutdown
            atsam4_reg_sync!(tx_buf, 0, &self.cs, ISSI_CONFIG_PAGE, 0x00, 0x08)
        };
        self.last_rx_len = 0;
        Ok((0, pos))
    }

    /// Increase brightness
    /// Minimum value: 0x00
    /// Maximum value: 0xFF
    pub fn brightness_increase(&mut self, inc: u8) -> Result<u8, IssiError> {
        let val = if inc as u16 + self.current_global_brightness as u16 > 0xFF {
            0xFF
        } else {
            self.current_global_brightness + inc
        };
        self.brightness_set(val)?;
        Ok(val)
    }

    /// Decrease brightness
    /// Minimum value: 0x00
    /// Maximum value: 0xFF
    pub fn brightness_decrease(&mut self, dec: u8) -> Result<u8, IssiError> {
        let val = if self.current_global_brightness as i16 - (dec as i16) < 0 {
            0x0
        } else {
            self.current_global_brightness - dec
        };
        self.brightness_set(val)?;
        Ok(val)
    }

    /// Set brightness
    pub fn brightness_set(&mut self, val: u8) -> Result<u8, IssiError> {
        self.current_global_brightness = val;
        if self.func_queue.enqueue(Function::Brightness).is_ok() {
            defmt::trace!("brightness_set: {:?}", self.current_global_brightness);
            Ok(val)
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    fn brightness_set_rx(&mut self, _rx_buf: &[u32]) -> Result<(), IssiError> {
        Ok(())
    }

    fn brightness_set_tx(&mut self, tx_buf: &mut [u32]) -> Result<(usize, usize), IssiError> {
        let pos = atsam4_reg_sync!(
            tx_buf,
            0,
            &self.cs,
            ISSI_CONFIG_PAGE,
            0x01,
            self.current_global_brightness
        );
        self.last_rx_len = 0;
        Ok((0, pos))
    }

    /// Reset brightness to default value
    pub fn brightness_reset(&mut self) -> Result<u8, IssiError> {
        self.brightness_set(self.initial_global_brightness)?;
        Ok(self.initial_global_brightness)
    }

    /// Current brightness
    pub fn brightness(&self) -> u8 {
        self.current_global_brightness
    }

    /// Open Circuit Detect Setup
    /// Configures registers and does an open circuit detect
    /// Must wait at least 750 us after the registers are set before reading
    pub fn open_circuit_detect_setup(&mut self) -> Result<(), IssiError> {
        if self
            .func_queue
            .enqueue(Function::OpenCircuitDetectSetup)
            .is_ok()
        {
            defmt::trace!("open_circuit_detect_setup");
            Ok(())
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    /// Open Circuit Detect Read
    /// Reads the open circuit detect registers into memory.
    ///
    /// Use open_circuit_lookup() or open_circuit_raw() to read the values.
    /// Must wait at least 750 us after configuring the registers before reading
    pub fn open_circuit_detect_read(&mut self) -> Result<(), IssiError> {
        // Clear open data
        self.open_detect_ready = false;
        if self
            .func_queue
            .enqueue(Function::OpenCircuitDetectRead)
            .is_ok()
            || self.func_queue.enqueue(Function::Reset).is_err()
        {
            defmt::trace!("open_circuit_detect_read");
            Ok(())
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    fn open_circuit_detect_setup_rx(&mut self, _rx_buf: &[u32]) -> Result<(), IssiError> {
        defmt::trace!("open_circuit_detect_setup_rx");
        // Nothing to do
        Ok(())
    }

    fn open_circuit_detect_setup_tx(
        &mut self,
        tx_buf: &mut [u32],
    ) -> Result<(usize, usize), IssiError> {
        // Set scaling and pwm to max
        self.scale_pwm_max()?;

        let chips = &self.cs;
        let pos = 0;

        // Set Global Current Control (needed for accurate readings)
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x01, 0x0F);

        // Disable pull resistors
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x02, 0x00);

        // Set OSD to open detection
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x00, 0x0B);

        defmt::trace!("open_circuit_detect_setup_tx: {:?} - {:X}", pos, tx_buf);
        self.last_rx_len = 0;
        Ok((0, pos))
    }

    fn open_circuit_detect_read_rx(&mut self, rx_buf: &[u32]) -> Result<(), IssiError> {
        for (pos, word) in rx_buf[..self.last_rx_len].iter().enumerate() {
            let chip = pos / (ISSI_OPEN_REG_LEN + 2);
            let pos = pos - chip * (ISSI_OPEN_REG_LEN + 2);
            let (data, _, _) = atsam4_var_data(*word);
            // First three bytes for each chip are ignored (spi read setup)
            if pos >= 2 {
               self.open_detect[chip][pos - 2] = data;
            }
        }
        defmt::trace!("Open detect: {:X}", self.open_detect);
        self.open_detect_ready = true;
        Ok(())
    }

    fn openshort_circuit_detect_read_tx(
        &mut self,
        tx_buf: &mut [u32],
    ) -> Result<(usize, usize), IssiError> {
        // The DMA format encodes the CS and last byte in a transaction
        let mut pos = 0;
        for cs in self.cs {
            // Setup config page in read mode
            tx_buf[pos] = atsam4_var_spi(ISSI_CONFIG_PAGE | 0x80, cs, false);
            pos += 1;

            // Setup first register to read
            tx_buf[pos] = atsam4_var_spi(ISSI_OPEN_REG_START, cs, false);
            pos += 1;

            // Send 0x00, chip will auto-increment which register to read from next
            for i in 0..ISSI_OPEN_REG_LEN - 2 {
                //let reg = ISSI_OPEN_REG_START + i as u8;
                tx_buf[pos + i] = atsam4_var_spi(0x00, cs, false);
            }
            pos += ISSI_OPEN_REG_LEN - 2;

            // Set lastxfer on final read byte
            tx_buf[pos] = atsam4_var_spi(0x00, cs, true);
            pos += 1;
        }

        // Set total length to read + register setup
        self.last_rx_len = pos;
        Ok((pos, pos))
    }

    /// Short Circuit Detect Setup
    /// Configures registers and does a short circuit detect
    /// Must wait at least 750 us after the registers are set before reading
    pub fn short_circuit_detect_setup(&mut self) -> Result<(), IssiError> {
        if self
            .func_queue
            .enqueue(Function::ShortCircuitDetectSetup)
            .is_ok()
        {
            defmt::trace!("short_circuit_detect_setup");
            Ok(())
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    /// Short Circuit Detect Read
    /// Reads the short circuit detect registers into memory.
    ///
    /// Use short_circuit_lookup() or short_circuit_raw() to read the values.
    /// Must wait at least 750 us after configuring the registers before reading
    pub fn short_circuit_detect_read(&mut self) -> Result<(), IssiError> {
        // Clear short data
        self.short_detect_ready = false;
        if self
            .func_queue
            .enqueue(Function::ShortCircuitDetectRead)
            .is_ok()
            || self.func_queue.enqueue(Function::Reset).is_err()
        {
            defmt::trace!("short_circuit_detect_read");
            Ok(())
        } else {
            Err(IssiError::FuncQueueFull)
        }
    }

    fn short_circuit_detect_setup_rx(&mut self, _rx_buf: &[u32]) -> Result<(), IssiError> {
        defmt::trace!("short_circuit_detect_setup_rx");
        // Nothing to do
        Ok(())
    }

    fn short_circuit_detect_setup_tx(
        &mut self,
        tx_buf: &mut [u32],
    ) -> Result<(usize, usize), IssiError> {
        // Set scaling and pwm to max
        self.scale_pwm_max()?;

        let chips = &self.cs;
        let pos = 0;

        // Set Global Current Control (needed for accurate readings)
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x01, 0x0F);

        // Set pull down resistors
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x02, 0x00);

        // Set OSD to short detection
        let pos = atsam4_reg_sync!(tx_buf, pos, chips, ISSI_CONFIG_PAGE, 0x00, 0x0D);

        self.last_rx_len = 0;
        Ok((0, pos))
    }

    fn short_circuit_detect_read_rx(&mut self, rx_buf: &[u32]) -> Result<(), IssiError> {
        for (pos, word) in rx_buf[..self.last_rx_len].iter().enumerate() {
            let chip = pos / (ISSI_OPEN_REG_LEN + 2);
            let pos = pos - chip * (ISSI_OPEN_REG_LEN + 2);
            let (data, _, _) = atsam4_var_data(*word);
            // First three bytes for each chip are ignored (spi read setup)
            if pos >= 2 {
                self.short_detect[chip][pos - 2] = data;
            }
        }
        defmt::trace!("Short detect: {:X}", self.short_detect);
        self.short_detect_ready = true;
        Ok(())
    }

    /// Can be used to find open circuit channel positions after calling open_detect()
    pub fn open_circuit_lookup(&self, chip: usize, ch: usize) -> Result<bool, IssiError> {
        if self.open_detect_ready {
            Ok((self.open_detect[chip][ch / 8] >> (ch % 8)) & 0x01 == 0x01)
        } else {
            Err(IssiError::OpenDetectNotReady)
        }
    }

    /// Raw data lookup for open circuit detections
    pub fn open_circuit_raw(&self) -> Result<[[u8; ISSI_OPEN_REG_LEN]; CHIPS], IssiError> {
        if self.open_detect_ready {
            Ok(self.open_detect)
        } else {
            Err(IssiError::OpenDetectNotReady)
        }
    }

    /// Can be used to find short circuit channel positions after calling short_detect()
    pub fn short_circuit_lookup(&self, chip: usize, ch: usize) -> Result<bool, IssiError> {
        if self.short_detect_ready {
            Ok((self.short_detect[chip][ch / 8] >> (ch % 8)) & 0x01 == 0x01)
        } else {
            Err(IssiError::ShortDetectNotReady)
        }
    }

    /// Raw data lookup for short circuit detections
    pub fn short_circuit_raw(&self) -> Result<[[u8; ISSI_OPEN_REG_LEN]; CHIPS], IssiError> {
        if self.short_detect_ready {
            Ok(self.short_detect)
        } else {
            Err(IssiError::ShortDetectNotReady)
        }
    }

    /// Set scaling and pwm registers to 0xFF for all channels
    /// This is used by the open and short detection features
    fn scale_pwm_max(&mut self) -> Result<(), IssiError> {
        for chip in 0..CHIPS {
            for ch in ISSI_PAGE_START as usize..ISSI_PAGE_LEN as usize{
                self.page_buf.scaling[chip][ch] = 0xFF;
                self.page_buf.pwm[chip][ch] = 0xFF;
            }
        }
        // Enqueue scaling and pwm updates
        self.pwm()?;
        self.scaling()?;

        Ok(())
    }
}
