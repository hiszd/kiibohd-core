# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.4 (2023-05-19)

### New Features

 - <csr-id-d3191fc2c5ab90fc125c155554569b083a6a1545/> Revamp kiibohd-hall-effect
   - Different modes
     * Normal (best precision, additional processing)
     * Low-latency mode (best precision, optimized for latency)
     * Test mode (widest range, lower precision)
   - Remove C compatibility (no longer planned)
   - On/off event generator (primilarily for low-latency mode)
   - No more sample averaging
     * Only use averaging when deciding when to recalibrate
   - Recalibrate within a specified range after a period of stability is
     detected
     * Allows for per-key temperature and humidity stability
     * Can dynamically adjust for new switches (different magnetic
       strength)
   - While not supported, test mode does support detecting magnet polarity
     * Per-key modes are not supported (as the ADC will need to switch per
       strobe and this may involve recalibration, which is slow)
   - Add TriggerEventIterator
     * Easier interface to allow to generate N number of trigger events
       from a single call (instead of using a fixed size vector)
   - Add Activate/Deactive USB HID LED states

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 164 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Revamp kiibohd-hall-effect ([`d3191fc`](https://github.com/kiibohd/kiibohd-core/commit/d3191fc2c5ab90fc125c155554569b083a6a1545))
</details>

## 0.1.3 (2022-11-29)

### Fixes

- General fixes

### Bug Fixes

 - <csr-id-ed512c548d08b009fe34d4c638521a2accb2ce12/> Cleanup CHANGELOGs for cargo smart-release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 12 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release is31fl3743b v0.1.3, kll-core v0.1.7, kiibohd-hall-effect v0.1.3, kiibohd-keyscanning v0.1.3, kiibohd-hall-effect-keyscanning v0.1.3, kiibohd-hid-io v0.1.4, kiibohd-usb v0.1.5 ([`0cfed73`](https://github.com/kiibohd/kiibohd-core/commit/0cfed738eb237387c8c2c8b6ca0476cd5b4d4241))
    - Cleanup CHANGELOGs for cargo smart-release ([`ed512c5`](https://github.com/kiibohd/kiibohd-core/commit/ed512c548d08b009fe34d4c638521a2accb2ce12))
    - Release kll-macros v0.1.2, kll-core v0.1.6, kiibohd-hid-io v0.1.3 ([`cf9ad2e`](https://github.com/kiibohd/kiibohd-core/commit/cf9ad2ec744f0452856a1c778031665fe38c0e45))
    - Update GitHub Actions (deny, pants, udeps) ([`b6ec165`](https://github.com/kiibohd/kiibohd-core/commit/b6ec165d19153d8acaffb8ff4ae8504fcfe7e40c))
</details>

## 0.1.2 (2022-11-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 523 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#1](https://github.com/kiibohd/kiibohd-core/issues/1), [#13](https://github.com/kiibohd/kiibohd-core/issues/13)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1](https://github.com/kiibohd/kiibohd-core/issues/1)**
    - Keyscanning module initial merge ([`0dba8f8`](https://github.com/kiibohd/kiibohd-core/commit/0dba8f88fbd9cc42618398afb54c1b522ad37858))
 * **[#13](https://github.com/kiibohd/kiibohd-core/issues/13)**
    - Add keywords and categories to all the Cargo.toml (and fix a few typos) ([`4553cb4`](https://github.com/kiibohd/kiibohd-core/commit/4553cb456ab7df2e2874f03e385166e062787375))
 * **Uncategorized**
    - Release is31fl3743b v0.1.2, kll-hid v0.1.2, kll-macros v0.1.1, kll-core v0.1.5, kiibohd-hall-effect v0.1.2, kiibohd-keyscanning v0.1.2, kiibohd-hall-effect-keyscanning v0.1.2, kiibohd-hid-io v0.1.2, kiibohd-usb v0.1.3 ([`5a574aa`](https://github.com/kiibohd/kiibohd-core/commit/5a574aa1da0321613614c4d7f6f285fe149af409))
    - Fix changelogs ([`33ef4a3`](https://github.com/kiibohd/kiibohd-core/commit/33ef4a3f4fded7a8dd1f00510291f4075711186f))
    - Initial CHANGELOG.md ([`04edeeb`](https://github.com/kiibohd/kiibohd-core/commit/04edeebcb78d924d4b139b56c0b513633f7f95cc))
    - Cargo fmt ([`8e38526`](https://github.com/kiibohd/kiibohd-core/commit/8e385266d5c631630c95fec6fb13808e1395de0a))
    - Add KeyScanning trait ([`218896b`](https://github.com/kiibohd/kiibohd-core/commit/218896b335f0b46d7cf9d5430afb8a98feb2c4b7))
    - Add better debbuing for i331fl3743b crate ([`6416b1c`](https://github.com/kiibohd/kiibohd-core/commit/6416b1cf07440184ba088a077f59a7414a7fb8eb))
    - [kiibohd-keyscanning] Add off state ignore option ([`5cd975c`](https://github.com/kiibohd/kiibohd-core/commit/5cd975c07908246fd49f8550ecceec7220e6ae0e))
    - Update defmt configurations ([`58c3aac`](https://github.com/kiibohd/kiibohd-core/commit/58c3aac6996ba72a24c12910e7875ecd2f6be969))
    - More clippy fixes ([`528672a`](https://github.com/kiibohd/kiibohd-core/commit/528672a0f7f255eb95cda7fd5423cfc553fa959e))
    - Increment patch ([`cc4f15f`](https://github.com/kiibohd/kiibohd-core/commit/cc4f15f18096cf75947204eab219c19f3dcaed18))
    - Update README.md ([`d7fe786`](https://github.com/kiibohd/kiibohd-core/commit/d7fe786cb66298bbaf0a8848963193f4216a2bd3))
    - Add kll-core support to kiibohd-hall-effect-keyscanning ([`d0a5c83`](https://github.com/kiibohd/kiibohd-core/commit/d0a5c8376f3b17bf3e3418e5466d095295d5137f))
    - Adding no-std keywords ([`59254c5`](https://github.com/kiibohd/kiibohd-core/commit/59254c5018132cb379790e6e0df6dc02f75b7c0f))
    - Cargo fmt ([`c37456d`](https://github.com/kiibohd/kiibohd-core/commit/c37456d7bfb1f032a0947e4aeb19ea24761e8e7a))
    - Adding README.md for kll-macros ([`603de2f`](https://github.com/kiibohd/kiibohd-core/commit/603de2f8172c09bb47ab1e038299a97bf79c4e4c))
    - Add Off-state event generated (generate_event) ([`310b013`](https://github.com/kiibohd/kiibohd-core/commit/310b013360a8a46636c756aae2d9da5b9bcad4fb))
    - Adding kll-core KeyEvent to TriggerEvent conversion ([`eb54635`](https://github.com/kiibohd/kiibohd-core/commit/eb54635c7ae2735dc9660fc08a668bb11f9bc2a6))
    - Adding state() lookup to kiibohd-keyscanning ([`0ebd4d1`](https://github.com/kiibohd/kiibohd-core/commit/0ebd4d14ef797db38d479bba41f5e2fb0c705d67))
    - Fixing power of 2 issues with heapless::Vec ([`8cce7c2`](https://github.com/kiibohd/kiibohd-core/commit/8cce7c29199561a1051c42a9c195fa577a335ee6))
    - Updating to defmt 0.3 ([`831f49e`](https://github.com/kiibohd/kiibohd-core/commit/831f49e1e4d8a3026417544604208a1b4a8243a1))
    - Fixing multiplication overflow panic ([`985c72d`](https://github.com/kiibohd/kiibohd-core/commit/985c72dc69e8861566bc705e3ec9ee5f3e856d37))
    - Cargo fmt ([`64995b8`](https://github.com/kiibohd/kiibohd-core/commit/64995b8459bf1027d8171d57e7fb9f2c75ce33f8))
    - Added missing column size constant to timing calculations ([`70e8597`](https://github.com/kiibohd/kiibohd-core/commit/70e85978a85b1bafdfb125f815ed13798b07f874))
    - Updating kiibohd-keyscanning ([`1c51025`](https://github.com/kiibohd/kiibohd-core/commit/1c51025e8568e4e00571527b87a3ea8d20c251c8))
    - Refactored kiibohd-keyscanning module ([`999bf4d`](https://github.com/kiibohd/kiibohd-core/commit/999bf4d7d14cee85ca1351df67cfef805f23bda2))
</details>

