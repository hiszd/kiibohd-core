# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 (2023-05-19)

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
 - <csr-id-2fa2b8cd1b063f9bae2b2d1656f57b312788ea45/> Directional ADC value filtering
   - Only update values immediately in the current direction of the filter
     (any value)
   - Opposite direction values (i.e. 400 then 390) must be a difference of at least MAX_DEV
     otherwise the samples in the set will be ignored/disregarded
   - If the difference is large enough, change the direction of the filter
   - This should greatly reduce ADC value movement, while still giving
     excellent sensitivity in the desired direction
 - <csr-id-44d34a802706ba1115b104c242ca4db48eb37547/> Support for ADC sample deviation exclusion
   - For ADCs that support multiple samples at a specific instance compare
     each of the samples against a set deviation
   - If the deviation between the samples is too high, reject all the
     samples
   - This helps with spurious ADC noise and smooths out the resulting data
     used for analysis
   - Generally this should be infrequent and imperceptible to the user

### Bug Fixes

 - <csr-id-bef7bcb06d45e09db02e199451432509ab05e331/> Update averaging to use the previous value instead of scratch samples
   - Using the previous computed value instead of the previous scratch
     samples reduces sample bounce (decaying filter)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 78 calendar days.
 - 164 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Revamp kiibohd-hall-effect ([`d3191fc`](https://github.com/kiibohd/kiibohd-core/commit/d3191fc2c5ab90fc125c155554569b083a6a1545))
    - Update averaging to use the previous value instead of scratch samples ([`bef7bcb`](https://github.com/kiibohd/kiibohd-core/commit/bef7bcb06d45e09db02e199451432509ab05e331))
    - Directional ADC value filtering ([`2fa2b8c`](https://github.com/kiibohd/kiibohd-core/commit/2fa2b8cd1b063f9bae2b2d1656f57b312788ea45))
    - Support for ADC sample deviation exclusion ([`44d34a8`](https://github.com/kiibohd/kiibohd-core/commit/44d34a802706ba1115b104c242ca4db48eb37547))
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

 - 27 commits contributed to the release over the course of 636 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#13](https://github.com/kiibohd/kiibohd-core/issues/13)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#13](https://github.com/kiibohd/kiibohd-core/issues/13)**
    - Add keywords and categories to all the Cargo.toml (and fix a few typos) ([`4553cb4`](https://github.com/kiibohd/kiibohd-core/commit/4553cb456ab7df2e2874f03e385166e062787375))
 * **Uncategorized**
    - Release is31fl3743b v0.1.2, kll-hid v0.1.2, kll-macros v0.1.1, kll-core v0.1.5, kiibohd-hall-effect v0.1.2, kiibohd-keyscanning v0.1.2, kiibohd-hall-effect-keyscanning v0.1.2, kiibohd-hid-io v0.1.2, kiibohd-usb v0.1.3 ([`5a574aa`](https://github.com/kiibohd/kiibohd-core/commit/5a574aa1da0321613614c4d7f6f285fe149af409))
    - Fix changelogs ([`33ef4a3`](https://github.com/kiibohd/kiibohd-core/commit/33ef4a3f4fded7a8dd1f00510291f4075711186f))
    - Initial CHANGELOG.md ([`04edeeb`](https://github.com/kiibohd/kiibohd-core/commit/04edeebcb78d924d4b139b56c0b513633f7f95cc))
    - Add better debbuing for i331fl3743b crate ([`6416b1c`](https://github.com/kiibohd/kiibohd-core/commit/6416b1cf07440184ba088a077f59a7414a7fb8eb))
    - Simplifying log crate ([`5a8f450`](https://github.com/kiibohd/kiibohd-core/commit/5a8f4505c68c681b773e8cf6e96a62eeaef2c4d3))
    - Update defmt configurations ([`58c3aac`](https://github.com/kiibohd/kiibohd-core/commit/58c3aac6996ba72a24c12910e7875ecd2f6be969))
    - More clippy fixes ([`528672a`](https://github.com/kiibohd/kiibohd-core/commit/528672a0f7f255eb95cda7fd5423cfc553fa959e))
    - Increment patch ([`cc4f15f`](https://github.com/kiibohd/kiibohd-core/commit/cc4f15f18096cf75947204eab219c19f3dcaed18))
    - Update README.md ([`aeacfb2`](https://github.com/kiibohd/kiibohd-core/commit/aeacfb274fe2b57d410aa63c594af047edccf3f7))
    - Add kll-core support to kiibohd-hall-effect-keyscanning ([`d0a5c83`](https://github.com/kiibohd/kiibohd-core/commit/d0a5c8376f3b17bf3e3418e5466d095295d5137f))
    - Adding no-std keywords ([`59254c5`](https://github.com/kiibohd/kiibohd-core/commit/59254c5018132cb379790e6e0df6dc02f75b7c0f))
    - Updating Cargo.toml files to publish initial crates ([`e18dafb`](https://github.com/kiibohd/kiibohd-core/commit/e18dafb3802406146f6f70b522418d1139cec09c))
    - Resolve no_std compilation issues due to log ([`6f7df7c`](https://github.com/kiibohd/kiibohd-core/commit/6f7df7c1e830dec3d2138055c6c447054aba753e))
    - Fixing power of 2 issues with heapless::Vec ([`8cce7c2`](https://github.com/kiibohd/kiibohd-core/commit/8cce7c29199561a1051c42a9c195fa577a335ee6))
    - Updating to defmt 0.3 ([`831f49e`](https://github.com/kiibohd/kiibohd-core/commit/831f49e1e4d8a3026417544604208a1b4a8243a1))
    - Upating to 2021 edition ([`ea8ed92`](https://github.com/kiibohd/kiibohd-core/commit/ea8ed9259590c31456b11eba01abdd4a8138bf32))
    - Fixing cargo fmt typo ([`0f8f032`](https://github.com/kiibohd/kiibohd-core/commit/0f8f032216a3c4294a920b092114ed56b6cde764))
    - Adding basic kiibohd-hall-effect-keyscanning crate ([`78607a0`](https://github.com/kiibohd/kiibohd-core/commit/78607a0b7e3c5f1d2f915eb18f47d77ca207fa93))
    - Upgrading to heapless 0.7 ([`167a127`](https://github.com/kiibohd/kiibohd-core/commit/167a127642ef371d6d9d9a644a3f63816a408f14))
    - Updates to kiibohd-log and kiibohd-usb ([`231fccb`](https://github.com/kiibohd/kiibohd-core/commit/231fccb8df2732bdfab30ed92faa956ec1ecfe17))
    - Updating license to MIT+Apache 2.0 ([`1497fd7`](https://github.com/kiibohd/kiibohd-core/commit/1497fd7d369e16f6f4cc7590e903661c5bc47026))
    - Updating to use two separate analysis modes ([`a72fc64`](https://github.com/kiibohd/kiibohd-core/commit/a72fc64b56cd7547e0ccbd994e92f40c550447d6))
    - Adding two run modes: Calibration and Normal ([`a2e1478`](https://github.com/kiibohd/kiibohd-core/commit/a2e1478d475afd982ad92428850c82e4e9d7d6f8))
    - Adding missing DeviceVersion field ([`86747b4`](https://github.com/kiibohd/kiibohd-core/commit/86747b4d242a8cec51217516eb1b8d65bc8befc3))
    - Finished adding basic kiibohd-hall-effect unit tests ([`ffe1cc2`](https://github.com/kiibohd/kiibohd-core/commit/ffe1cc230e259c1e7f8b240985395a6d72076cda))
    - Moving hid-io-kiibohd and initial version of kiibohd-hall-effect ([`d1a109a`](https://github.com/kiibohd/kiibohd-core/commit/d1a109afca0a84cf6943b3234db917c1862ba571))
</details>

