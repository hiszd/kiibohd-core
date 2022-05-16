# kiibohd-core

[![Rust](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml/badge.svg)](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml)

kiibohd-core is a re-implementation of the kiibohd firmware as a collection of rust modules.
It has been re-thought from the ground up as a set of input device crate modules which should make it easy to integrate individual pieces into other projects.

It is even possible to integrate pieces into other languages using the -ffi crates (generally look at [kiibohd-core-ffi](kiibohd-core-ffi)).

**NOTE**: Do not import the -ffi crates if working with rust. They are intended for use with C firmwares.

**NOTE**: Crates in this repo generally use nightly due to requirements of hid-io-protocol

**NOTE**: kll-compiler crate is special and is excluded from the main cargo due to no_std conflicts.


## Testing

Can be used as a quick sanity for all the modules.

```bash
cargo build --all
cargo clippy --all
cargo fmt --all
cargo test --all
cargo doc --all
```
