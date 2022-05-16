# kiibohd-usb

[![Rust](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml/badge.svg)](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/kiibohd-usb/badge.svg)](https://docs.rs/kiibohd-usb)
[![Crates.io](https://img.shields.io/crates/v/kiibohd-usb.svg)](https://crates.io/crates/kiibohd-usb)
[![Crates.io](https://img.shields.io/crates/l/kiibohd-usb.svg)](https://crates.io/crates/kiibohd-usb)
[![Crates.io](https://img.shields.io/crates/d/kiibohd-usb.svg)](https://crates.io/crates/kiibohd-usb)

Combination USB HID interface from the kiibohd project.
Instanciates the following USB HID interfaces:
* Boot mode keyboard (supports auto-switching through SET_PROTOCOL and manual switching)
* NKRO mode keyboard
* Consumer Ctrl and System Ctrl
* Mouse
* [HID-IO](https://github.com/hid-io/hid-io-core)

## Usage

```rust
let (mut kbd_producer, mut kbd_consumer) = KBD_QUEUE.split();
let (mut mouse_producer, mut mouse_consumer) = MOUSE_QUEUE.split();
let (mut ctrl_producer, mut ctrl_consumer) = CTRL_QUEUE.split();
let (mut hidio_rx_producer, mut hidio_rx_consumer) = HIDIO_RX_QUEUE.split();
let (mut hidio_tx_producer, mut hidio_tx_consumer) = HIDIO_TX_QUEUE.split();
let usb_hid = HidInterface::new(
		usb_bus, /* UsbBusAllocator */
		HidCountryCode::NotSupported,
		kbd_consumer,
		mouse_consumer,
		ctrl_consumer,
		hidio_rx_producer,
		hidio_tx_consumer,
);

usb_hid.poll(); // Poll HID-IO
usb_hid.push(); // Push hid reports and poll HID-IO
```

See [docs.rs](https://docs.rs/kiibohd-usb/) for more details.


## WIP

- Mouse interface not enabled yet (still some issues during allocation on atsam4s)

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
