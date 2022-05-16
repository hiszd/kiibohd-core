# kiibohd-hall-effect-keyscanning

[![Rust](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml/badge.svg)](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/kiibohd-hall-effect-keyscanning/badge.svg)](https://docs.rs/kiibohd-hall-effect-keyscanning)
[![Crates.io](https://img.shields.io/crates/v/kiibohd-hall-effect-keyscanning.svg)](https://crates.io/crates/kiibohd-hall-effect-keyscanning)
[![Crates.io](https://img.shields.io/crates/l/kiibohd-hall-effect-keyscanning.svg)](https://crates.io/crates/kiibohd-hall-effect-keyscanning)
[![Crates.io](https://img.shields.io/crates/d/kiibohd-hall-effect-keyscanning.svg)](https://crates.io/crates/kiibohd-hall-effect-keyscanning)

Keyscanning embedded-hal driver for [kiibohd-hall-effect](../kiibohd-hall-effect).
Can be used with single-shot, interrupt or DMA-connected ADC drivers.

## Usage

```rust
const ADC_SAMPLES: usize = 1;
const RSIZE: usize = 6; // Matrix rows
const CSIZE: usize = 12; // Matrix columns
const MSIZE: usize = RSIZE * CSIZE; // Total matrix size
type Matrix = kiibohd_hall_effect_keyscanning::Matrix<PioX<Output<PushPull>>, CSIZE, MSIZE>;

let mut matrix = Matrix::new(cols).unwrap();
matrix.next_strobe().unwrap(); // Strobe first column

// Retrieve adc sample and key index
let sample = read_adc();
let index = determine_key_index();

// Store the sample value at the specified index
// ADC_SAMPLES specifies how many samples are needed (averaged) until a processed sense value is returned
match matrix.record::<ADC_SAMPLES>(index, sample) {
		Ok(val) => {
				// If data bucket has accumulated enough samples, pass to the next stage
				if let Some(sense) = val {
						// Processed ADC data
				}
		}
		Err(e) => {
		    // Usually this is an index error
				defmt::error!("Sample record failed ({}, {}, {}):{} -> {}", i, strobe, index, sample, e);
		}

```

## Building

```bash
cargo build
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
