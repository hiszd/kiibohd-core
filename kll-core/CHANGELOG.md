# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.8 (2023-05-19)

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

### Bug Fixes

 - <csr-id-7a166007b9028882297472aa7143641cca178096/> const_ptr_read now stable
 - <csr-id-03c1db16dde4618a7c778c2180aa1f8ea948297d/> Dependency updates for kll-core and kll-compiler

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 164 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Const_ptr_read now stable ([`7a16600`](https://github.com/kiibohd/kiibohd-core/commit/7a166007b9028882297472aa7143641cca178096))
    - Dependency updates for kll-core and kll-compiler ([`03c1db1`](https://github.com/kiibohd/kiibohd-core/commit/03c1db16dde4618a7c778c2180aa1f8ea948297d))
    - Revamp kiibohd-hall-effect ([`d3191fc`](https://github.com/kiibohd/kiibohd-core/commit/d3191fc2c5ab90fc125c155554569b083a6a1545))
</details>

## 0.1.7 (2022-11-29)

### Bug Fixes

 - <csr-id-2265b9977161272386034d1550b73a7ec32334d2/> Upgrade byteorder and heapless
 - <csr-id-ed512c548d08b009fe34d4c638521a2accb2ce12/> Cleanup CHANGELOGs for cargo smart-release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release is31fl3743b v0.1.3, kll-core v0.1.7, kiibohd-hall-effect v0.1.3, kiibohd-keyscanning v0.1.3, kiibohd-hall-effect-keyscanning v0.1.3, kiibohd-hid-io v0.1.4, kiibohd-usb v0.1.5 ([`0cfed73`](https://github.com/kiibohd/kiibohd-core/commit/0cfed738eb237387c8c2c8b6ca0476cd5b4d4241))
    - Cleanup CHANGELOGs for cargo smart-release ([`ed512c5`](https://github.com/kiibohd/kiibohd-core/commit/ed512c548d08b009fe34d4c638521a2accb2ce12))
    - Upgrade byteorder and heapless ([`2265b99`](https://github.com/kiibohd/kiibohd-core/commit/2265b9977161272386034d1550b73a7ec32334d2))
</details>

## 0.1.6 (2022-11-29)

### Bug Fixes

 - <csr-id-5da78a4f2c7c359ceea2367a223beea5996a66d1/> Update CHANGELOGs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 11 calendar days.
 - 11 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release kll-macros v0.1.2, kll-core v0.1.6, kiibohd-hid-io v0.1.3 ([`cf9ad2e`](https://github.com/kiibohd/kiibohd-core/commit/cf9ad2ec744f0452856a1c778031665fe38c0e45))
    - Update CHANGELOGs ([`5da78a4`](https://github.com/kiibohd/kiibohd-core/commit/5da78a4f2c7c359ceea2367a223beea5996a66d1))
    - Update GitHub Actions (deny, pants, udeps) ([`b6ec165`](https://github.com/kiibohd/kiibohd-core/commit/b6ec165d19153d8acaffb8ff4ae8504fcfe7e40c))
    - Fix clippy warning ([`45583cb`](https://github.com/kiibohd/kiibohd-core/commit/45583cb9e5ed185df8b257984aa5a1b996d49160))
</details>

## 0.1.5 (2022-11-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 413 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#13](https://github.com/kiibohd/kiibohd-core/issues/13), [#2](https://github.com/kiibohd/kiibohd-core/issues/2), [#3](https://github.com/kiibohd/kiibohd-core/issues/3), [#4](https://github.com/kiibohd/kiibohd-core/issues/4)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#13](https://github.com/kiibohd/kiibohd-core/issues/13)**
    - Add keywords and categories to all the Cargo.toml (and fix a few typos) ([`4553cb4`](https://github.com/kiibohd/kiibohd-core/commit/4553cb456ab7df2e2874f03e385166e062787375))
 * **[#2](https://github.com/kiibohd/kiibohd-core/issues/2)**
    - Initial commit of macros. ([`cd3c6e0`](https://github.com/kiibohd/kiibohd-core/commit/cd3c6e0a228d5d6c77bc57307de427d8a4103226))
 * **[#3](https://github.com/kiibohd/kiibohd-core/issues/3)**
    - Added a few event conversions and carried out some refactoring ([`51aa09f`](https://github.com/kiibohd/kiibohd-core/commit/51aa09f0c59135d82bbc813103b11f3b5dfb0234))
 * **[#4](https://github.com/kiibohd/kiibohd-core/issues/4)**
    - Conversions for animations and led ([`2b09501`](https://github.com/kiibohd/kiibohd-core/commit/2b095013128063d9920c65fb8c74d43ceebae5cb))
 * **Uncategorized**
    - Release is31fl3743b v0.1.2, kll-hid v0.1.2, kll-macros v0.1.1, kll-core v0.1.5, kiibohd-hall-effect v0.1.2, kiibohd-keyscanning v0.1.2, kiibohd-hall-effect-keyscanning v0.1.2, kiibohd-hid-io v0.1.2, kiibohd-usb v0.1.3 ([`5a574aa`](https://github.com/kiibohd/kiibohd-core/commit/5a574aa1da0321613614c4d7f6f285fe149af409))
    - Fix changelogs ([`33ef4a3`](https://github.com/kiibohd/kiibohd-core/commit/33ef4a3f4fded7a8dd1f00510291f4075711186f))
    - Initial CHANGELOG.md ([`04edeeb`](https://github.com/kiibohd/kiibohd-core/commit/04edeebcb78d924d4b139b56c0b513633f7f95cc))
    - Arbitrary_enum_discriminant now stable in nightly ([`44abac3`](https://github.com/kiibohd/kiibohd-core/commit/44abac3e850be183bfa63a9b28363713ca99d1d5))
    - Update is31fl3743b and fix clippy warnings ([`f125eed`](https://github.com/kiibohd/kiibohd-core/commit/f125eed08a1b2d390b7b8d2fa563aeb2d5759b7e))
    - Add better debbuing for i331fl3743b crate ([`6416b1c`](https://github.com/kiibohd/kiibohd-core/commit/6416b1cf07440184ba088a077f59a7414a7fb8eb))
    - Stabilized compiler feature ([`8cd3098`](https://github.com/kiibohd/kiibohd-core/commit/8cd309877aa02639bb7de38a1a46890ad3637d08))
    - Simplifying log crate ([`5a8f450`](https://github.com/kiibohd/kiibohd-core/commit/5a8f4505c68c681b773e8cf6e96a62eeaef2c4d3))
    - [kll-core] Fix update status position ([`6b0c01d`](https://github.com/kiibohd/kiibohd-core/commit/6b0c01d4b3f452375a94847ced49297d5d27530f))
    - Increment versions (kll-core, kiibohd-usb) ([`0e9fbf4`](https://github.com/kiibohd/kiibohd-core/commit/0e9fbf40b9f9243f727d80c44a3cae64a4639968))
    - Adding Analog conversion support and fixing kiibohd-usb mouse support ([`4cc97e8`](https://github.com/kiibohd/kiibohd-core/commit/4cc97e8b8302f76ef006032e60ef7b3a2e613da0))
    - Fix missing defmt enable ([`0a3a5f4`](https://github.com/kiibohd/kiibohd-core/commit/0a3a5f48fc753d87ba2bcfe1bc8af845ae73fa5f))
    - Missing version ([`214e9cb`](https://github.com/kiibohd/kiibohd-core/commit/214e9cbb2dce64f7452af37f9e8b79993870b272))
    - Update defmt configurations ([`58c3aac`](https://github.com/kiibohd/kiibohd-core/commit/58c3aac6996ba72a24c12910e7875ecd2f6be969))
    - More clippy fixes ([`528672a`](https://github.com/kiibohd/kiibohd-core/commit/528672a0f7f255eb95cda7fd5423cfc553fa959e))
    - Increment patch ([`cc4f15f`](https://github.com/kiibohd/kiibohd-core/commit/cc4f15f18096cf75947204eab219c19f3dcaed18))
    - Add binary conversion to TriggerEvent ([`cd00256`](https://github.com/kiibohd/kiibohd-core/commit/cd0025615b4ab207426996b9541a7be78e81e0e8))
    - Update README.md ([`b6915fa`](https://github.com/kiibohd/kiibohd-core/commit/b6915facad7154f5d2f80dd57143eb41fdfd5d33))
    - Add kll-core support to kiibohd-hall-effect-keyscanning ([`d0a5c83`](https://github.com/kiibohd/kiibohd-core/commit/d0a5c8376f3b17bf3e3418e5466d095295d5137f))
    - Adding no-std keywords ([`59254c5`](https://github.com/kiibohd/kiibohd-core/commit/59254c5018132cb379790e6e0df6dc02f75b7c0f))
    - Adding process_off_state_lookups ([`babf695`](https://github.com/kiibohd/kiibohd-core/commit/babf695a81c0f31a5445ace0cdc383caa1eea873))
    - Updating Cargo.toml files to publish initial crates ([`e18dafb`](https://github.com/kiibohd/kiibohd-core/commit/e18dafb3802406146f6f70b522418d1139cec09c))
    - Adding README.md for kll-macros ([`603de2f`](https://github.com/kiibohd/kiibohd-core/commit/603de2f8172c09bb47ab1e038299a97bf79c4e4c))
    - Adding README.md for kll-core ([`8dfd29e`](https://github.com/kiibohd/kiibohd-core/commit/8dfd29efde09e92d4ec178f52374136d7239598d))
    - Resolve no_std compilation issues due to log ([`6f7df7c`](https://github.com/kiibohd/kiibohd-core/commit/6f7df7c1e830dec3d2138055c6c447054aba753e))
    - Convert kll-core validation test to a generic struct ([`3d06f99`](https://github.com/kiibohd/kiibohd-core/commit/3d06f990ec94655fb95b94323011197ee4d37894))
    - Initial generic kll -> kll-core validation test ([`0aa8806`](https://github.com/kiibohd/kiibohd-core/commit/0aa8806e5cfb9b811a2958c1b590a3e0d4f4bdfe))
    - Initial working kll-compiler -> kll-core flow ([`4a21b5a`](https://github.com/kiibohd/kiibohd-core/commit/4a21b5a2e5f1c2ffc9048975cc8948bc00fce663))
    - Adding layout support to kllcore emitter ([`9fa3cac`](https://github.com/kiibohd/kiibohd-core/commit/9fa3cacef661d3e1688fb20f113adc38f383bfc7))
    - Fixing power of 2 issues with heapless::Vec ([`8cce7c2`](https://github.com/kiibohd/kiibohd-core/commit/8cce7c29199561a1051c42a9c195fa577a335ee6))
    - Initial kll-core integration ([`3a5940f`](https://github.com/kiibohd/kiibohd-core/commit/3a5940fbe1a1445daa5b336b0f3041927cc9833f))
    - Initial IS31FL3743B support for atsam4 pdc ([`9674dc7`](https://github.com/kiibohd/kiibohd-core/commit/9674dc7410b51b0cc13a5a52118f3bf2e4651e7a))
    - Updating to defmt 0.3 ([`831f49e`](https://github.com/kiibohd/kiibohd-core/commit/831f49e1e4d8a3026417544604208a1b4a8243a1))
    - Upating to 2021 edition ([`ea8ed92`](https://github.com/kiibohd/kiibohd-core/commit/ea8ed9259590c31456b11eba01abdd4a8138bf32))
    - Fixing cargo fmt and clippy warnings ([`edcf4db`](https://github.com/kiibohd/kiibohd-core/commit/edcf4db1f62129b6f48a477e08883eb24ec4c057))
    - Small fixes ([`1ac32f2`](https://github.com/kiibohd/kiibohd-core/commit/1ac32f20649e8f6ded05af03606ff4a0793c3a9c))
    - Initial skeleton of kll-core implementation ([`025dcea`](https://github.com/kiibohd/kiibohd-core/commit/025dceaa4c3e311de4ab34679b1f7fa0a2a1f84e))
</details>

