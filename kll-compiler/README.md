# kll-compiler

[![Rust](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml/badge.svg)](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/kll-compiler/badge.svg)](https://docs.rs/kll-compiler)
[![Crates.io](https://img.shields.io/crates/v/kll-compiler.svg)](https://crates.io/crates/kll-compiler)
[![Crates.io](https://img.shields.io/crates/l/kll-compiler.svg)](https://crates.io/crates/kll-compiler)
[![Crates.io](https://img.shields.io/crates/d/kll-compiler.svg)](https://crates.io/crates/kll-compiler)

Rust implementation of the KLL compiler.
Designed to be integrated into build.rs as a library or as a stand-alone utility.


## Usage

See [kiibohd-firmware](https://github.com/kiibohd/kiibohd-firmware/blob/main/common/build.rs) for the primary use-case.


## Testing

```bash
cargo test

# To see verbose test output when debugging
RUST_LOG=trace cargo test emitters::kllcore::test::layer_lookup_simple -- --nocapture
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
