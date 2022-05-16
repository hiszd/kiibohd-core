# kll-core

[![Rust](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml/badge.svg)](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/kll-core/badge.svg)](https://docs.rs/kll-core)
[![Crates.io](https://img.shields.io/crates/v/kll-core.svg)](https://crates.io/crates/kll-core)
[![Crates.io](https://img.shields.io/crates/l/kll-core.svg)](https://crates.io/crates/kll-core)
[![Crates.io](https://img.shields.io/crates/d/kll-core.svg)](https://crates.io/crates/kll-core)

kll-core is the KLL (Kiibohd Layout Language) funcitonal state machine implementation.
It is designed to be paired with the [kll-compiler](../kll-compiler) crate to process the generate state machine.

The main use-case for kll-core is embedded environments (no_std); however, it does work in standard environments.
kll-core uses externally defined datastructures to build the state-machine so the functionality can be manipulated without having to recompile kll-core.
This is especially important for embedded devices so firmware can be updated without having to change state configuration.

## Usage

See [kiibohd-firmware](https://github.com/kiibohd/kiibohd-firmware/) for the primary use-case.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
