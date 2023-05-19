# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.3 (2023-05-19)

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

 - <csr-id-8250b4ee70829c403266ac9392a5678bbb73bcef/> Update kiibohd-hid-io changelog
 - <csr-id-d35b8681c506c03f37aa5db90fb1f0c8e71a3faf/> Inconsistent hash key
   - The binary format that kll_core generates is not consistent when it
     comes to unused packing bytes.
     This means that using Vec<u8> of that TriggerList (or ResultList) may
     not be consistent between usages in different functions (or
     iterations).
   - To fix this, create a hash key safe version of the trigger (and
     release) guides
   - This doesn't effect kll at runtime as the padding bytes are ignored
 - <csr-id-b5eda36360794b261d3bb03430d3a615d4cf1525/> kll-compiler - Reduce variable clones
   - No need to keep multiple copies in memory
 - <csr-id-7a166007b9028882297472aa7143641cca178096/> const_ptr_read now stable
 - <csr-id-03c1db16dde4618a7c778c2180aa1f8ea948297d/> Dependency updates for kll-core and kll-compiler
 - <csr-id-bef7bcb06d45e09db02e199451432509ab05e331/> Update averaging to use the previous value instead of scratch samples
   - Using the previous computed value instead of the previous scratch
     samples reduces sample bounce (decaying filter)
 - <csr-id-ed512c548d08b009fe34d4c638521a2accb2ce12/> Cleanup CHANGELOGs for cargo smart-release
 - <csr-id-2265b9977161272386034d1550b73a7ec32334d2/> Upgrade byteorder and heapless

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 170 calendar days.
 - 170 days passed between releases.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release kiibohd-hid-io v0.1.5, kiibohd-usb v0.1.6 ([`4538c36`](https://github.com/kiibohd/kiibohd-core/commit/4538c3688d070fcbb0b3b1a5ade481016b7e5e27))
    - Update kiibohd-hid-io changelog ([`8250b4e`](https://github.com/kiibohd/kiibohd-core/commit/8250b4ee70829c403266ac9392a5678bbb73bcef))
    - Release kll-core v0.1.8, kiibohd-hall-effect v0.2.0, kiibohd-keyscanning v0.1.4, kiibohd-hall-effect-keyscanning v0.2.0, safety bump kiibohd-hall-effect-keyscanning v0.2.0 ([`e15da97`](https://github.com/kiibohd/kiibohd-core/commit/e15da97be5b16e0298a22176486430ebce069c1c))
    - Inconsistent hash key ([`d35b868`](https://github.com/kiibohd/kiibohd-core/commit/d35b8681c506c03f37aa5db90fb1f0c8e71a3faf))
    - Kll-compiler - Reduce variable clones ([`b5eda36`](https://github.com/kiibohd/kiibohd-core/commit/b5eda36360794b261d3bb03430d3a615d4cf1525))
    - Const_ptr_read now stable ([`7a16600`](https://github.com/kiibohd/kiibohd-core/commit/7a166007b9028882297472aa7143641cca178096))
    - Dependency updates for kll-core and kll-compiler ([`03c1db1`](https://github.com/kiibohd/kiibohd-core/commit/03c1db16dde4618a7c778c2180aa1f8ea948297d))
    - Revamp kiibohd-hall-effect ([`d3191fc`](https://github.com/kiibohd/kiibohd-core/commit/d3191fc2c5ab90fc125c155554569b083a6a1545))
    - Update averaging to use the previous value instead of scratch samples ([`bef7bcb`](https://github.com/kiibohd/kiibohd-core/commit/bef7bcb06d45e09db02e199451432509ab05e331))
    - Directional ADC value filtering ([`2fa2b8c`](https://github.com/kiibohd/kiibohd-core/commit/2fa2b8cd1b063f9bae2b2d1656f57b312788ea45))
    - Support for ADC sample deviation exclusion ([`44d34a8`](https://github.com/kiibohd/kiibohd-core/commit/44d34a802706ba1115b104c242ca4db48eb37547))
    - Release is31fl3743b v0.1.3, kll-core v0.1.7, kiibohd-hall-effect v0.1.3, kiibohd-keyscanning v0.1.3, kiibohd-hall-effect-keyscanning v0.1.3, kiibohd-hid-io v0.1.4, kiibohd-usb v0.1.5 ([`0cfed73`](https://github.com/kiibohd/kiibohd-core/commit/0cfed738eb237387c8c2c8b6ca0476cd5b4d4241))
    - Cleanup CHANGELOGs for cargo smart-release ([`ed512c5`](https://github.com/kiibohd/kiibohd-core/commit/ed512c548d08b009fe34d4c638521a2accb2ce12))
    - Upgrade byteorder and heapless ([`2265b99`](https://github.com/kiibohd/kiibohd-core/commit/2265b9977161272386034d1550b73a7ec32334d2))
</details>

## 0.1.2 (2022-11-29)

<csr-id-2e5b8067349ebca66e1da4faaea43c8611dbaf80/>

### Changes

- Migrate structopts to clap v3

### Other

 - <csr-id-2e5b8067349ebca66e1da4faaea43c8611dbaf80/> hid-io-protocol -> v0.1.4

### Bug Fixes

 - <csr-id-5da78a4f2c7c359ceea2367a223beea5996a66d1/> Update CHANGELOGs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 11 calendar days.
 - 11 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release kll-compiler v0.1.2 ([`17ba165`](https://github.com/kiibohd/kiibohd-core/commit/17ba1656b96c9ab6eeaabd4060a5d862b89d1a81))
    - Update CHANGELOG ([`19b4dac`](https://github.com/kiibohd/kiibohd-core/commit/19b4dacc658dc34aaa9761a0b4e125c01438322e))
    - Release kll-macros v0.1.2, kll-core v0.1.6, kiibohd-hid-io v0.1.3 ([`cf9ad2e`](https://github.com/kiibohd/kiibohd-core/commit/cf9ad2ec744f0452856a1c778031665fe38c0e45))
    - Update CHANGELOGs ([`5da78a4`](https://github.com/kiibohd/kiibohd-core/commit/5da78a4f2c7c359ceea2367a223beea5996a66d1))
    - Hid-io-protocol -> v0.1.4 ([`2e5b806`](https://github.com/kiibohd/kiibohd-core/commit/2e5b8067349ebca66e1da4faaea43c8611dbaf80))
    - Update GitHub Actions (deny, pants, udeps) ([`b6ec165`](https://github.com/kiibohd/kiibohd-core/commit/b6ec165d19153d8acaffb8ff4ae8504fcfe7e40c))
    - GitHub Actions - Fix toolchain action ([`2ca3bb4`](https://github.com/kiibohd/kiibohd-core/commit/2ca3bb40454e072a5cf3c28f6a911e0e505c4f54))
    - GitHub Actions update ([`51ab9e8`](https://github.com/kiibohd/kiibohd-core/commit/51ab9e8c26ec0fccbf1ddbe8cdb7afd1f9bdd05f))
    - Fix clippy warning ([`45583cb`](https://github.com/kiibohd/kiibohd-core/commit/45583cb9e5ed185df8b257984aa5a1b996d49160))
    - Release kiibohd-usb v0.1.4 ([`ebe27a6`](https://github.com/kiibohd/kiibohd-core/commit/ebe27a62c73c3cc489be911581528072baa1a058))
    - Adjusting changelogs prior to release of kiibohd-usb v0.1.4 ([`4191d69`](https://github.com/kiibohd/kiibohd-core/commit/4191d69f9d180a27a8b2759fa60c4adccfaded15))
    - Usb-device and usbd-hid patches have been merged upstream ([`1f218d8`](https://github.com/kiibohd/kiibohd-core/commit/1f218d80657b55cac6d9f07aeaf4491c6798002e))
    - Release kiibohd-usb v0.1.3 ([`c688091`](https://github.com/kiibohd/kiibohd-core/commit/c688091c1c2ab9863700543598fb6ead9e1ad35f))
    - Adjusting changelogs prior to release of kiibohd-usb v0.1.3 ([`8d8bff3`](https://github.com/kiibohd/kiibohd-core/commit/8d8bff34fcf48f89d59dede7b8d7104a6a659cf2))
    - Release kiibohd-hid-io v0.1.2 ([`7bbcb23`](https://github.com/kiibohd/kiibohd-core/commit/7bbcb233604fffa6f86c64dc6b897091199c2dc4))
    - Adjusting changelogs prior to release of kiibohd-hid-io v0.1.2 ([`f7ed5a7`](https://github.com/kiibohd/kiibohd-core/commit/f7ed5a7a4c2a93a8ea960cf756c24a7eb726e4be))
    - Update to hid-io-protocol v0.1.3 ([`2e4b8c6`](https://github.com/kiibohd/kiibohd-core/commit/2e4b8c6ceab6adcb6a81f88fd68e866edd2ed67a))
    - Release is31fl3743b v0.1.2, kll-hid v0.1.2, kll-macros v0.1.1, kll-core v0.1.5, kiibohd-hall-effect v0.1.2, kiibohd-keyscanning v0.1.2, kiibohd-hall-effect-keyscanning v0.1.2, kiibohd-hid-io v0.1.2, kiibohd-usb v0.1.3 ([`5a574aa`](https://github.com/kiibohd/kiibohd-core/commit/5a574aa1da0321613614c4d7f6f285fe149af409))
</details>

## 0.1.1 (2022-11-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 152 commits contributed to the release over the course of 639 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#1](https://github.com/kiibohd/kiibohd-core/issues/1), [#13](https://github.com/kiibohd/kiibohd-core/issues/13), [#2](https://github.com/kiibohd/kiibohd-core/issues/2), [#3](https://github.com/kiibohd/kiibohd-core/issues/3), [#4](https://github.com/kiibohd/kiibohd-core/issues/4)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1](https://github.com/kiibohd/kiibohd-core/issues/1)**
    - Keyscanning module initial merge ([`0dba8f8`](https://github.com/kiibohd/kiibohd-core/commit/0dba8f88fbd9cc42618398afb54c1b522ad37858))
 * **[#13](https://github.com/kiibohd/kiibohd-core/issues/13)**
    - Add keywords and categories to all the Cargo.toml (and fix a few typos) ([`4553cb4`](https://github.com/kiibohd/kiibohd-core/commit/4553cb456ab7df2e2874f03e385166e062787375))
 * **[#2](https://github.com/kiibohd/kiibohd-core/issues/2)**
    - Initial commit of macros. ([`cd3c6e0`](https://github.com/kiibohd/kiibohd-core/commit/cd3c6e0a228d5d6c77bc57307de427d8a4103226))
 * **[#3](https://github.com/kiibohd/kiibohd-core/issues/3)**
    - Added a few event conversions and carried out some refactoring ([`51aa09f`](https://github.com/kiibohd/kiibohd-core/commit/51aa09f0c59135d82bbc813103b11f3b5dfb0234))
 * **[#4](https://github.com/kiibohd/kiibohd-core/issues/4)**
    - Conversions for animations and led ([`2b09501`](https://github.com/kiibohd/kiibohd-core/commit/2b095013128063d9920c65fb8c74d43ceebae5cb))
 * **Uncategorized**
    - Release kll-compiler v0.1.1 ([`e03888f`](https://github.com/kiibohd/kiibohd-core/commit/e03888f89f82b81e9fd3566e01af8e8359912034))
    - Update pest and ignore clippy warnings ([`bd95015`](https://github.com/kiibohd/kiibohd-core/commit/bd950153f7d6b537d4e9c701e97f822668d6424d))
    - Fix changelogs ([`33ef4a3`](https://github.com/kiibohd/kiibohd-core/commit/33ef4a3f4fded7a8dd1f00510291f4075711186f))
    - Initial CHANGELOG.md ([`04edeeb`](https://github.com/kiibohd/kiibohd-core/commit/04edeebcb78d924d4b139b56c0b513633f7f95cc))
    - Arbitrary_enum_discriminant now stable in nightly ([`44abac3`](https://github.com/kiibohd/kiibohd-core/commit/44abac3e850be183bfa63a9b28363713ca99d1d5))
    - Cargo fmt ([`8e38526`](https://github.com/kiibohd/kiibohd-core/commit/8e385266d5c631630c95fec6fb13808e1395de0a))
    - Add KeyScanning trait ([`218896b`](https://github.com/kiibohd/kiibohd-core/commit/218896b335f0b46d7cf9d5430afb8a98feb2c4b7))
    - Fix pwm and scaling for open/short detection on is31fl3743b ([`0ec2103`](https://github.com/kiibohd/kiibohd-core/commit/0ec21033b564b8cb18051c15d36e657e12d9d843))
    - Update is31fl3743b and fix clippy warnings ([`f125eed`](https://github.com/kiibohd/kiibohd-core/commit/f125eed08a1b2d390b7b8d2fa563aeb2d5759b7e))
    - Adding basic version of the Is31fl3743b driver ([`51ee22c`](https://github.com/kiibohd/kiibohd-core/commit/51ee22c8178ed519b533c841b3617deb16d87a3e))
    - Fix clippy warning ([`51050cc`](https://github.com/kiibohd/kiibohd-core/commit/51050cc03238d2eb663fdfff8540e4518eab8471))
    - HID-IO pixel control ([`9f896c7`](https://github.com/kiibohd/kiibohd-core/commit/9f896c734188198c9037d7d2fade4f4a2fda96fc))
    - Add h0021 and h0026 to kiibohd-hid-io ([`2c23d22`](https://github.com/kiibohd/kiibohd-core/commit/2c23d224d2e2639d59c5b20e1eae49422d117e30))
    - Fix hid-io-protocol missing generic ([`f07c269`](https://github.com/kiibohd/kiibohd-core/commit/f07c269adb5ea1392bff83d3c93d1055f104486c))
    - Add better debbuing for i331fl3743b crate ([`6416b1c`](https://github.com/kiibohd/kiibohd-core/commit/6416b1cf07440184ba088a077f59a7414a7fb8eb))
    - Stabilized compiler feature ([`8cd3098`](https://github.com/kiibohd/kiibohd-core/commit/8cd309877aa02639bb7de38a1a46890ad3637d08))
    - [kiibohd-hid-io] Fix rx buffer processing ([`98fef86`](https://github.com/kiibohd/kiibohd-core/commit/98fef86895b8aa78d40d6a0ee8b74d1674511b5b))
    - Usbd-hid now uses defmt instead of defmt-impl feature ([`4039041`](https://github.com/kiibohd/kiibohd-core/commit/4039041f1e79ad10fd87e3c2536da4f4b240feea))
    - [kiibohd-usb] Adding HID Lock LED support ([`ce32c30`](https://github.com/kiibohd/kiibohd-core/commit/ce32c302c003900690c645d70ea2c97e87b370ce))
    - Fix clippy lints ([`6d404e5`](https://github.com/kiibohd/kiibohd-core/commit/6d404e561abd569c609af0e03716bb79e9cdeb24))
    - Simplifying log crate ([`5a8f450`](https://github.com/kiibohd/kiibohd-core/commit/5a8f4505c68c681b773e8cf6e96a62eeaef2c4d3))
    - [kiibohd-usb] Fix remote wakeup and nkro support ([`3aa9f7e`](https://github.com/kiibohd/kiibohd-core/commit/3aa9f7e9273f1d64933f9fe2a0c8c37960cea705))
    - [kll-core] Fix update status position ([`6b0c01d`](https://github.com/kiibohd/kiibohd-core/commit/6b0c01d4b3f452375a94847ced49297d5d27530f))
    - [kiibohd-keyscanning] Add off state ignore option ([`5cd975c`](https://github.com/kiibohd/kiibohd-core/commit/5cd975c07908246fd49f8550ecceec7220e6ae0e))
    - Kiibohd-usb now passes USB compliance HID Tests ([`63a6b3e`](https://github.com/kiibohd/kiibohd-core/commit/63a6b3eebcc1578aa294fc88831b4f0d675fb82f))
    - Increment versions (kll-core, kiibohd-usb) ([`0e9fbf4`](https://github.com/kiibohd/kiibohd-core/commit/0e9fbf40b9f9243f727d80c44a3cae64a4639968))
    - Adding Analog conversion support and fixing kiibohd-usb mouse support ([`4cc97e8`](https://github.com/kiibohd/kiibohd-core/commit/4cc97e8b8302f76ef006032e60ef7b3a2e613da0))
    - Fix missing defmt enable ([`0a3a5f4`](https://github.com/kiibohd/kiibohd-core/commit/0a3a5f48fc753d87ba2bcfe1bc8af845ae73fa5f))
    - Re-enable for git usage ([`fb219cc`](https://github.com/kiibohd/kiibohd-core/commit/fb219cca16bb8f08650d25a0b0291b484700817c))
    - Handling usb-device crate temp issue ([`0a05523`](https://github.com/kiibohd/kiibohd-core/commit/0a055232dd42478aaff72810889c6e0820425f5e))
    - Missing version ([`214e9cb`](https://github.com/kiibohd/kiibohd-core/commit/214e9cbb2dce64f7452af37f9e8b79993870b272))
    - Missing defmt ([`ba6846e`](https://github.com/kiibohd/kiibohd-core/commit/ba6846eda56f153b7f947a76bdddff4b1d1e1fd2))
    - Update defmt configurations ([`58c3aac`](https://github.com/kiibohd/kiibohd-core/commit/58c3aac6996ba72a24c12910e7875ecd2f6be969))
    - More clippy fixes ([`528672a`](https://github.com/kiibohd/kiibohd-core/commit/528672a0f7f255eb95cda7fd5423cfc553fa959e))
    - Increment patch ([`cc4f15f`](https://github.com/kiibohd/kiibohd-core/commit/cc4f15f18096cf75947204eab219c19f3dcaed18))
    - Add binary conversion to TriggerEvent ([`cd00256`](https://github.com/kiibohd/kiibohd-core/commit/cd0025615b4ab207426996b9541a7be78e81e0e8))
    - Update README.md ([`8075ed7`](https://github.com/kiibohd/kiibohd-core/commit/8075ed7527b687f98f1c15f3a9c84a7c24d40f77))
    - Update README.md ([`f38eab7`](https://github.com/kiibohd/kiibohd-core/commit/f38eab7ac896e237a875d1280b276559ec79c641))
    - Update README.md ([`b6915fa`](https://github.com/kiibohd/kiibohd-core/commit/b6915facad7154f5d2f80dd57143eb41fdfd5d33))
    - Update README.md ([`48be84e`](https://github.com/kiibohd/kiibohd-core/commit/48be84ed0ba9513d060e7748200b0b24d80e6798))
    - Update README.md ([`f7d1735`](https://github.com/kiibohd/kiibohd-core/commit/f7d173585b79bf551ea73812d008fe0100a21ca4))
    - Update README.md ([`b08610d`](https://github.com/kiibohd/kiibohd-core/commit/b08610d8d975776f9ad749985d8e8a7616b8559e))
    - Update README.md ([`354dc3c`](https://github.com/kiibohd/kiibohd-core/commit/354dc3ca80838e4e6b6669194216f60493cc3b51))
    - Update README.md ([`d7fe786`](https://github.com/kiibohd/kiibohd-core/commit/d7fe786cb66298bbaf0a8848963193f4216a2bd3))
    - Update README.md ([`3e1af10`](https://github.com/kiibohd/kiibohd-core/commit/3e1af107daa5b5a085403167d0cb2eb2fcf3adf6))
    - Update README.md ([`aeacfb2`](https://github.com/kiibohd/kiibohd-core/commit/aeacfb274fe2b57d410aa63c594af047edccf3f7))
    - Update README.md ([`1228efd`](https://github.com/kiibohd/kiibohd-core/commit/1228efdf73543615fbcf1ffc715e517283a767c5))
    - Update README.md ([`784d325`](https://github.com/kiibohd/kiibohd-core/commit/784d3259a3e798a5fad642189736de9f95e7dd98))
    - Is31fl3743b README.md ([`ee0eefe`](https://github.com/kiibohd/kiibohd-core/commit/ee0eefe1d154d6491afdd474fa4f8e4ad53880c2))
    - Fix clippy warnings ([`acba465`](https://github.com/kiibohd/kiibohd-core/commit/acba4651a0d349b981889fe9debd202ad96f1d97))
    - Add kll-core support to kiibohd-hall-effect-keyscanning ([`d0a5c83`](https://github.com/kiibohd/kiibohd-core/commit/d0a5c8376f3b17bf3e3418e5466d095295d5137f))
    - Fix typo ([`4ba9592`](https://github.com/kiibohd/kiibohd-core/commit/4ba95923178cd5755433d3314650882e57baa5d7))
    - Adding no-std keywords ([`59254c5`](https://github.com/kiibohd/kiibohd-core/commit/59254c5018132cb379790e6e0df6dc02f75b7c0f))
    - Adding process_off_state_lookups ([`babf695`](https://github.com/kiibohd/kiibohd-core/commit/babf695a81c0f31a5445ace0cdc383caa1eea873))
    - Cargo fmt ([`c37456d`](https://github.com/kiibohd/kiibohd-core/commit/c37456d7bfb1f032a0947e4aeb19ea24761e8e7a))
    - Support custom crates.io packages for usb ([`59b8e0f`](https://github.com/kiibohd/kiibohd-core/commit/59b8e0f43f10021c1758b8f44b224bd4be008e31))
    - Set versions for kiibohd-usb ([`33999e3`](https://github.com/kiibohd/kiibohd-core/commit/33999e3e2468d881d89ce4a035369bf4dacfdbd0))
    - Handle compilation error for missing match ([`f28bbb7`](https://github.com/kiibohd/kiibohd-core/commit/f28bbb71d6c41529cdde001afb955f4007e76240))
    - Updating Cargo.toml files to publish initial crates ([`e18dafb`](https://github.com/kiibohd/kiibohd-core/commit/e18dafb3802406146f6f70b522418d1139cec09c))
    - Adding README.md for kll-macros ([`603de2f`](https://github.com/kiibohd/kiibohd-core/commit/603de2f8172c09bb47ab1e038299a97bf79c4e4c))
    - Adding README.md for kll-core ([`8dfd29e`](https://github.com/kiibohd/kiibohd-core/commit/8dfd29efde09e92d4ec178f52374136d7239598d))
    - Adding README for kll-hid ([`c346a26`](https://github.com/kiibohd/kiibohd-core/commit/c346a26508814c336b7fd2970d4ce54f18ccc184))
    - Add Off-state event generated (generate_event) ([`310b013`](https://github.com/kiibohd/kiibohd-core/commit/310b013360a8a46636c756aae2d9da5b9bcad4fb))
    - Add enqueue_ functions for kiibohd-usb ([`bc989f9`](https://github.com/kiibohd/kiibohd-core/commit/bc989f9c81098047396de4c49f13034df9fd9c88))
    - Adding kll-core KeyEvent to TriggerEvent conversion ([`eb54635`](https://github.com/kiibohd/kiibohd-core/commit/eb54635c7ae2735dc9660fc08a668bb11f9bc2a6))
    - Adding state() lookup to kiibohd-keyscanning ([`0ebd4d1`](https://github.com/kiibohd/kiibohd-core/commit/0ebd4d14ef797db38d479bba41f5e2fb0c705d67))
    - U8 to u16 typo ([`b936e79`](https://github.com/kiibohd/kiibohd-core/commit/b936e796f14be4a670467d987ab687ec10ff1db9))
    - Resolve no_std compilation issues due to log ([`6f7df7c`](https://github.com/kiibohd/kiibohd-core/commit/6f7df7c1e830dec3d2138055c6c447054aba753e))
    - Convert kll-core validation test to a generic struct ([`3d06f99`](https://github.com/kiibohd/kiibohd-core/commit/3d06f990ec94655fb95b94323011197ee4d37894))
    - Initial generic kll -> kll-core validation test ([`0aa8806`](https://github.com/kiibohd/kiibohd-core/commit/0aa8806e5cfb9b811a2958c1b590a3e0d4f4bdfe))
    - Initial working kll-compiler -> kll-core flow ([`4a21b5a`](https://github.com/kiibohd/kiibohd-core/commit/4a21b5a2e5f1c2ffc9048975cc8948bc00fce663))
    - Initial HidIoEvent handler ([`8019a12`](https://github.com/kiibohd/kiibohd-core/commit/8019a12f4d953470635f58dc54697e9bdb56bb11))
    - P-Channel MOSFETs are inverted ([`4bcd578`](https://github.com/kiibohd/kiibohd-core/commit/4bcd57804bb0ecd5a4bfd0c4e6dcd95467d68e8c))
    - More error messages and handle None results as no-op ([`2afde29`](https://github.com/kiibohd/kiibohd-core/commit/2afde298400604783a843181a24f3d0fa1bd01bf))
    - Fixing clippy warning when using generated code ([`4a62796`](https://github.com/kiibohd/kiibohd-core/commit/4a627969c9d7ca56faa853b8d785eb6bb963df1a))
    - Add error detection on layer-rs lookups ([`340527a`](https://github.com/kiibohd/kiibohd-core/commit/340527ae4e97396ce0e6defae451a380bf54c985))
    - Upgrading GitHub Actions checkout to v3 ([`2658377`](https://github.com/kiibohd/kiibohd-core/commit/265837790a1843b69589f72ab646055d4adf4997))
    - Adding layout support to kllcore emitter ([`9fa3cac`](https://github.com/kiibohd/kiibohd-core/commit/9fa3cacef661d3e1688fb20f113adc38f383bfc7))
    - Initial version of generate_state_scheduling ([`80461c8`](https://github.com/kiibohd/kiibohd-core/commit/80461c861e61a08835af9b29158c96e960890725))
    - Fixing power of 2 issues with heapless::Vec ([`8cce7c2`](https://github.com/kiibohd/kiibohd-core/commit/8cce7c29199561a1051c42a9c195fa577a335ee6))
    - [kll-compiler] - Adding initial implied_state functionality ([`586fa9e`](https://github.com/kiibohd/kiibohd-core/commit/586fa9ea3a41cee17c4e819633f9cd97781c20cb))
    - Add layers() to KllGroups ([`1f15cf9`](https://github.com/kiibohd/kiibohd-core/commit/1f15cf9763ae4e1b4bedc25193f5cf1daaf6ee70))
    - Initial rust code generation for kll-core ([`ec6412a`](https://github.com/kiibohd/kiibohd-core/commit/ec6412a94667bc3c815e8c279b5b399bb024723b))
    - Initial kll-core integration ([`3a5940f`](https://github.com/kiibohd/kiibohd-core/commit/3a5940fbe1a1445daa5b336b0f3041927cc9833f))
    - Doc typo ([`112c9a6`](https://github.com/kiibohd/kiibohd-core/commit/112c9a6fc1f2a96f1dae9bba7833b2fcb412973f))
    - Initial IS31FL3743B support for atsam4 pdc ([`9674dc7`](https://github.com/kiibohd/kiibohd-core/commit/9674dc7410b51b0cc13a5a52118f3bf2e4651e7a))
    - Updating to defmt 0.3 ([`831f49e`](https://github.com/kiibohd/kiibohd-core/commit/831f49e1e4d8a3026417544604208a1b4a8243a1))
    - Cargo fmt typo ([`5e6998d`](https://github.com/kiibohd/kiibohd-core/commit/5e6998def3dc0ac05f78534a5f0fc83105f9d7e4))
    - Ignoring clippy warning ([`069c776`](https://github.com/kiibohd/kiibohd-core/commit/069c776aeeb304fd749b61d0c78460fb89831676))
    - Adding temporary GitHub Action integratino for kll-compiler ([`638f25c`](https://github.com/kiibohd/kiibohd-core/commit/638f25ce6845337d1914f30e17c41c6737801873))
    - Disabling broken tests ([`d562073`](https://github.com/kiibohd/kiibohd-core/commit/d56207355564662045dbe0c284151483738b4967))
    - Fixing build and clippy warnings ([`6e6788f`](https://github.com/kiibohd/kiibohd-core/commit/6e6788fdb57ffefb630da14c79b6c015a908bf3e))
    - Renaming project to kll-compiler ([`7a66956`](https://github.com/kiibohd/kiibohd-core/commit/7a6695641ebfc5fba4b6406489a1c6c58797ba58))
    - Reduce triggers, map results to capabilities ([`03c45ef`](https://github.com/kiibohd/kiibohd-core/commit/03c45efdd01db7832cfb5cdc6aacf11fc253d952))
    - Merge all config/base/default/partial kll files ([`b5c937f`](https://github.com/kiibohd/kiibohd-core/commit/b5c937f6e197f2fe64eb4420cb18e11f91e65b80))
    - Add command line args ([`081db66`](https://github.com/kiibohd/kiibohd-core/commit/081db66334d3ffe1c48367997d0713505921ee18))
    - Add kiibohd emitter ([`8118dd1`](https://github.com/kiibohd/kiibohd-core/commit/8118dd10daa801a086b223187aeb0c219714724a))
    - Rework trigger/results ([`4e25f7e`](https://github.com/kiibohd/kiibohd-core/commit/4e25f7ec3bf2cb79f96061c6f2ab5d5aa41cfb71))
    - Chip away at unhandled AST conditions ([`be2c16f`](https://github.com/kiibohd/kiibohd-core/commit/be2c16f6357650bab4a80af634ac3fe8d2ad160c))
    - Successfully parse all examples ([`fab59e1`](https://github.com/kiibohd/kiibohd-core/commit/fab59e1d5025d075d6e0310076e4c0a3db066691))
    - Add basic rust emitters ([`b435929`](https://github.com/kiibohd/kiibohd-core/commit/b4359298691abfe12dee8e81265aad873ac191ba))
    - Refactor variables ([`82c7c85`](https://github.com/kiibohd/kiibohd-core/commit/82c7c8572d46a9a6abde361da37b1b7ab1b3b52a))
    - Add more complex examples ([`faa2f9c`](https://github.com/kiibohd/kiibohd-core/commit/faa2f9c611cdbba5d505e3c5e26cb6d5ed766138))
    - Add more tests ([`0f26079`](https://github.com/kiibohd/kiibohd-core/commit/0f2607902a1de25760353f101ebb612a9827435e))
    - Make state public ([`a671049`](https://github.com/kiibohd/kiibohd-core/commit/a671049e47901d28c7ec0e6f4aaa0e168adcfb32))
    - Split into sub files ([`bdb1e9c`](https://github.com/kiibohd/kiibohd-core/commit/bdb1e9c2e48f233d7acd7735d8ca92fcf4e27747))
    - Move everything to pest_consume ([`d6de380`](https://github.com/kiibohd/kiibohd-core/commit/d6de3805fc7125dde764c6132d53b62539bb1e70))
    - Split parsing to functions ([`397246d`](https://github.com/kiibohd/kiibohd-core/commit/397246d3bd27a5a8ac56a8eba5c6f61af0b19117))
    - Add display trait to everything ([`44d181d`](https://github.com/kiibohd/kiibohd-core/commit/44d181dde99be9347198b41c089c71dbab765516))
    - Parse the rest of the triggers and animation internals ([`3339da0`](https://github.com/kiibohd/kiibohd-core/commit/3339da07c7a54014bf4c1b1fcb340adfe8a75f5e))
    - Parse strings into sub components ([`71ca630`](https://github.com/kiibohd/kiibohd-core/commit/71ca630bb529ee48fdeaa7d7bf416e06a4a90fcf))
    - Basic kll parser ([`06ceae2`](https://github.com/kiibohd/kiibohd-core/commit/06ceae29151e7c00c96fb45bb652b0cd5a9ea57f))
    - Update README.md ([`ebe7f5f`](https://github.com/kiibohd/kiibohd-core/commit/ebe7f5faca1f75c3897e974e39d708d17850ac2d))
    - Moving top-level kiibohd-core to kiibohd-core-ffi ([`80eb182`](https://github.com/kiibohd/kiibohd-core/commit/80eb1824e2dad168ad37bd7aa48b4c874dea2c22))
    - Update README.md ([`4f75f08`](https://github.com/kiibohd/kiibohd-core/commit/4f75f088c6c772f64d2ba963e3da36e1f086db80))
    - Upating to 2021 edition ([`ea8ed92`](https://github.com/kiibohd/kiibohd-core/commit/ea8ed9259590c31456b11eba01abdd4a8138bf32))
    - Fixing multiplication overflow panic ([`985c72d`](https://github.com/kiibohd/kiibohd-core/commit/985c72dc69e8861566bc705e3ec9ee5f3e856d37))
    - Cargo fmt ([`64995b8`](https://github.com/kiibohd/kiibohd-core/commit/64995b8459bf1027d8171d57e7fb9f2c75ce33f8))
    - Added missing column size constant to timing calculations ([`70e8597`](https://github.com/kiibohd/kiibohd-core/commit/70e85978a85b1bafdfb125f815ed13798b07f874))
    - Updating kiibohd-keyscanning ([`1c51025`](https://github.com/kiibohd/kiibohd-core/commit/1c51025e8568e4e00571527b87a3ea8d20c251c8))
    - Fixing cargo fmt and clippy warnings ([`edcf4db`](https://github.com/kiibohd/kiibohd-core/commit/edcf4db1f62129b6f48a477e08883eb24ec4c057))
    - Small fixes ([`1ac32f2`](https://github.com/kiibohd/kiibohd-core/commit/1ac32f20649e8f6ded05af03606ff4a0793c3a9c))
    - Refactored kiibohd-keyscanning module ([`999bf4d`](https://github.com/kiibohd/kiibohd-core/commit/999bf4d7d14cee85ca1351df67cfef805f23bda2))
    - Initial skeleton of kll-core implementation ([`025dcea`](https://github.com/kiibohd/kiibohd-core/commit/025dceaa4c3e311de4ab34679b1f7fa0a2a1f84e))
    - Updating to new usbd-hid new_ep_in_with_settings() api ([`7f1fd76`](https://github.com/kiibohd/kiibohd-core/commit/7f1fd762c19964fe50835cb462220d0ad3098039))
    - Adding defmt support to kiibohd-usb ([`d941980`](https://github.com/kiibohd/kiibohd-core/commit/d941980ff0ab56009ec794c2783ebc186882369c))
    - Fixing clippy warnings ([`8c29227`](https://github.com/kiibohd/kiibohd-core/commit/8c2922788e68b9def3ea7174a41f962927f06191))
    - Enabling defmt support in hid-io-protocol ([`022cb11`](https://github.com/kiibohd/kiibohd-core/commit/022cb1157deebfb037fac0118ddba6063e52e482))
    - Fixing cargo fmt typo ([`0f8f032`](https://github.com/kiibohd/kiibohd-core/commit/0f8f032216a3c4294a920b092114ed56b6cde764))
    - Adding basic kiibohd-hall-effect-keyscanning crate ([`78607a0`](https://github.com/kiibohd/kiibohd-core/commit/78607a0b7e3c5f1d2f915eb18f47d77ca207fa93))
    - Cleanup cargo fmt ([`764b0ae`](https://github.com/kiibohd/kiibohd-core/commit/764b0ae9b37c08d3201e64096719e8529387ef0d))
    - Typo ([`ea653aa`](https://github.com/kiibohd/kiibohd-core/commit/ea653aa561099a2638336ca53288ea5f26e9aeef))
    - Splitting hid-io into rust and ffi versions ([`5746c10`](https://github.com/kiibohd/kiibohd-core/commit/5746c1015242c5cf21d603da1f7220bcb06c64a0))
    - Fixing typo in DWT ([`a6c1488`](https://github.com/kiibohd/kiibohd-core/commit/a6c148871d2565c1108a1500314a3763d2b5f206))
    - Upgrading to heapless 0.7 ([`167a127`](https://github.com/kiibohd/kiibohd-core/commit/167a127642ef371d6d9d9a644a3f63816a408f14))
    - Add missing README.md for kiibohd-usb ([`75c89e5`](https://github.com/kiibohd/kiibohd-core/commit/75c89e5151fc067ad127d27bce537d524935f497))
    - Updates to kiibohd-log and kiibohd-usb ([`231fccb`](https://github.com/kiibohd/kiibohd-core/commit/231fccb8df2732bdfab30ed92faa956ec1ecfe17))
    - Small touch-ups to kiibohd-log and kiibohd-usb ([`7faf2b9`](https://github.com/kiibohd/kiibohd-core/commit/7faf2b9cb92c292c4dfd656e7346aa040507159d))
    - Adding initial kiibohd-log and kiibohd-usb ([`547cd8e`](https://github.com/kiibohd/kiibohd-core/commit/547cd8e15da8d664c68f2af899b0bbacb5037eb1))
    - Updating license to MIT+Apache 2.0 ([`1497fd7`](https://github.com/kiibohd/kiibohd-core/commit/1497fd7d369e16f6f4cc7590e903661c5bc47026))
    - Updating to use two separate analysis modes ([`a72fc64`](https://github.com/kiibohd/kiibohd-core/commit/a72fc64b56cd7547e0ccbd994e92f40c550447d6))
    - Adding two run modes: Calibration and Normal ([`a2e1478`](https://github.com/kiibohd/kiibohd-core/commit/a2e1478d475afd982ad92428850c82e4e9d7d6f8))
    - Adding missing DeviceVersion field ([`86747b4`](https://github.com/kiibohd/kiibohd-core/commit/86747b4d242a8cec51217516eb1b8d65bc8befc3))
    - Adding missing file ([`325c7e1`](https://github.com/kiibohd/kiibohd-core/commit/325c7e10f045e3bb30ca3b8b8251e6ea6e1cfceb))
    - Finished adding basic kiibohd-hall-effect unit tests ([`ffe1cc2`](https://github.com/kiibohd/kiibohd-core/commit/ffe1cc230e259c1e7f8b240985395a6d72076cda))
    - Moving hid-io-kiibohd and initial version of kiibohd-hall-effect ([`d1a109a`](https://github.com/kiibohd/kiibohd-core/commit/d1a109afca0a84cf6943b3234db917c1862ba571))
    - Initial commit ([`52ad09e`](https://github.com/kiibohd/kiibohd-core/commit/52ad09e601b3f6c9207ba1bef1a65eb8176b7886))
</details>

