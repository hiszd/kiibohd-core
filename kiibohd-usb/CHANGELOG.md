# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.6 (2023-05-19)

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

 - 2 commits contributed to the release over the course of 6 calendar days.
 - 170 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release kll-core v0.1.8, kiibohd-hall-effect v0.2.0, kiibohd-keyscanning v0.1.4, kiibohd-hall-effect-keyscanning v0.2.0, safety bump kiibohd-hall-effect-keyscanning v0.2.0 ([`e15da97`](https://github.com/kiibohd/kiibohd-core/commit/e15da97be5b16e0298a22176486430ebce069c1c))
    - Revamp kiibohd-hall-effect ([`d3191fc`](https://github.com/kiibohd/kiibohd-core/commit/d3191fc2c5ab90fc125c155554569b083a6a1545))
</details>

## 0.1.5 (2022-11-29)

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

## 0.1.4 (2022-11-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Adjusting changelogs prior to release of kiibohd-usb v0.1.4 ([`4191d69`](https://github.com/kiibohd/kiibohd-core/commit/4191d69f9d180a27a8b2759fa60c4adccfaded15))
    - Usb-device and usbd-hid patches have been merged upstream ([`1f218d8`](https://github.com/kiibohd/kiibohd-core/commit/1f218d80657b55cac6d9f07aeaf4491c6798002e))
</details>

## 0.1.3 (2022-11-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 35 commits contributed to the release over the course of 540 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#13](https://github.com/kiibohd/kiibohd-core/issues/13)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#13](https://github.com/kiibohd/kiibohd-core/issues/13)**
    - Add keywords and categories to all the Cargo.toml (and fix a few typos) ([`4553cb4`](https://github.com/kiibohd/kiibohd-core/commit/4553cb456ab7df2e2874f03e385166e062787375))
 * **Uncategorized**
    - Adjusting changelogs prior to release of kiibohd-usb v0.1.3 ([`8d8bff3`](https://github.com/kiibohd/kiibohd-core/commit/8d8bff34fcf48f89d59dede7b8d7104a6a659cf2))
    - Release is31fl3743b v0.1.2, kll-hid v0.1.2, kll-macros v0.1.1, kll-core v0.1.5, kiibohd-hall-effect v0.1.2, kiibohd-keyscanning v0.1.2, kiibohd-hall-effect-keyscanning v0.1.2, kiibohd-hid-io v0.1.2, kiibohd-usb v0.1.3 ([`5a574aa`](https://github.com/kiibohd/kiibohd-core/commit/5a574aa1da0321613614c4d7f6f285fe149af409))
    - Fix changelogs ([`33ef4a3`](https://github.com/kiibohd/kiibohd-core/commit/33ef4a3f4fded7a8dd1f00510291f4075711186f))
    - Initial CHANGELOG.md ([`04edeeb`](https://github.com/kiibohd/kiibohd-core/commit/04edeebcb78d924d4b139b56c0b513633f7f95cc))
    - Add better debbuing for i331fl3743b crate ([`6416b1c`](https://github.com/kiibohd/kiibohd-core/commit/6416b1cf07440184ba088a077f59a7414a7fb8eb))
    - Usbd-hid now uses defmt instead of defmt-impl feature ([`4039041`](https://github.com/kiibohd/kiibohd-core/commit/4039041f1e79ad10fd87e3c2536da4f4b240feea))
    - [kiibohd-usb] Adding HID Lock LED support ([`ce32c30`](https://github.com/kiibohd/kiibohd-core/commit/ce32c302c003900690c645d70ea2c97e87b370ce))
    - [kiibohd-usb] Fix remote wakeup and nkro support ([`3aa9f7e`](https://github.com/kiibohd/kiibohd-core/commit/3aa9f7e9273f1d64933f9fe2a0c8c37960cea705))
    - Kiibohd-usb now passes USB compliance HID Tests ([`63a6b3e`](https://github.com/kiibohd/kiibohd-core/commit/63a6b3eebcc1578aa294fc88831b4f0d675fb82f))
    - Increment versions (kll-core, kiibohd-usb) ([`0e9fbf4`](https://github.com/kiibohd/kiibohd-core/commit/0e9fbf40b9f9243f727d80c44a3cae64a4639968))
    - Adding Analog conversion support and fixing kiibohd-usb mouse support ([`4cc97e8`](https://github.com/kiibohd/kiibohd-core/commit/4cc97e8b8302f76ef006032e60ef7b3a2e613da0))
    - Re-enable for git usage ([`fb219cc`](https://github.com/kiibohd/kiibohd-core/commit/fb219cca16bb8f08650d25a0b0291b484700817c))
    - Handling usb-device crate temp issue ([`0a05523`](https://github.com/kiibohd/kiibohd-core/commit/0a055232dd42478aaff72810889c6e0820425f5e))
    - Update defmt configurations ([`58c3aac`](https://github.com/kiibohd/kiibohd-core/commit/58c3aac6996ba72a24c12910e7875ecd2f6be969))
    - More clippy fixes ([`528672a`](https://github.com/kiibohd/kiibohd-core/commit/528672a0f7f255eb95cda7fd5423cfc553fa959e))
    - Increment patch ([`cc4f15f`](https://github.com/kiibohd/kiibohd-core/commit/cc4f15f18096cf75947204eab219c19f3dcaed18))
    - Update README.md ([`b08610d`](https://github.com/kiibohd/kiibohd-core/commit/b08610d8d975776f9ad749985d8e8a7616b8559e))
    - Cargo fmt ([`c37456d`](https://github.com/kiibohd/kiibohd-core/commit/c37456d7bfb1f032a0947e4aeb19ea24761e8e7a))
    - Support custom crates.io packages for usb ([`59b8e0f`](https://github.com/kiibohd/kiibohd-core/commit/59b8e0f43f10021c1758b8f44b224bd4be008e31))
    - Set versions for kiibohd-usb ([`33999e3`](https://github.com/kiibohd/kiibohd-core/commit/33999e3e2468d881d89ce4a035369bf4dacfdbd0))
    - Updating Cargo.toml files to publish initial crates ([`e18dafb`](https://github.com/kiibohd/kiibohd-core/commit/e18dafb3802406146f6f70b522418d1139cec09c))
    - Add enqueue_ functions for kiibohd-usb ([`bc989f9`](https://github.com/kiibohd/kiibohd-core/commit/bc989f9c81098047396de4c49f13034df9fd9c88))
    - Fixing power of 2 issues with heapless::Vec ([`8cce7c2`](https://github.com/kiibohd/kiibohd-core/commit/8cce7c29199561a1051c42a9c195fa577a335ee6))
    - Updating to defmt 0.3 ([`831f49e`](https://github.com/kiibohd/kiibohd-core/commit/831f49e1e4d8a3026417544604208a1b4a8243a1))
    - Upating to 2021 edition ([`ea8ed92`](https://github.com/kiibohd/kiibohd-core/commit/ea8ed9259590c31456b11eba01abdd4a8138bf32))
    - Initial skeleton of kll-core implementation ([`025dcea`](https://github.com/kiibohd/kiibohd-core/commit/025dceaa4c3e311de4ab34679b1f7fa0a2a1f84e))
    - Updating to new usbd-hid new_ep_in_with_settings() api ([`7f1fd76`](https://github.com/kiibohd/kiibohd-core/commit/7f1fd762c19964fe50835cb462220d0ad3098039))
    - Adding defmt support to kiibohd-usb ([`d941980`](https://github.com/kiibohd/kiibohd-core/commit/d941980ff0ab56009ec794c2783ebc186882369c))
    - Cleanup cargo fmt ([`764b0ae`](https://github.com/kiibohd/kiibohd-core/commit/764b0ae9b37c08d3201e64096719e8529387ef0d))
    - Splitting hid-io into rust and ffi versions ([`5746c10`](https://github.com/kiibohd/kiibohd-core/commit/5746c1015242c5cf21d603da1f7220bcb06c64a0))
    - Add missing README.md for kiibohd-usb ([`75c89e5`](https://github.com/kiibohd/kiibohd-core/commit/75c89e5151fc067ad127d27bce537d524935f497))
    - Updates to kiibohd-log and kiibohd-usb ([`231fccb`](https://github.com/kiibohd/kiibohd-core/commit/231fccb8df2732bdfab30ed92faa956ec1ecfe17))
    - Small touch-ups to kiibohd-log and kiibohd-usb ([`7faf2b9`](https://github.com/kiibohd/kiibohd-core/commit/7faf2b9cb92c292c4dfd656e7346aa040507159d))
    - Adding initial kiibohd-log and kiibohd-usb ([`547cd8e`](https://github.com/kiibohd/kiibohd-core/commit/547cd8e15da8d664c68f2af899b0bbacb5037eb1))
</details>

