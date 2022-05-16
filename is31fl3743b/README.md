# is31fl3743b

[![Rust](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml/badge.svg)](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/is31fl3743b/badge.svg)](https://docs.rs/is31fl3743b)
[![Crates.io](https://img.shields.io/crates/v/is31fl3743b.svg)](https://crates.io/crates/is31fl3743b)
[![Crates.io](https://img.shields.io/crates/l/is31fl3743b.svg)](https://crates.io/crates/is31fl3743b)
[![Crates.io](https://img.shields.io/crates/d/is31fl3743b.svg)](https://crates.io/crates/is31fl3743b)

Embedded driver crate for the Lumissil/ISSI [is31fl3743b](https://www.lumissil.com/assets/pdf/core/IS31FL3743B_DS.pdf) SPI LED driver.

Currently only supports atsam4 MCUs due to reliance on specialized DMA buffers.
Porting to other MCUs is possible but likely won't be as efficient.

## Example Usage

See [docs.rs](https://docs.rs/is31fl3743b/0.1.0/is31fl3743b) for example usage.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
