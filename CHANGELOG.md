# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.4.2](https://github.com/framesurge/perseus/compare/v0.4.1...v0.4.2) (2023-05-17)


### Bug Fixes

* critical typo in cli arch scanning ([#291](https://github.com/framesurge/perseus/issues/291)) ([dfe92ba](https://github.com/framesurge/perseus/commit/dfe92ba4fefa80788866b2a9e4633403db553154))


### Documentation Changes

* add missing links to Cranelift ([#273](https://github.com/framesurge/perseus/issues/273)) ([df922df](https://github.com/framesurge/perseus/commit/df922df1f6001a8a5e5012c234a9ed52ad2ea334))

### [0.4.1](https://github.com/framesurge/perseus/compare/v0.4.0...v0.4.1) (2023-05-12)


### Bug Fixes

* **cli:** fixed aarch64 downloads on m2 macs ([#286](https://github.com/framesurge/perseus/issues/286)) ([cbc6faf](https://github.com/framesurge/perseus/commit/cbc6fafad9b8ee3d5437b23e07a192eb7f1bb6f5))
* prevented global state caching on engine-side ([23e6deb](https://github.com/framesurge/perseus/commit/23e6deb813c33089d884f9746205a2824bdd522f)), closes [#280](https://github.com/framesurge/perseus/issues/280)
* removed `dbg!` statement ([9f885a9](https://github.com/framesurge/perseus/commit/9f885a9331a9f197eb342bb57875fd30a43a23c3))


### Documentation Changes

* added `--locked` flag for v0.3.x installation ([#274](https://github.com/framesurge/perseus/issues/274)) ([2708508](https://github.com/framesurge/perseus/commit/27085082a39e902a57f5747d2c4b084edf1da3bc))
* fixed several typos ([#272](https://github.com/framesurge/perseus/issues/272)) ([2fb434b](https://github.com/framesurge/perseus/commit/2fb434bb52eb422bde8db5ffa649d4ad94a8432e))
* fixed typo ([#271](https://github.com/framesurge/perseus/issues/271)) ([4993025](https://github.com/framesurge/perseus/commit/49930255c8551e7340623cbaca9931cbd0d2f79b))
* updated all references to v0.4 being in beta ([6763c0c](https://github.com/framesurge/perseus/commit/6763c0c347e0379552fc698a0ed84d48d04ab93d))

## [0.4.0](https://github.com/framesurge/perseus/compare/v0.4.0-rc.1...v0.4.0) (2023-04-09)


### Bug Fixes

* fixed semantically invalid html in website and added faq about it ([e69431e](https://github.com/framesurge/perseus/commit/e69431ef921735c771f50f5c5f8e193b8eef7149))


### Documentation Changes

* updated docs to v0.4.0 ([5512a30](https://github.com/framesurge/perseus/commit/5512a30eabb875eb5a4d5d11c8bbf3e6747f6b75))

## [0.4.0-rc.1](https://github.com/framesurge/perseus/compare/v0.4.0-beta.22...v0.4.0-rc.1) (2023-04-09)


### ⚠ BREAKING CHANGES

* removed localized global state (global state functions
should no longer take a locale)

* fix: transmitted global state in locale redirection pages

### Features

* added rocket integration ([#266](https://github.com/framesurge/perseus/issues/266)) ([25bd115](https://github.com/framesurge/perseus/commit/25bd115d690d6dd8421a44b80837af0d7e92c993))
* added support for broader error types across state generation ([9aa079b](https://github.com/framesurge/perseus/commit/9aa079b1f74627c4f76927c9d3d25f11316a941e)), closes [#264](https://github.com/framesurge/perseus/issues/264)
* remove localized global state ([#268](https://github.com/framesurge/perseus/issues/268)) ([2017f69](https://github.com/framesurge/perseus/commit/2017f69a978fc0d3946d38e8c27ec7b0e0e03453)), closes [#267](https://github.com/framesurge/perseus/issues/267)


### Bug Fixes

* cleaned up rocket integration ([5c8e50d](https://github.com/framesurge/perseus/commit/5c8e50d17467b63d61ccf7d9138e523b9c4a271b))
* **cli:** fixed help tests ([0800311](https://github.com/framesurge/perseus/commit/0800311f5b00defd1963eb81425442304c46763d))
* fixed bad version in rocket integration ([6bd2ca0](https://github.com/framesurge/perseus/commit/6bd2ca0a05fe3444a2b79c59a953cda97ffdd8b1))

## [0.4.0-beta.22](https://github.com/framesurge/perseus/compare/v0.4.0-beta.21...v0.4.0-beta.22) (2023-03-12)


### Features

* support `anyhow::Error` in state generation functions ([#264](https://github.com/framesurge/perseus/issues/264)) ([53ad2ad](https://github.com/framesurge/perseus/commit/53ad2ad3a30ffe83645386a8802358ffe9322913))


### Bug Fixes

* updated `cargo_toml` to v0.15 ([68ef993](https://github.com/framesurge/perseus/commit/68ef9935bae8bdc3c01d83b236ca2838dfe5b1c1))


### Documentation Changes

* added docs on using perseus in a workspace ([b0653dc](https://github.com/framesurge/perseus/commit/b0653dccad8b7e0a12b0501f16b2c0065226f263))

## [0.4.0-beta.21](https://github.com/framesurge/perseus/compare/v0.4.0-beta.20...v0.4.0-beta.21) (2023-03-01)


### Features

* **i18n:** added support for auto-setting the `lang` attribute ([1e3206c](https://github.com/framesurge/perseus/commit/1e3206c761a95c321d201d57514b84f4a1a0d250)), closes [#261](https://github.com/framesurge/perseus/issues/261)


### Bug Fixes

* fixed incremental generation on index template ([b3b3b82](https://github.com/framesurge/perseus/commit/b3b3b82338132805c73dea02399ab5edaaf0d8a4)), closes [#262](https://github.com/framesurge/perseus/issues/262)

## [0.4.0-beta.20](https://github.com/framesurge/perseus/compare/v0.4.0-beta.19...v0.4.0-beta.20) (2023-02-26)


### Bug Fixes

* **cli:** fixed cli tests to test bleeding-edge core ([27442d5](https://github.com/framesurge/perseus/commit/27442d50db95b2919297631d775ebcb86a34b90f)), closes [#259](https://github.com/framesurge/perseus/issues/259)
* fixed crash handling system ([b085888](https://github.com/framesurge/perseus/commit/b085888bc504bfb228d35a5db8f28f870e41f29f))
* fixed hsr ignore deployment issues ([#260](https://github.com/framesurge/perseus/issues/260)) ([e9c6239](https://github.com/framesurge/perseus/commit/e9c62391761583a63d779d6fc9087cf4b8b85eba))
* fixed nested static aliases in exporting ([c15624e](https://github.com/framesurge/perseus/commit/c15624e1d4a8bdfc957409adc2c8fb4eee8249e8)), closes [#258](https://github.com/framesurge/perseus/issues/258)
* fixed static content tests ([88cad92](https://github.com/framesurge/perseus/commit/88cad92229c58eed1a3d31e572e0fc962f7f50f0))
* removed unused import ([febdbaf](https://github.com/framesurge/perseus/commit/febdbaf9080eba7e504461dfff00e73d3313f24e))

## [0.4.0-beta.19](https://github.com/framesurge/perseus/compare/v0.4.0-beta.18...v0.4.0-beta.19) (2023-02-18)


### ⚠ BREAKING CHANGES

* implicit `new` method removed from intermediate
reactive types

### Features

* added hsr ignoring feature applied by default to unreactive state ([6768f4e](https://github.com/framesurge/perseus/commit/6768f4e05dbd0147205d78cb469a988c6d1e545b))


### Bug Fixes

* fixed two broken tests ([ed5b5ad](https://github.com/framesurge/perseus/commit/ed5b5adc3d6770861cc16e7ad0cf31f68e09dfab))


### revert

* removed `new` implementation on reactive types ([60a60f0](https://github.com/framesurge/perseus/commit/60a60f09b49c849bcf4ea50ab6e38679dd1c3efe))


### Documentation Changes

* added docs on rx_collection iteration ([d297af1](https://github.com/framesurge/perseus/commit/d297af1ba885376eb93ff4cb2259246a0819da10))

## [0.4.0-beta.18](https://github.com/framesurge/perseus/compare/v0.4.0-beta.17...v0.4.0-beta.18) (2023-02-10)


### ⚠ BREAKING CHANGES

* removed `RouterLoadState::ErrorLoaded` entirely (you
should now rely solely on error views, which will be able to poll the
last router state, which probably contributed to/caused the error)

### Features

* added automatic conversion methods for reactive types ([#252](https://github.com/framesurge/perseus/issues/252)) ([bb36a4c](https://github.com/framesurge/perseus/commit/bb36a4ccb1defaf9ff0ab69d6191917a5443fab0))
* **cli:** added `--verbose` for easier logging ([10cd48b](https://github.com/framesurge/perseus/commit/10cd48b19efbf2887399787877c239c19f2fa93e))
* **cli:** added `.cargo/config.toml` to default `init`/`new` templates ([a8afc7d](https://github.com/framesurge/perseus/commit/a8afc7d95ad1759055513ff828e3513947efc357))
* **cli:** added support for eatch exclusions with `--custom-watch !my-path` ([6f57d4a](https://github.com/framesurge/perseus/commit/6f57d4a9f94d51ecb44f44b50424605f83f4f1a1))
* greatly improved default error views ([2199da6](https://github.com/framesurge/perseus/commit/2199da6df89536dc72de94983060aca848903b7b))


### Bug Fixes

* added `target/` to generated `.gitignore` ([#254](https://github.com/framesurge/perseus/issues/254)) ([9b7b99d](https://github.com/framesurge/perseus/commit/9b7b99dea45f653a17f2c69057904125648a7d81))
* **cli:** fixed deployment with revalidation ([#256](https://github.com/framesurge/perseus/issues/256)) ([89e864a](https://github.com/framesurge/perseus/commit/89e864a0ebcebb5dc6792f95d5aa066eea3944d0))
* **error_views:** fixed positioning of default error views ([efcfec5](https://github.com/framesurge/perseus/commit/efcfec5129aafee994f32395679bdb1273b08453))
* fixed bad inequality in default error views ([b485a30](https://github.com/framesurge/perseus/commit/b485a300a1e4a31391d5030e6510bcd248881720))
* fixed cause of perseus hydration bugs ([6d38da6](https://github.com/framesurge/perseus/commit/6d38da68d3c226da932fa98922da38765dbb7c4f))
* fixed hanging servers on test errors ([2ad8bea](https://github.com/framesurge/perseus/commit/2ad8bea273fc5a8846f4ec788afcda01573e24be))
* fixed outdated macro reference in compile error ([c5dc4a6](https://github.com/framesurge/perseus/commit/c5dc4a68cd8b8e69627ac693cc98ec15616330bf))
* fixed panic handler issues ([07ab6c6](https://github.com/framesurge/perseus/commit/07ab6c6ef6897a7d3c7fb984b6415c45917e2f69))
* **perseus-axum:** fixed snippet hosting ([5f54722](https://github.com/framesurge/perseus/commit/5f54722403823e972f143d6bdbe27b94d7cd40f6))
* re-enabled minification ([9512df0](https://github.com/framesurge/perseus/commit/9512df0a94b2244c4893c90859ce6d72cd5e971f))
* removed debug message ([803123b](https://github.com/framesurge/perseus/commit/803123b68fc310b3fd7367e9af52678a3007088f))


### Code Refactorings

* simplified error router states ([6252f6a](https://github.com/framesurge/perseus/commit/6252f6a4804c784e2020f31ccb32728af9275852))
* split testing scripts into `core` and `cli` ([0a2d2df](https://github.com/framesurge/perseus/commit/0a2d2dfacd2409d4c030a882011844533483699b))
* **tests:** unified testing infrastructure for `perseus-cli` ([5b18efd](https://github.com/framesurge/perseus/commit/5b18efd3e2c83f521f1a5c745848d3daec8387ca))


### Documentation Changes

* added all missing links to the docs ([c3c575b](https://github.com/framesurge/perseus/commit/c3c575b9a7cb6e2582b7581f12e13ded23526f42))
* added docs on watch exclusions ([8f9936a](https://github.com/framesurge/perseus/commit/8f9936ae268eed3918f5c15c8c261da2231f56ab))
* added note on `snoop` to dev cycle docs ([fa62ebe](https://github.com/framesurge/perseus/commit/fa62ebefecdf92c64c863c0a391803305c1037cb))
* added note on hanging servers from testing ([ade5137](https://github.com/framesurge/perseus/commit/ade5137d3eeb7a213524b16ec8176c181a256058))
* covered `Cargo.toml` in more depth ([adc3c34](https://github.com/framesurge/perseus/commit/adc3c34f88c676672865d5ed3b2e92e59128191e))
* fixed broken link ([2d28862](https://github.com/framesurge/perseus/commit/2d288622ff7e3476820a2dc06f16beb543f82505))
* removed outdated warnings about styling quirks ([a8704a2](https://github.com/framesurge/perseus/commit/a8704a277946d49d25f855eb84fa7a722722b850))
* wrote docs on capsules v components ([fb8563a](https://github.com/framesurge/perseus/commit/fb8563a8d4a4ee411bac6a5367c52a9dc5dcddb3))

## [0.4.0-beta.17](https://github.com/framesurge/perseus/compare/v0.4.0-beta.16...v0.4.0-beta.17) (2023-01-18)


### Features

* **cli:** added automatic js minification on deployment ([d516f0a](https://github.com/framesurge/perseus/commit/d516f0a97107e8ea0ef1ffee3e617eea7f2404dd))
* implemented `PartialEq + Eq` on all rx collections ([56ee370](https://github.com/framesurge/perseus/commit/56ee370aafeccd35227019cbe6b377dbdf76f274))


### Bug Fixes

* **capsules:** fixed capsules with exporting ([b6e0916](https://github.com/framesurge/perseus/commit/b6e091674c440338193050b18d99fef93812c253))
* **capsules:** fixed further issues with capsule exporting ([e9c5f4d](https://github.com/framesurge/perseus/commit/e9c5f4d92655655a0efec04c397282f0fd860154))
* **capsules:** fixed initial widget caching with i18n ([e3b49e1](https://github.com/framesurge/perseus/commit/e3b49e1ecc4dd7b9780a8e321ca1c44e2303ab9b))
* fixed bad delaayed widgets transmute ([0701ff8](https://github.com/framesurge/perseus/commit/0701ff820d3d9d8961ce25c2c4a023e613f21def))
* **hsr:** fixed hsr ([a897b4e](https://github.com/framesurge/perseus/commit/a897b4eda9b545a01d8ca2b78080fbdc40b655ab))
* made `ThawPrefs` locale-agnostic ([108b754](https://github.com/framesurge/perseus/commit/108b754f32e0b00fa373b37fa1d4f07cec6f2515))
* removed premature freezing API change ([2a9bfde](https://github.com/framesurge/perseus/commit/2a9bfde7b12db058ed20f8efa42d8291f22fe7c3))


### Documentation Changes

* added clear example of `.capsule_ref()` ([f23fcfa](https://github.com/framesurge/perseus/commit/f23fcfa48ccf6e59a221ac45d20e069598d80cc1))
* added notes on nested state and rx collections ([a4c1dfb](https://github.com/framesurge/perseus/commit/a4c1dfbd6489e58285b12d7ddb8c80c024fbd7e5))
* added some missing lang declarations ([c3f2a02](https://github.com/framesurge/perseus/commit/c3f2a02712b90769969cab14de2e428a6ce3d117))
* edited core principles and introductory docs ([7dd032f](https://github.com/framesurge/perseus/commit/7dd032f28625b0d9f242dae17ec216df4ad6d8fb))
* edited migration guide ([96d895d](https://github.com/framesurge/perseus/commit/96d895d93e7bb9b5ba93daa0c66bd4a984282661))
* fixed missing codeblock lang declarations ([962a9fb](https://github.com/framesurge/perseus/commit/962a9fbd3467af55b3ba2c8acaa12cb0530a1229))
* removed old docs and refactored substantially ([0c8a9be](https://github.com/framesurge/perseus/commit/0c8a9bef05bb70ef375f47b80b3261fb60b6156f))
* tempered a line in the migration docs ([ea3d76c](https://github.com/framesurge/perseus/commit/ea3d76c20f31eaa2af5201af9e0060ac334d49d7))
* updated faqs ([0b5fecb](https://github.com/framesurge/perseus/commit/0b5fecbdcc407a66511cb62d024cab1a5c83aa41))
* wrote further docs ([9b7fd5d](https://github.com/framesurge/perseus/commit/9b7fd5dd5f25e122cdb2f4db937502241058d4ca))
* wrote introductory capsules docs ([0b48598](https://github.com/framesurge/perseus/commit/0b48598d06e0e16252f55f75df7447b5ff848c0b))
* wrote new docs on manually implementing state types ([e430cf6](https://github.com/framesurge/perseus/commit/e430cf694e8b5158ff4d9a88c654368b9e393fc3))

## [0.4.0-beta.16](https://github.com/framesurge/perseus/compare/v0.4.0-beta.15...v0.4.0-beta.16) (2023-01-13)


### Bug Fixes

* fixed `#[main_export]` ([f4d6821](https://github.com/framesurge/perseus/commit/f4d682140d9f6b920d511c5765a4fb7e36f16029))
* fixed docs.rs build flags ([41577bb](https://github.com/framesurge/perseus/commit/41577bb70562e03b28abb3073ebd1fc9aa2448a3))

## [0.4.0-beta.15](https://github.com/framesurge/perseus/compare/v0.4.0-beta.14...v0.4.0-beta.15) (2023-01-13)


### ⚠ BREAKING CHANGES

* removed `FetchError::PreloadNotFound`
* **i18n:** `cx` now comes first in `t!` and `link!`
invocations (following Sycamore convention)

### Features

* added simple error views example to basic example ([520eebf](https://github.com/framesurge/perseus/commit/520eebfe13c108f7aa5bf806d170e4ba497a9fda))
* **cli:** added support for `cargo` colors ([513b1cf](https://github.com/framesurge/perseus/commit/513b1cf94cb7a1a849af088ba9e34f035b6ce474))
* **cli:** added support for watching `snoop` and `build` commands ([c9808fb](https://github.com/framesurge/perseus/commit/c9808fbe6ccee1412b13509d3de186cd8ceaeb0c))
* **cli:** made `perseus test` actually run tests ([bc7582a](https://github.com/framesurge/perseus/commit/bc7582a2f63d599df34ff33354b12aa313acf961))
* **docs:** made `perseus` client-side documentable ([bfe34c5](https://github.com/framesurge/perseus/commit/bfe34c5c79a9530b4797824ce0666eeada7aa468))
* **i18n:** created `switch_locale` function on reactor ([6bbd3cb](https://github.com/framesurge/perseus/commit/6bbd3cbe61e7377382400532e2317f22ac2139d4))
* made rx collections support extension ([7185ef6](https://github.com/framesurge/perseus/commit/7185ef6ba60685f26ae1de38a1bc64be52c2a530))


### Bug Fixes

* **cli:** fixed serve/test numbering issues ([99e13be](https://github.com/framesurge/perseus/commit/99e13be56b6c630c0813732252b6785b0899c97b))
* **docs.rs:** made client-side items documentable on docs.rs ([9420cb7](https://github.com/framesurge/perseus/commit/9420cb714765faee66db8c8def6967004fb454cf))
* **docs:** fixed broken docs links ([ee56bc4](https://github.com/framesurge/perseus/commit/ee56bc47f84a88275939665a1e07dc2ced824f53))
* **docs:** made documentation work fully for `#[cfg(client)]` items ([f22f05d](https://github.com/framesurge/perseus/commit/f22f05d7ef3fceddf1b884156978d15703a8597c))
* fixed `.base` example ([cbc74b6](https://github.com/framesurge/perseus/commit/cbc74b617dc1f482a55b44926d30ec3925f06052))
* **i18n:** fixed `t!` macro with multiple interpolations ([f2aa96e](https://github.com/framesurge/perseus/commit/f2aa96e7b7282a01e7cd35a585068199f9181e82))
* **website:** fixed scrolling issues ([7c63355](https://github.com/framesurge/perseus/commit/7c633558bd51b03f02846e758ecf2fb18c08210a))


### Code Refactorings

* cleaned up `checkpoint()` availability ([72e8af4](https://github.com/framesurge/perseus/commit/72e8af4e1a1d05121fcad2496af6e9fbaaf91eec))
* simplified preload error handling ([746cdc3](https://github.com/framesurge/perseus/commit/746cdc3b7337ec1d6aac93894c67409174e4ca14))


### Documentation Changes

* added missing language declarations ([e08ce5a](https://github.com/framesurge/perseus/commit/e08ce5a819b05ffeab91019b6a73be136a245f32))
* fixed broken example link ([6fca6a8](https://github.com/framesurge/perseus/commit/6fca6a80fdffa38202b16f5d425425b46217c0c8))
* fixed missing language declarations ([7b9d2df](https://github.com/framesurge/perseus/commit/7b9d2df857ff5ad80d781dd38c50ebc399fdc52c))
* removed note on `cargo-clif` issues ([2572357](https://github.com/framesurge/perseus/commit/2572357cff32a4954e9e8439ebc4701646753154))
* wrote docs on `PerseusApp` ([2b39064](https://github.com/framesurge/perseus/commit/2b390648619b1e75aaca500b68963b73fc072d3f))
* wrote first app tutorial ([149a521](https://github.com/framesurge/perseus/commit/149a521da58d99132735aa9633268e28f3d73dc0))
* wrote further fundamental docs ([5bb9bd3](https://github.com/framesurge/perseus/commit/5bb9bd3a191a29d2ead1b0508e7f83a5c753f27d))
* wrote hydration docs ([a82ee5c](https://github.com/framesurge/perseus/commit/a82ee5cd3beee9c1807b51a0f715714e8dc67ce0))
* wrote js interop docs ([be9dbc9](https://github.com/framesurge/perseus/commit/be9dbc96b42fe0c0fca5787df364791356ab2922))
* wrote more docs ([6c5f6a7](https://github.com/framesurge/perseus/commit/6c5f6a770fe1c6fdae1a64f83bad14f18135adb9))
* wrote new testing docs ([36b3792](https://github.com/framesurge/perseus/commit/36b379200bf26641115ead3c627c8977b8b8dc6d))

## [0.4.0-beta.14](https://github.com/framesurge/perseus/compare/v0.4.0-beta.13...v0.4.0-beta.14) (2023-01-02)


### Bug Fixes

* **cli:** added missing `RUSTFLAGS` ([0735b8c](https://github.com/framesurge/perseus/commit/0735b8c6cb4330e30e4bca3a53d67b5d472ce813)), closes [#249](https://github.com/framesurge/perseus/issues/249)
* **cli:** made `cargo metadata` invocation use given cargo path ([1784909](https://github.com/framesurge/perseus/commit/1784909a5da87b554756d8ea26c64e971d80f389))
* **docs.rs:** added proper metadata config for docs.rs ([55861ac](https://github.com/framesurge/perseus/commit/55861acc243e6edc3e55241457dfdbdcd3c66045))
* fixed typo in docs.rs settings ([b2b3e92](https://github.com/framesurge/perseus/commit/b2b3e929eb275d23a832b874053f3f2bd3ef4a6a))

## [0.4.0-beta.13](https://github.com/framesurge/perseus/compare/v0.4.0-beta.12...v0.4.0-beta.13) (2023-01-02)


### Bug Fixes

* **docs.rs:** added `rustdocflags` ([f06eb48](https://github.com/framesurge/perseus/commit/f06eb48a48cdcbabdb3d768bb26a9de24a8a73ae))
* fixed broken tribble link in contributing docs ([#247](https://github.com/framesurge/perseus/issues/247)) ([f6717fb](https://github.com/framesurge/perseus/commit/f6717fb61799b4e99272142ec2d2c8c76cceae23))
* fixed deployment paths ([0f8127c](https://github.com/framesurge/perseus/commit/0f8127c94f9a0f279385756275ea3ffdfc52d8b9)), closes [#63](https://github.com/framesurge/perseus/issues/63)
* fixed double error page display ([2454205](https://github.com/framesurge/perseus/commit/2454205961831cf0defb177fcf12e0a6623b1a74))
* fixed error view deployment bugs ([f57f7f1](https://github.com/framesurge/perseus/commit/f57f7f1ef77eeeb514d880452730f39a2b450adb))
* temporarily disabled website checking ([a63a1a9](https://github.com/framesurge/perseus/commit/a63a1a9962b9e11f681beaed221b7b05f3792472))
* typo in `perseus new` Cargo.toml ([#248](https://github.com/framesurge/perseus/issues/248)) ([71bfaa1](https://github.com/framesurge/perseus/commit/71bfaa13a54e185acd147d84a61a3748f8d952a9))


### Documentation Changes

* added extra migration step for `.head_with_state()` ([ae93d77](https://github.com/framesurge/perseus/commit/ae93d77bb74efd2153813ebe73c9e46e95981b6a))
* added further migration notes ([1e9fc70](https://github.com/framesurge/perseus/commit/1e9fc70d443dcce81fd72d4137201b3f76754c42))
* added some more migration tips ([16cd9d5](https://github.com/framesurge/perseus/commit/16cd9d5965f8131d7cbcca81a101f1784b6429d7))
* updated all old links ([c1f613c](https://github.com/framesurge/perseus/commit/c1f613cbd32b5b16661d4c9cc28efdd69d931de0))

## [0.4.0-beta.12](https://github.com/framesurge/perseus/compare/v0.4.0-beta.11...v0.4.0-beta.12) (2023-01-01)


### ⚠ BREAKING CHANGES

* all target-gates should switch to `#[cfg(engine)]` and
`#[cfg(client)]`, rather than using `target_arch`
* removed nnow obsolete `#[perseus::engine]` and
`#[perseus::browser]` macros
* plugins can no longer add templates (provide the
functions to the user for them to manually add)
* `G` parameter removed entirely from plugins
* you must now call `.build()` after creating a
template/capsule with `::new(..)` and setting your various functions
* header setting functions now take a `Scope` as their
first argument
* removed several outdated `checkpoint`s (see book for details)
* global state is now built per-locale, and build state
functions therefore take a `locale: String` argument now
* functional plugin actions for failing global state have
been removed due to alternate generation systems (you should hook into
failed builds instead)
* all plugin actions must now return `Result<T, Box<dyn
std::error::Error>>` (return some arbitrary error and then use
`.into()`)
* all plugin actions that previously took
`Rc<perseus::errors::EngineError>` now take `Rc<perseus::errors::Error>`
* `.template()` and `.error_pages()` on `PerseusApp` now
take actual values, rather than functions that return those values
* any functions that took `path` and `locale` now take `StateGeneratorInfo`, which includes those as fields
* all macros on state generator functions (e.g. `#[build_state]`) are replaced by the single macro `#[engine_only_fn]`
* templates that take reactive state must have the `#[template]` annotation and be specified with `.template_with_state()`
* templates that take unreactive state must have no macro annotation, and be specified wijth `.template_with_unreactive_state()`
* templates that take no state must not have a `#[template]` annotation
* `define_app!` has been completely removed in favor of `PerseusApp`
* `#[make_rx(MyRxTypeName)]` must be changed to `#[derive(Serialize, Deserialize, ReactiveState)]`, followed by `#[rx(alias = "MyRxTypeName")]`
* renamed `#[template_rx]` to `#[template]` (unreactive
templates should now use `#[template(unreactive)]`)

### Features

* added more reactive collections ([04e1629](https://github.com/framesurge/perseus/commit/04e16290383fdfa445fee088de3ca647718ef103))
* added support for wasm engines ([f6568e9](https://github.com/framesurge/perseus/commit/f6568e9ce11be97390982c69dedcd737b5692eac))
* added suspended interactivity system ([5efcad4](https://github.com/framesurge/perseus/commit/5efcad48d61f599206cb8e93740a0dc196bf9e00))
* added suspended state and request-time/amalgamated global state ([#242](https://github.com/framesurge/perseus/issues/242)) ([c174a69](https://github.com/framesurge/perseus/commit/c174a69a3f66538159d16807cc25b20b610a3a95)), closes [/github.com/framesurge/perseus/issues/150#issuecomment-1329785198](https://github.com/framesurge//github.com/framesurge/perseus/issues/150/issues/issuecomment-1329785198)
* **cli:** made cli symlink exported content instead of copying it ([c54d884](https://github.com/framesurge/perseus/commit/c54d88462e37e68deaafd051989adbe3a765fde7))
* created capsules system and rewrote just about everything ([#224](https://github.com/framesurge/perseus/issues/224)) ([a4c59f2](https://github.com/framesurge/perseus/commit/a4c59f22c572094c13d2840eb17cc2e75a6fd83d))
* created version of `blame_err!` that doesn't return ([628405d](https://github.com/framesurge/perseus/commit/628405dd20c4a033588559dcd25d43d80f5cd9ec))
* improved global state system ([#223](https://github.com/framesurge/perseus/issues/223)) ([85d7f4a](https://github.com/framesurge/perseus/commit/85d7f4a0b57ed5d956ab2571a9d386e9d2c109cd))
* made `#[template_rx]` support unreactive state ([43fdf11](https://github.com/framesurge/perseus/commit/43fdf11485870810afd123bb169d139a4801e3cf))
* made plugin actions return `Result` ([42f0d99](https://github.com/framesurge/perseus/commit/42f0d9937d59c1664c0741f837cc33a7b62b8ff0)), closes [#234](https://github.com/framesurge/perseus/issues/234)
* removed all redundant macros! 🎉 ([#238](https://github.com/framesurge/perseus/issues/238)) ([dccb7a5](https://github.com/framesurge/perseus/commit/dccb7a5be85da4023394c996ddff19b8bd5ac759))
* removed unnecessary panics and added custom panic handler system ([c232aed](https://github.com/framesurge/perseus/commit/c232aed6b70735df108fcdc645eda09fec2b9d3a))


### Bug Fixes

* 'fixed' hydration issues ([182f72b](https://github.com/framesurge/perseus/commit/182f72bcdf39171601411327b94f94d693a6f2f5))
* allowed statenames other than `state` ([c80e647](https://github.com/framesurge/perseus/commit/c80e647db12b07e8508e7d2c0fa4ed38dc3207bf))
* fixed `perseus new` ([659b911](https://github.com/framesurge/perseus/commit/659b911f5dc9eaa8165f4b76a225ce0f312e9dd6))
* fixed js-interop example ([2928bb7](https://github.com/framesurge/perseus/commit/2928bb71637a0ed593604ccb8d22b35030dbd3c5))
* made tool windows installations resilient against paths with whitespace ([#228](https://github.com/framesurge/perseus/issues/228)) ([0d3717f](https://github.com/framesurge/perseus/commit/0d3717fe140a766a4a2b9ca4055f400712b6d645)), closes [#227](https://github.com/framesurge/perseus/issues/227)
* updated website examples ([6410f22](https://github.com/framesurge/perseus/commit/6410f2204909ec304e1ed350bf0450e790241a8e))
* **website:** fixed index example sourcing ([88c29cd](https://github.com/framesurge/perseus/commit/88c29cdae1ceb387a2cd6897bebec20b34f01fd4))


### Code Refactorings

* cleaned up some things ([4df987e](https://github.com/framesurge/perseus/commit/4df987edfaf59cd4832d085f251f5c917c2659e4))
* improved ergonomics of `PerseusApp` ([9c3444a](https://github.com/framesurge/perseus/commit/9c3444a80403b61838ea3febf726a8d7833a8bc1))
* turn `#[make_rx]` into `#[derive(ReactiveState)]` ([#237](https://github.com/framesurge/perseus/issues/237)) ([8ec2d6f](https://github.com/framesurge/perseus/commit/8ec2d6f9b09601950fd20aefc6f2c77fb309d034))
* use child scopes for pages ([#230](https://github.com/framesurge/perseus/issues/230)) ([6af8191](https://github.com/framesurge/perseus/commit/6af819165f01b4816cd3594b176d72fa9b27bc68))


### Documentation Changes

* alerted users to [#229](https://github.com/framesurge/perseus/issues/229) ([a825cec](https://github.com/framesurge/perseus/commit/a825cece49d6615a408927712bcb4567e59fdf27))
* clarified warning ([0696e48](https://github.com/framesurge/perseus/commit/0696e48b97e8d6232bd853c2a67b3d0c87bdbed6))
* fixed readme links ([75dd5ce](https://github.com/framesurge/perseus/commit/75dd5ced6a22cf0444493fff5bed35da61362183))
* removed old installation instruction ([bd06110](https://github.com/framesurge/perseus/commit/bd06110ff334b5dcd1f4e455c632b22fd3d7705f))
* removed readme warning about [#229](https://github.com/framesurge/perseus/issues/229) ([195078a](https://github.com/framesurge/perseus/commit/195078a6e65458fcc96b54dc03bf4f4296439d63))
* updated v0.4.x docs and documented migration ([8a81a3e](https://github.com/framesurge/perseus/commit/8a81a3e8561b7c94afa9778d22adf1032d6f06f6))

## [0.4.0-beta.12](https://github.com/framesurge/perseus/compare/v0.4.0-beta.11...v0.4.0-beta.12) (2023-01-01)


### ⚠ BREAKING CHANGES

* all target-gates should switch to `#[cfg(engine)]` and
`#[cfg(client)]`, rather than using `target_arch`
* removed nnow obsolete `#[perseus::engine]` and
`#[perseus::browser]` macros
* plugins can no longer add templates (provide the
functions to the user for them to manually add)
* `G` parameter removed entirely from plugins
* you must now call `.build()` after creating a
template/capsule with `::new(..)` and setting your various functions
* header setting functions now take a `Scope` as their
first argument
* removed several outdated `checkpoint`s (see book for details)
* global state is now built per-locale, and build state
functions therefore take a `locale: String` argument now
* functional plugin actions for failing global state have
been removed due to alternate generation systems (you should hook into
failed builds instead)
* all plugin actions must now return `Result<T, Box<dyn
std::error::Error>>` (return some arbitrary error and then use
`.into()`)
* all plugin actions that previously took
`Rc<perseus::errors::EngineError>` now take `Rc<perseus::errors::Error>`
* `.template()` and `.error_pages()` on `PerseusApp` now
take actual values, rather than functions that return those values
* any functions that took `path` and `locale` now take `StateGeneratorInfo`, which includes those as fields
* all macros on state generator functions (e.g. `#[build_state]`) are replaced by the single macro `#[engine_only_fn]`
* templates that take reactive state must have the `#[template]` annotation and be specified with `.template_with_state()`
* templates that take unreactive state must have no macro annotation, and be specified wijth `.template_with_unreactive_state()`
* templates that take no state must not have a `#[template]` annotation
* `define_app!` has been completely removed in favor of `PerseusApp`
* `#[make_rx(MyRxTypeName)]` must be changed to `#[derive(Serialize, Deserialize, ReactiveState)]`, followed by `#[rx(alias = "MyRxTypeName")]`
* renamed `#[template_rx]` to `#[template]` (unreactive
templates should now use `#[template(unreactive)]`)

### Features

* added more reactive collections ([04e1629](https://github.com/framesurge/perseus/commit/04e16290383fdfa445fee088de3ca647718ef103))
* added support for wasm engines ([f6568e9](https://github.com/framesurge/perseus/commit/f6568e9ce11be97390982c69dedcd737b5692eac))
* added suspended interactivity system ([5efcad4](https://github.com/framesurge/perseus/commit/5efcad48d61f599206cb8e93740a0dc196bf9e00))
* added suspended state and request-time/amalgamated global state ([#242](https://github.com/framesurge/perseus/issues/242)) ([c174a69](https://github.com/framesurge/perseus/commit/c174a69a3f66538159d16807cc25b20b610a3a95)), closes [/github.com/framesurge/perseus/issues/150#issuecomment-1329785198](https://github.com/framesurge//github.com/framesurge/perseus/issues/150/issues/issuecomment-1329785198)
* **cli:** made cli symlink exported content instead of copying it ([c54d884](https://github.com/framesurge/perseus/commit/c54d88462e37e68deaafd051989adbe3a765fde7))
* created capsules system and rewrote just about everything ([#224](https://github.com/framesurge/perseus/issues/224)) ([a4c59f2](https://github.com/framesurge/perseus/commit/a4c59f22c572094c13d2840eb17cc2e75a6fd83d))
* created version of `blame_err!` that doesn't return ([628405d](https://github.com/framesurge/perseus/commit/628405dd20c4a033588559dcd25d43d80f5cd9ec))
* improved global state system ([#223](https://github.com/framesurge/perseus/issues/223)) ([85d7f4a](https://github.com/framesurge/perseus/commit/85d7f4a0b57ed5d956ab2571a9d386e9d2c109cd))
* made `#[template_rx]` support unreactive state ([43fdf11](https://github.com/framesurge/perseus/commit/43fdf11485870810afd123bb169d139a4801e3cf))
* made plugin actions return `Result` ([42f0d99](https://github.com/framesurge/perseus/commit/42f0d9937d59c1664c0741f837cc33a7b62b8ff0)), closes [#234](https://github.com/framesurge/perseus/issues/234)
* removed all redundant macros! 🎉 ([#238](https://github.com/framesurge/perseus/issues/238)) ([dccb7a5](https://github.com/framesurge/perseus/commit/dccb7a5be85da4023394c996ddff19b8bd5ac759))
* removed unnecessary panics and added custom panic handler system ([c232aed](https://github.com/framesurge/perseus/commit/c232aed6b70735df108fcdc645eda09fec2b9d3a))


### Bug Fixes

* 'fixed' hydration issues ([182f72b](https://github.com/framesurge/perseus/commit/182f72bcdf39171601411327b94f94d693a6f2f5))
* allowed statenames other than `state` ([c80e647](https://github.com/framesurge/perseus/commit/c80e647db12b07e8508e7d2c0fa4ed38dc3207bf))
* fixed `perseus new` ([659b911](https://github.com/framesurge/perseus/commit/659b911f5dc9eaa8165f4b76a225ce0f312e9dd6))
* fixed js-interop example ([2928bb7](https://github.com/framesurge/perseus/commit/2928bb71637a0ed593604ccb8d22b35030dbd3c5))
* made tool windows installations resilient against paths with whitespace ([#228](https://github.com/framesurge/perseus/issues/228)) ([0d3717f](https://github.com/framesurge/perseus/commit/0d3717fe140a766a4a2b9ca4055f400712b6d645)), closes [#227](https://github.com/framesurge/perseus/issues/227)
* updated website examples ([6410f22](https://github.com/framesurge/perseus/commit/6410f2204909ec304e1ed350bf0450e790241a8e))
* **website:** fixed index example sourcing ([88c29cd](https://github.com/framesurge/perseus/commit/88c29cdae1ceb387a2cd6897bebec20b34f01fd4))


### Code Refactorings

* cleaned up some things ([4df987e](https://github.com/framesurge/perseus/commit/4df987edfaf59cd4832d085f251f5c917c2659e4))
* improved ergonomics of `PerseusApp` ([9c3444a](https://github.com/framesurge/perseus/commit/9c3444a80403b61838ea3febf726a8d7833a8bc1))
* turn `#[make_rx]` into `#[derive(ReactiveState)]` ([#237](https://github.com/framesurge/perseus/issues/237)) ([8ec2d6f](https://github.com/framesurge/perseus/commit/8ec2d6f9b09601950fd20aefc6f2c77fb309d034))
* use child scopes for pages ([#230](https://github.com/framesurge/perseus/issues/230)) ([6af8191](https://github.com/framesurge/perseus/commit/6af819165f01b4816cd3594b176d72fa9b27bc68))


### Documentation Changes

* alerted users to [#229](https://github.com/framesurge/perseus/issues/229) ([a825cec](https://github.com/framesurge/perseus/commit/a825cece49d6615a408927712bcb4567e59fdf27))
* clarified warning ([0696e48](https://github.com/framesurge/perseus/commit/0696e48b97e8d6232bd853c2a67b3d0c87bdbed6))
* fixed readme links ([75dd5ce](https://github.com/framesurge/perseus/commit/75dd5ced6a22cf0444493fff5bed35da61362183))
* removed old installation instruction ([bd06110](https://github.com/framesurge/perseus/commit/bd06110ff334b5dcd1f4e455c632b22fd3d7705f))
* removed readme warning about [#229](https://github.com/framesurge/perseus/issues/229) ([195078a](https://github.com/framesurge/perseus/commit/195078a6e65458fcc96b54dc03bf4f4296439d63))
* updated v0.4.x docs and documented migration ([8a81a3e](https://github.com/framesurge/perseus/commit/8a81a3e8561b7c94afa9778d22adf1032d6f06f6))

## [0.4.0-beta.12](https://github.com/framesurge/perseus/compare/v0.4.0-beta.11...v0.4.0-beta.12) (2023-01-01)


### ⚠ BREAKING CHANGES

* all target-gates should switch to `#[cfg(engine)]` and
`#[cfg(client)]`, rather than using `target_arch`
* removed nnow obsolete `#[perseus::engine]` and
`#[perseus::browser]` macros
* plugins can no longer add templates (provide the
functions to the user for them to manually add)
* `G` parameter removed entirely from plugins
* you must now call `.build()` after creating a
template/capsule with `::new(..)` and setting your various functions
* header setting functions now take a `Scope` as their
first argument
* removed several outdated `checkpoint`s (see book for details)
* global state is now built per-locale, and build state
functions therefore take a `locale: String` argument now
* functional plugin actions for failing global state have
been removed due to alternate generation systems (you should hook into
failed builds instead)
* all plugin actions must now return `Result<T, Box<dyn
std::error::Error>>` (return some arbitrary error and then use
`.into()`)
* all plugin actions that previously took
`Rc<perseus::errors::EngineError>` now take `Rc<perseus::errors::Error>`
* `.template()` and `.error_pages()` on `PerseusApp` now
take actual values, rather than functions that return those values
* any functions that took `path` and `locale` now take `StateGeneratorInfo`, which includes those as fields
* all macros on state generator functions (e.g. `#[build_state]`) are replaced by the single macro `#[engine_only_fn]`
* templates that take reactive state must have the `#[template]` annotation and be specified with `.template_with_state()`
* templates that take unreactive state must have no macro annotation, and be specified wijth `.template_with_unreactive_state()`
* templates that take no state must not have a `#[template]` annotation
* `define_app!` has been completely removed in favor of `PerseusApp`
* `#[make_rx(MyRxTypeName)]` must be changed to `#[derive(Serialize, Deserialize, ReactiveState)]`, followed by `#[rx(alias = "MyRxTypeName")]`
* renamed `#[template_rx]` to `#[template]` (unreactive
templates should now use `#[template(unreactive)]`)

### Features

* added more reactive collections ([04e1629](https://github.com/framesurge/perseus/commit/04e16290383fdfa445fee088de3ca647718ef103))
* added support for wasm engines ([f6568e9](https://github.com/framesurge/perseus/commit/f6568e9ce11be97390982c69dedcd737b5692eac))
* added suspended interactivity system ([5efcad4](https://github.com/framesurge/perseus/commit/5efcad48d61f599206cb8e93740a0dc196bf9e00))
* added suspended state and request-time/amalgamated global state ([#242](https://github.com/framesurge/perseus/issues/242)) ([c174a69](https://github.com/framesurge/perseus/commit/c174a69a3f66538159d16807cc25b20b610a3a95)), closes [/github.com/framesurge/perseus/issues/150#issuecomment-1329785198](https://github.com/framesurge//github.com/framesurge/perseus/issues/150/issues/issuecomment-1329785198)
* **cli:** made cli symlink exported content instead of copying it ([c54d884](https://github.com/framesurge/perseus/commit/c54d88462e37e68deaafd051989adbe3a765fde7))
* created capsules system and rewrote just about everything ([#224](https://github.com/framesurge/perseus/issues/224)) ([a4c59f2](https://github.com/framesurge/perseus/commit/a4c59f22c572094c13d2840eb17cc2e75a6fd83d))
* created version of `blame_err!` that doesn't return ([628405d](https://github.com/framesurge/perseus/commit/628405dd20c4a033588559dcd25d43d80f5cd9ec))
* improved global state system ([#223](https://github.com/framesurge/perseus/issues/223)) ([85d7f4a](https://github.com/framesurge/perseus/commit/85d7f4a0b57ed5d956ab2571a9d386e9d2c109cd))
* made `#[template_rx]` support unreactive state ([43fdf11](https://github.com/framesurge/perseus/commit/43fdf11485870810afd123bb169d139a4801e3cf))
* made plugin actions return `Result` ([42f0d99](https://github.com/framesurge/perseus/commit/42f0d9937d59c1664c0741f837cc33a7b62b8ff0)), closes [#234](https://github.com/framesurge/perseus/issues/234)
* removed all redundant macros! 🎉 ([#238](https://github.com/framesurge/perseus/issues/238)) ([dccb7a5](https://github.com/framesurge/perseus/commit/dccb7a5be85da4023394c996ddff19b8bd5ac759))
* removed unnecessary panics and added custom panic handler system ([c232aed](https://github.com/framesurge/perseus/commit/c232aed6b70735df108fcdc645eda09fec2b9d3a))


### Bug Fixes

* 'fixed' hydration issues ([182f72b](https://github.com/framesurge/perseus/commit/182f72bcdf39171601411327b94f94d693a6f2f5))
* allowed statenames other than `state` ([c80e647](https://github.com/framesurge/perseus/commit/c80e647db12b07e8508e7d2c0fa4ed38dc3207bf))
* fixed `perseus new` ([659b911](https://github.com/framesurge/perseus/commit/659b911f5dc9eaa8165f4b76a225ce0f312e9dd6))
* fixed js-interop example ([2928bb7](https://github.com/framesurge/perseus/commit/2928bb71637a0ed593604ccb8d22b35030dbd3c5))
* made tool windows installations resilient against paths with whitespace ([#228](https://github.com/framesurge/perseus/issues/228)) ([0d3717f](https://github.com/framesurge/perseus/commit/0d3717fe140a766a4a2b9ca4055f400712b6d645)), closes [#227](https://github.com/framesurge/perseus/issues/227)
* updated website examples ([6410f22](https://github.com/framesurge/perseus/commit/6410f2204909ec304e1ed350bf0450e790241a8e))
* **website:** fixed index example sourcing ([88c29cd](https://github.com/framesurge/perseus/commit/88c29cdae1ceb387a2cd6897bebec20b34f01fd4))


### Code Refactorings

* cleaned up some things ([4df987e](https://github.com/framesurge/perseus/commit/4df987edfaf59cd4832d085f251f5c917c2659e4))
* improved ergonomics of `PerseusApp` ([9c3444a](https://github.com/framesurge/perseus/commit/9c3444a80403b61838ea3febf726a8d7833a8bc1))
* turn `#[make_rx]` into `#[derive(ReactiveState)]` ([#237](https://github.com/framesurge/perseus/issues/237)) ([8ec2d6f](https://github.com/framesurge/perseus/commit/8ec2d6f9b09601950fd20aefc6f2c77fb309d034))
* use child scopes for pages ([#230](https://github.com/framesurge/perseus/issues/230)) ([6af8191](https://github.com/framesurge/perseus/commit/6af819165f01b4816cd3594b176d72fa9b27bc68))


### Documentation Changes

* alerted users to [#229](https://github.com/framesurge/perseus/issues/229) ([a825cec](https://github.com/framesurge/perseus/commit/a825cece49d6615a408927712bcb4567e59fdf27))
* clarified warning ([0696e48](https://github.com/framesurge/perseus/commit/0696e48b97e8d6232bd853c2a67b3d0c87bdbed6))
* fixed readme links ([75dd5ce](https://github.com/framesurge/perseus/commit/75dd5ced6a22cf0444493fff5bed35da61362183))
* removed old installation instruction ([bd06110](https://github.com/framesurge/perseus/commit/bd06110ff334b5dcd1f4e455c632b22fd3d7705f))
* removed readme warning about [#229](https://github.com/framesurge/perseus/issues/229) ([195078a](https://github.com/framesurge/perseus/commit/195078a6e65458fcc96b54dc03bf4f4296439d63))
* updated v0.4.x docs and documented migration ([8a81a3e](https://github.com/framesurge/perseus/commit/8a81a3e8561b7c94afa9778d22adf1032d6f06f6))

## [0.4.0-beta.11](https://github.com/framesurge/perseus/compare/v0.4.0-beta.10...v0.4.0-beta.11) (2022-11-06)


### Features

* added greater control over minification ([5385f12](https://github.com/framesurge/perseus/commit/5385f1239448d7955493f1ff8e6ee6bd84787e47))
* added page state store caching, preloading, and memory management ([#204](https://github.com/framesurge/perseus/issues/204)) ([0c4fa6b](https://github.com/framesurge/perseus/commit/0c4fa6bf5ce5063b0a564a92b77600d63e80a86e))


### Bug Fixes

* fixed quotation marks in rneder cfg interpolation ([6a9c9c7](https://github.com/framesurge/perseus/commit/6a9c9c7b43235e6d05913d9a60b264b00b2c9ae1))
* fixed serving bug with index page ([8a1efb5](https://github.com/framesurge/perseus/commit/8a1efb5af8c2071325ffe441490055bad24a2fdc))
* fixed subsequent loads of pages with special characters ([c112b58](https://github.com/framesurge/perseus/commit/c112b584e00d814d4c711110ae5dc723b847dc7d))
* fixed support for paths with url encodings ([a329952](https://github.com/framesurge/perseus/commit/a32995210e0346c86254d06c04b0291c3f8e0c20))
* made build paths possible on index page ([18bd1bf](https://github.com/framesurge/perseus/commit/18bd1bf834894c17ca7d270db8e730fefcb7b42d))
* **website:** added langauge declarations to som badly formatted codeblocks ([3f5b8cf](https://github.com/framesurge/perseus/commit/3f5b8cf712821da8826bddef788a5318c7606682))
* **website:** made scrollbars obey dark theme ([5f0c704](https://github.com/framesurge/perseus/commit/5f0c704cc52c229c891fb91aa898b2b0053a773d))
* **website:** updated `#[component]` annotations ([430d4a2](https://github.com/framesurge/perseus/commit/430d4a2babff5ea3eff8e25fc00fd3407560ad8d))


### Performance Improvements

* added inbuilt minification ([50e04e0](https://github.com/framesurge/perseus/commit/50e04e0651996eda379a992414b389eed1d1a3ea))
* **website:** improved image performance ([a0328fc](https://github.com/framesurge/perseus/commit/a0328fc78eead93c6b4249dfbdf1d15e7edb9f58))


### Documentation Changes

* add instruction to replace perseus_integration ([#197](https://github.com/framesurge/perseus/issues/197)) ([8612c9e](https://github.com/framesurge/perseus/commit/8612c9e7ed35deaee90b16b882234454ef1c7a44))
* clarified tokio issues in v0.3.x in docs ([ff36ff2](https://github.com/framesurge/perseus/commit/ff36ff23f5c4132ace13f2ad2e18488ed0dd3be9))
* **perseus-macro:** readability fixups ([#194](https://github.com/framesurge/perseus/issues/194)) ([f74b400](https://github.com/framesurge/perseus/commit/f74b4008fb901f915791e41048a9e8865b69a41c))


### Code Refactorings

* **src:** readability improvements ([#193](https://github.com/framesurge/perseus/issues/193)) ([2309c5c](https://github.com/framesurge/perseus/commit/2309c5cdd83efb1bc8e7962ac6c1b7a7eb60c0ee))
* **website:** added some nicer padding on docs pages ([b152a2e](https://github.com/framesurge/perseus/commit/b152a2e5e8ad0b04ffe2408f57c07af358b57038))
* **website:** made codeblocks slightly lighter in light mode ([264b65f](https://github.com/framesurge/perseus/commit/264b65faf1196cfc080de80edf274ed5593c7e74))

## [0.4.0-beta.10](https://github.com/framesurge/perseus/compare/v0.4.0-beta.9...v0.4.0-beta.10) (2022-09-21)


### Features

* created new website! 🎉 ([#181](https://github.com/framesurge/perseus/issues/181)) ([b7ace94](https://github.com/framesurge/perseus/commit/b7ace9438843b5c0a4a4e04ade1d754b96328732))
* **exporting:** made export server serve 404 automatically ([cb4c5b1](https://github.com/framesurge/perseus/commit/cb4c5b1fcfcb7d73ae6920e52c299c1e0835a49f))
* added `prelude` module to simplify imports ([60557af](https://github.com/framesurge/perseus/commit/60557afed5657603d010c6d8ea35c034aad08cec))


### Bug Fixes

* fixed relative path hosting ([8a01244](https://github.com/framesurge/perseus/commit/8a0124420067c55235fb666e36a3056c9cb08c03))


### Documentation Changes

* **examples/core:** readability improvements ([#191](https://github.com/framesurge/perseus/issues/191)) ([653ca85](https://github.com/framesurge/perseus/commit/653ca851ae3d12611dac40dcca252f4b4d8ca75b))
* **perseus-cli:** readability improvements ([#192](https://github.com/framesurge/perseus/issues/192)) ([4f301ab](https://github.com/framesurge/perseus/commit/4f301ab3efe3e32b1481d98d70deeb7fa5fc1d70))
* `next` docs readability improvements ([#189](https://github.com/framesurge/perseus/issues/189)) ([a91bc2c](https://github.com/framesurge/perseus/commit/a91bc2c94a543b569417209203ba10917a125b9e))
* 0.4.x readability improvements ([#188](https://github.com/framesurge/perseus/issues/188)) ([e385e21](https://github.com/framesurge/perseus/commit/e385e2120fc55b9ec131c40ef8ac0a2adbdf4ddb))

## [0.4.0-beta.9](https://github.com/framesurge/perseus/compare/v0.4.0-beta.8...v0.4.0-beta.9) (2022-09-11)


### Features

* **cli:** added `perseus check` for faster validation ([5a1be87](https://github.com/framesurge/perseus/commit/5a1be873e82c14330ce55df5aba2b1e4fe21d814))


### Bug Fixes

* fixed revalidation time strings ([76ed6fc](https://github.com/framesurge/perseus/commit/76ed6fcb21e7bf7ee8026315e0336ba6dac8220d))
* fixed route announcer dimensions ([dba4a4f](https://github.com/framesurge/perseus/commit/dba4a4f4044418c9e1997f2dd12cb8702fe68c3a))


### Documentation Changes

* merged 0.4.x with next docs ([4be9e5b](https://github.com/framesurge/perseus/commit/4be9e5bc72bd5dc42ae77b4cb463375a5bdb9814))
* noted incremental generation/revalidation quirks in examples ([8fd19c8](https://github.com/framesurge/perseus/commit/8fd19c8ba7930e13ca6d669d41edc551015b97c7))
* updated migration docs for current project status ([9e985ce](https://github.com/framesurge/perseus/commit/9e985cebcef66e470204faf0b9df9182308e36a8))

## [0.4.0-beta.8](https://github.com/framesurge/perseus/compare/v0.4.0-beta.7...v0.4.0-beta.8) (2022-09-04)


### Features

* added support for error page heads ([#179](https://github.com/framesurge/perseus/issues/179)) ([41590b1](https://github.com/framesurge/perseus/commit/41590b179210be04e7c05de55355511ec637b967))
* removed final wrapper `<div>` ([e4106f6](https://github.com/framesurge/perseus/commit/e4106f69e1162dc7530113aead64646d727a22f3))
* removed wrapper `<div>` inside root ([3198558](https://github.com/framesurge/perseus/commit/31985581dbbb535b607f5cc378c697001faef635))


### Bug Fixes

* fixed non-hydration rendering ([ae934bf](https://github.com/framesurge/perseus/commit/ae934bf75a2d9a2031e4d0fd2d3bb1eef8ac73b5))
* **website:** pinned to sycamore v0.8.0-beta.7 for now ([af6c017](https://github.com/framesurge/perseus/commit/af6c0174193a465f7137b33d127539252d7286e3))
* **website:** tmp fix for build ([21c608b](https://github.com/framesurge/perseus/commit/21c608ba7f4c3440db84e0262992a54818240f3c))
* made head replacement only target dynamic elements ([73aa387](https://github.com/framesurge/perseus/commit/73aa38714e3b0ecdbd625e7d555b5527aa23a226)), closes [#182](https://github.com/framesurge/perseus/issues/182)


### Documentation Changes

* made general improvements ([#180](https://github.com/framesurge/perseus/issues/180)) ([1def873](https://github.com/framesurge/perseus/commit/1def8735557b6587c3738cc09cf593b22cc0d454))
* updated faq for latest version ([fd42e44](https://github.com/framesurge/perseus/commit/fd42e44f25be8f42edb8d467bf5cd81da3661952))


### Code Refactorings

* enabled hydration in `plugins` example ([64cff9f](https://github.com/framesurge/perseus/commit/64cff9fbc2f22eb6049f2c156bc6512fd4b59a25))

## [0.4.0-beta.7](https://github.com/framesurge/perseus/compare/v0.4.0-beta.6...v0.4.0-beta.7) (2022-08-20)


### ⚠ BREAKING CHANGES

* - Added `ErrorLoaded { path }` case to `RouterLoadState` (which must now be matched)
- Removed `page_visible testing` checkpoint (use `page_interactive` instead)
- `router_entry checkpoint` is now only fired on subsequent loads

### Features

* **cli:** added support for automatically updating tools from lockfile ([52ab3f1](https://github.com/framesurge/perseus/commit/52ab3f17dec243fa2759489b7e5116c9c4ac4de9)), closes [#169](https://github.com/framesurge/perseus/issues/169)
* redesigned app shell with support for hydration ([#177](https://github.com/framesurge/perseus/issues/177)) ([d407727](https://github.com/framesurge/perseus/commit/d4077272fe2849546a043a8ae73f723635bee8ea))


### Bug Fixes

* fixed response caching functions ([e29e5d2](https://github.com/framesurge/perseus/commit/e29e5d2be36920126a32eca2ac3e4edbca6b8a95))


### Performance Improvements

* removed unnecessary content pre-rendering from server render process ([e1c9ad3](https://github.com/framesurge/perseus/commit/e1c9ad36421e3aa8f21b0d9065c22d5082e9bde7))

## [0.4.0-beta.6](https://github.com/framesurge/perseus/compare/v0.4.0-beta.5...v0.4.0-beta.6) (2022-08-13)


### Features

* added feedback to deployed server binaries ([04eab84](https://github.com/framesurge/perseus/commit/04eab84821a93fd5de0fa333a868375929c9adf8)), closes [#164](https://github.com/framesurge/perseus/issues/164)


### Bug Fixes

* **cli:** fixed `perseus new`/`perseus init` package versions ([22dac34](https://github.com/framesurge/perseus/commit/22dac34f87b173e7d16912a1a83a64f13e917ec0))
* **website:** added necessary images back ([8e6707c](https://github.com/framesurge/perseus/commit/8e6707c55b0b38bed9d53edd3bfa304029837599))
* fixed runtime panics in app shell ([cb39dc1](https://github.com/framesurge/perseus/commit/cb39dc178f6186588c507da4799d6a21c9aafbf8))


### Documentation Changes

* fixed typo ([#171](https://github.com/framesurge/perseus/issues/171)) ([c3e7c35](https://github.com/framesurge/perseus/commit/c3e7c35e5b9c7e0303f0e6d3da459bc05985d14f))
* update fetching demo readme ([#168](https://github.com/framesurge/perseus/issues/168)) ([402b36e](https://github.com/framesurge/perseus/commit/402b36e0fea335ded44c6a51da598872636d2afb))

## [0.4.0-beta.5](https://github.com/framesurge/perseus/compare/v0.4.0-beta.4...v0.4.0-beta.5) (2022-07-18)


### ⚠ BREAKING CHANGES

* `[lib]`/`[[bin]]` settings no longer required in
`Cargo.toml`, and `lib.rs` should be renamed to `main.rs` (everything is
a binary now)

### Features

* made cli auto-install needed tools and use global flags for config ([#160](https://github.com/framesurge/perseus/issues/160)) ([4682b9d](https://github.com/framesurge/perseus/commit/4682b9d3e2cf542c6ac521ee3563c4e34c342a8b))
* **i18n:** added lightweight translator ([b5bb075](https://github.com/framesurge/perseus/commit/b5bb07505d75aaa8b995d95244d96f6a46a4c545))


### Bug Fixes

* **cli:** made system tools cache XDG-compliant ([085cd9b](https://github.com/framesurge/perseus/commit/085cd9bda92012a0d18235d040cdc83f4069f294))
* fixed typo in new project template ([#163](https://github.com/framesurge/perseus/issues/163)) ([1d7cc9f](https://github.com/framesurge/perseus/commit/1d7cc9f9c024f4f62f798e06097f895159438ad5))


### Documentation Changes

* fixed broken links after binary changes ([fa99478](https://github.com/framesurge/perseus/commit/fa9947806113d2cd6feaad77ba3614d87434513a))
* updated docs for new cli setup ([eba6cab](https://github.com/framesurge/perseus/commit/eba6cab892774f4b49c032ff30a5378eb1959a40))

## [0.4.0-beta.4](https://github.com/framesurge/perseus/compare/v0.4.0-beta.3...v0.4.0-beta.4) (2022-07-14)


### ⚠ BREAKING CHANGES

* passed path, locale, and request to logic-based revalidation
* header setting functions now take either a usual state
argument or no state (no more `Option<T>`s)
* state amalgamation functions now take `path`, `locale`,
`build_state`, and `request_state`; `States` is private
* restructured exports significantly

### Features

* **cli:** added `perseus new` and `perseus init` ([0bf879b](https://github.com/framesurge/perseus/commit/0bf879bfc072117ed82e42b8fd111f21bc895483)), closes [#128](https://github.com/framesurge/perseus/issues/128)
* passed path, locale, and request to logic-based revalidation ([5473683](https://github.com/framesurge/perseus/commit/5473683a59db2aa19aabe5986461b739458a0470))
* removed `Option<T>` weirdness on header setting ([869000b](https://github.com/framesurge/perseus/commit/869000ba5fb4083c0175c011e3a31f398dbdd350))
* **cli:** added support for watching custom files ([723d4ca](https://github.com/framesurge/perseus/commit/723d4cae0659f4f02e2e7affc957df065b156373))
* added further target directory separation ([b2d7e16](https://github.com/framesurge/perseus/commit/b2d7e16c6912107a7717a8cb421728fd4f4b0a6c))
* added proper state mgmt to amalgamation ([ceaf7b2](https://github.com/framesurge/perseus/commit/ceaf7b264507856989134de12fe27b137f990679))
* added type-safety to time-based revalidation ([7b3ff88](https://github.com/framesurge/perseus/commit/7b3ff880fa6910c750400e95cbba33772d2eee6a))


### Bug Fixes

* fixed `perseus deploy` target binary name on Windows ([#156](https://github.com/framesurge/perseus/issues/156)) ([32a6f24](https://github.com/framesurge/perseus/commit/32a6f247cbac016735a9c341e469a1be6b8fc37c))
* fixed warp js snippets handling ([dab7e72](https://github.com/framesurge/perseus/commit/dab7e726ddfd73b6d4df66d6bff2e05f16b4aae9))
* fixed wasm issues with `ComputedDuration` ([247caff](https://github.com/framesurge/perseus/commit/247caffcd655062beb094d9d87a71da3d6fc7e22))


### Performance Improvements

* split target directories for engine/server ([651349d](https://github.com/framesurge/perseus/commit/651349d9b32a87cae3bf912694385fa80cee66ed))


### Code Refactorings

* restructured exports significantly ([70d425b](https://github.com/framesurge/perseus/commit/70d425b11670ef2796f83a5c338a4e95cce185a0))


### Documentation Changes

* fixed typos in readme ([9ba6947](https://github.com/framesurge/perseus/commit/9ba694750e6e7da4fefe70839f23a243ac64eabd))
* fixed typos in tiny example readme ([8ab3afd](https://github.com/framesurge/perseus/commit/8ab3afdc7553d7486c89817b56884f9b121dfa68))
* rewrote advanced docs ([191d8c9](https://github.com/framesurge/perseus/commit/191d8c9f040e34debd98e88cd8d0dd589c852af9))
* scaffolded out FDT ([ca12e03](https://github.com/framesurge/perseus/commit/ca12e0373a954cf7524dcd05ec804c565a535eb0))
* updated docs for `perseus new` and co ([8810c23](https://github.com/framesurge/perseus/commit/8810c23f604329549893842e00585f8a0b8cc6e8))
* **examples:** updated example writing docs and base example ([0c1b578](https://github.com/framesurge/perseus/commit/0c1b578e5b06a64258a348a68359224a7d95976f))
* substantially improved code-level docs ([6ec4852](https://github.com/framesurge/perseus/commit/6ec48529e2371dc464d993f3743091b9cccb0a96))
* updated js interop example readme ([11ed444](https://github.com/framesurge/perseus/commit/11ed44470139688c6f3c89fd7d872bda076eab13))
* **examples:** added js interop example ([766dd44](https://github.com/framesurge/perseus/commit/766dd4433aabb91fbec74b1957c3cba0729997c7))
* finished updates to code-level docs ([76ef81c](https://github.com/framesurge/perseus/commit/76ef81c5bc4f6b4055c93ae005247fb4766bb333))
* updated docs for cranelift and wasm-opt ([7829c74](https://github.com/framesurge/perseus/commit/7829c7447c888a66db250667ee7b91bd296385e8))
* updated signature docs ([2e008d4](https://github.com/framesurge/perseus/commit/2e008d4e48a6301796371dfb2adb96a3019dac27))
* wrote second app tutorial ([55751aa](https://github.com/framesurge/perseus/commit/55751aa24a515089c66d114d7a0fb128f2234354))
* wrote state generation docs ([eb7e8ee](https://github.com/framesurge/perseus/commit/eb7e8ee56bd95958ce3e85d98b39bafa6e21bb9b))

## [0.4.0-beta.3](https://github.com/framesurge/perseus/compare/v0.4.0-beta.2...v0.4.0-beta.3) (2022-07-04)


### ⚠ BREAKING CHANGES

* Made custom servers take `ServerProps` and `(String,
u16)` for the host and port.
* Changed delimiter in `t!` macro variable interpolation
from `:` to `=` to solve compiler misinterpretation errors

### Features

* made custom servers easier and added example ([d450a81](https://github.com/framesurge/perseus/commit/d450a81435a8eaf4d37e806ce57624a511ae38c5))
* **website:** added auto-versioning to docs.rs shorthand links ([e727121](https://github.com/framesurge/perseus/commit/e727121bc08be004a8bf4e9c29409c4244ce1a8b))
* **website:** added shorthand for docs.rs links ([ee379a1](https://github.com/framesurge/perseus/commit/ee379a1baa38c9be00c06db7ff431cfe61f58ffb))
* added size optimizations support into cli ([b083173](https://github.com/framesurge/perseus/commit/b083173033298784443eee2a9d398eac7b879d02))


### Bug Fixes

* fixed actix web integration ([6b538d3](https://github.com/framesurge/perseus/commit/6b538d3d058e17cbef490e3c4ab7844d84a771ff))
* fixed cast issues with `t!` interpolation ([2cae13a](https://github.com/framesurge/perseus/commit/2cae13a08af5918a7b5fa40f6cb928b7fb7c5ecf))
* fixed data setup for actix web integration ([6a129a1](https://github.com/framesurge/perseus/commit/6a129a1342619701b8600017693f078f98240e04))
* fixed live reloading ([1b924af](https://github.com/framesurge/perseus/commit/1b924af0ade5abe13b8fec1b4ecd766b0871c5ed))
* fixed size optimizations on website ([4e320c8](https://github.com/framesurge/perseus/commit/4e320c892934610cf46951cb56792513187528aa))
* fixed up website ([946e326](https://github.com/framesurge/perseus/commit/946e3265e389eca3842a1322753a0013b8945b21))
* removed bad default feature from actix integration ([4612e01](https://github.com/framesurge/perseus/commit/4612e015aa4059c0fc8e88c35b6777038c252d9c))
* **website:** added opts back ([4b25d46](https://github.com/framesurge/perseus/commit/4b25d469ce2b2c32c3586113fb79082aa8ab8f95))
* **website:** fixed latest stable version warning link ([a32f1ee](https://github.com/framesurge/perseus/commit/a32f1ee67d70d1dee9c974e26ad731238997a0b5))
* **website:** updated `opt-level` to `z` ([2b49591](https://github.com/framesurge/perseus/commit/2b49591eb049d81c6cbd5a29af3bf380e5aa074d))


### Documentation Changes

* added live reloading/hsr docs ([bd8a702](https://github.com/framesurge/perseus/commit/bd8a70287a92a34b95d7213ff55207027edb2d99))
* clarified `perseus-integration` crate purpose ([73dc11a](https://github.com/framesurge/perseus/commit/73dc11afe31f676d4a4d13f3eb24e7df0d4c57b4))
* clarified items in changelog ([2d0cc1b](https://github.com/framesurge/perseus/commit/2d0cc1b090c0161df3e2768405e12201a506abe6))
* copied `next` to `0.4.x` for improved UX ([3fd6834](https://github.com/framesurge/perseus/commit/3fd683426fa38e53028349044f7d5903d0005cd9))
* updated optimizations docs ([51ad962](https://github.com/framesurge/perseus/commit/51ad962cc7161657b856d2058765dd3ecc18f750))
* wrote some sections of the docs ([c7a9f09](https://github.com/framesurge/perseus/commit/c7a9f0995a51eb325f4949fa8fbcca16f6c22e93))

## [0.4.0-beta.2](https://github.com/framesurge/perseus/compare/v0.4.0-beta.1...v0.4.0-beta.2) (2022-06-27)


### ⚠ BREAKING CHANGES

*    Changed multiple APIs for functional plugin actions related to the builder (they all take the new EngineError type now)
*    Restructured exports related to engine functionality (this will get progressively worse as this PR develops!)
*    Removed the HOST and PORT environment variables for configuring the server (these are replaced with `PERSEUS_HOST` and `PERSEUS_PORT`)
*    Substantially refactored exports from Perseus
*    Divided client-side and server-side exports (many functions will now need to be target-gated)
*    Replaced `#[autoserde(...)]` macro with macros for each state function (`#[build_state]`, `#[build_paths]`, etc.)
*    The #[build_paths] macro must now be applied to all build paths functions (for client/server functionality division)
*    #[perseus::main] now takes an argument as the default server to use (server integrations should now be imported and used)
*    Made state functions automatically target-gated as `#[cfg(not(feature = "wasm32"))]`
*    The `#[should_revalidate]` macro must now be applied to all revalidation determination functions (for client/server functionality division)
*    WARNING: any filesystem paths in app building are now relative to the root of the app, not `.perseus/` (may require changes)

### Features

* add axum integration ([#146](https://github.com/framesurge/perseus/issues/146)) ([dbe8207](https://github.com/framesurge/perseus/commit/dbe8207eaac86e32fedacc0816c0b1a48512709d)), closes [#137](https://github.com/framesurge/perseus/issues/137) [#137](https://github.com/framesurge/perseus/issues/137)
* removed `.perseus/` ([#151](https://github.com/framesurge/perseus/issues/151)) ([14f415a](https://github.com/framesurge/perseus/commit/14f415a5610fd065966aede20365649595c59104))
* updated fetching example for async reqwest ([ea98465](https://github.com/framesurge/perseus/commit/ea984651a3e648dedbcba4891bcb88e4061ec689))


### Bug Fixes

* fixed uncaught syntax error ([3cc247b](https://github.com/framesurge/perseus/commit/3cc247bc3eb1ea7bfeddece62eae5b9678877496))
* removed extra doctype declaration ([7e7b2c4](https://github.com/framesurge/perseus/commit/7e7b2c4bd2185f3ed73b9754da2a6cdc87ccdd41)), closes [#154](https://github.com/framesurge/perseus/issues/154)


### Code Refactorings

* added error pages to `tiny` example ([4c6d1cf](https://github.com/framesurge/perseus/commit/4c6d1cf54af6205372994d068efd5a1240e8492e)), closes [#153](https://github.com/framesurge/perseus/issues/153)


### Documentation Changes

* create based scaffold for new docs ([39e7c83](https://github.com/framesurge/perseus/commit/39e7c83b24b92d6df9c2c4e112afd95ec5211b59))
* fixed misnomer in second app tutorial ([873562c](https://github.com/framesurge/perseus/commit/873562c5f8a6b404ab2e5359b269312a197ccbc7)), closes [#147](https://github.com/framesurge/perseus/issues/147)
* fixed some broken links ([5a0e107](https://github.com/framesurge/perseus/commit/5a0e10739d7f3b878a07f447524b50a0a8f1cb87)), closes [#149](https://github.com/framesurge/perseus/issues/149)
* removed erroneous code example in hello world tutorial ([51f2b2f](https://github.com/framesurge/perseus/commit/51f2b2f00e56336b3a4f0c35757c5083150841c8))
* updated server communication docs ([c4c7ed2](https://github.com/framesurge/perseus/commit/c4c7ed2fdbfef24d1c794d31c88e71d6c54d3248))
* updated tiny example in readme ([3642a4b](https://github.com/framesurge/perseus/commit/3642a4bde7c2a52ea7743359f7998af1100abced))

## [0.4.0-beta.1](https://github.com/framesurge/perseus/compare/v0.3.5...v0.4.0-beta.1) (2022-05-30)


### Features

* upgrade to Sycamore v0.8.0 ([#142](https://github.com/framesurge/perseus/issues/142)) ([b14b4e0](https://github.com/framesurge/perseus/commit/b14b4e0afc8c0b73a3f6ba64bec1bb0460849ea9)), closes [#137](https://github.com/framesurge/perseus/issues/137) [#137](https://github.com/framesurge/perseus/issues/137)


### Documentation Changes

* added notes about hydration bugs ([2aef79c](https://github.com/framesurge/perseus/commit/2aef79c402174e92b931de678648a95c1db6c678))
* fix typo in `basic` example readme ([#141](https://github.com/framesurge/perseus/issues/141)) ([35ff172](https://github.com/framesurge/perseus/commit/35ff172b44b42e5dcd68a3023c1450d903fa0804))
* updated docs for index view with example ([eccf137](https://github.com/framesurge/perseus/commit/eccf137032fbe8e6507be9e9317edc16e7576a4f))

### [0.3.5](https://github.com/framesurge/perseus/compare/v0.3.4...v0.3.5) (2022-04-20)


### Bug Fixes

* removed hydration ids from `.index_view()` ([41ab625](https://github.com/framesurge/perseus/commit/41ab625dba98290023f5b6de6894dc4899aaabf5))

### [0.3.4](https://github.com/framesurge/perseus/compare/v0.3.4-rc.6...v0.3.4) (2022-04-19)


### Documentation Changes

* fixed empty link ([8bd6a8f](https://github.com/framesurge/perseus/commit/8bd6a8f1c1a49d8c0dc3b2bd244bb2cfe4e7e46c))

### [0.3.4-rc.6](https://github.com/framesurge/perseus/compare/v0.3.4-rc.5...v0.3.4-rc.6) (2022-04-17)


### Bug Fixes

* removed cyclical dependency ([ef307cc](https://github.com/framesurge/perseus/commit/ef307cc1b2fedce9b73a4df4c58f26ece1cc3977))


### Documentation Changes

* updated readme book link ([0ba0187](https://github.com/framesurge/perseus/commit/0ba01877b5c3e2994f6cfea66dc4193d14b79aa9))
* updated readme code example ([bd30995](https://github.com/framesurge/perseus/commit/bd3099526fe08101cbc8377e28df536ac13b6924))

### [0.3.4-rc.5](https://github.com/framesurge/perseus/compare/v0.3.4-rc.4...v0.3.4-rc.5) (2022-04-14)

### [0.3.4-rc.4](https://github.com/framesurge/perseus/compare/v0.3.4-rc.3...v0.3.4-rc.4) (2022-04-14)

### [0.3.4-rc.3](https://github.com/framesurge/perseus/compare/v0.3.4-rc.2...v0.3.4-rc.3) (2022-04-13)


### Bug Fixes

* fixed versioning for `perseus-macro` dependencies ([e46c3ca](https://github.com/framesurge/perseus/commit/e46c3caf0e36dfc6ec8a0a99a88ee83b99ceb2be))

### [0.3.4-rc.2](https://github.com/framesurge/perseus/compare/v0.3.4-rc.1...v0.3.4-rc.2) (2022-04-13)

### [0.3.4-rc.1](https://github.com/framesurge/perseus/compare/v0.3.3...v0.3.4-rc.1) (2022-04-13)


### Features

* **examples:** added auth example and docs ([e02088c](https://github.com/framesurge/perseus/commit/e02088cf3ed7669b73792acaa9febc600de82437))
* added `.make_unrx()` ([b974576](https://github.com/framesurge/perseus/commit/b974576eaac7fd4aa0b533ec63d688bd24ab0733))
* added better errors when no state generation functions are provided ([e565632](https://github.com/framesurge/perseus/commit/e5656320c780048596dba6cad3aff8307968df69))
* added component name inference to `template_rx` ([d1ba2ef](https://github.com/framesurge/perseus/commit/d1ba2ef82d7519d4a28e6d392303f49d89ff3d8c))
* added examples for and finalized idb wrapper ([362d5ca](https://github.com/framesurge/perseus/commit/362d5caf0dbb7ccdc6a85a4706f5e5ab15d7294c))
* added global and reactive state ([90288f6](https://github.com/framesurge/perseus/commit/90288f65fe3f64575cb3a4dd6e133da9f99a49bf)), closes [#103](https://github.com/framesurge/perseus/issues/103)
* added global state ([a5fcc56](https://github.com/framesurge/perseus/commit/a5fcc567166dfd1710cdaad925b764ab5881c8c1)), closes [#119](https://github.com/framesurge/perseus/issues/119)
* added global state rehydration ([10634fb](https://github.com/framesurge/perseus/commit/10634fb7046438ca518ef6f40133220b06887422))
* added hot state reloading ([9805a7b](https://github.com/framesurge/perseus/commit/9805a7bfead8f24793c0b7e29f90d84470d910c4)), closes [#121](https://github.com/framesurge/perseus/issues/121)
* added idb wrapper for state freezing ([9d2a729](https://github.com/framesurge/perseus/commit/9d2a729ff9f370630ca8023c36d05bb9b5d6f7ee))
* added lazy global state instantiation ([82430db](https://github.com/framesurge/perseus/commit/82430db463769e9f329aebd8057f46b45562e6be))
* added live reloading ([2e33424](https://github.com/framesurge/perseus/commit/2e3342498b585aa10ef96933fe834986db92b8d5)), closes [#122](https://github.com/framesurge/perseus/issues/122)
* added macro to enable fine-grained reactive state ([e12d15c](https://github.com/framesurge/perseus/commit/e12d15c2a48962b55cb9126ce818436f6b78da6d))
* added page state store rehydration ([d95e355](https://github.com/framesurge/perseus/commit/d95e3550ed89a7091e20922f6e5c3e1af01b06e9))
* added proper error handling to hsr ([469732a](https://github.com/framesurge/perseus/commit/469732aede593bbb4aa36dda5873d6176573138c))
* added proper state thawing ([ea545a9](https://github.com/framesurge/perseus/commit/ea545a9d9b9bd30fdfaf26c1cfeddccdc55751ce))
* added reloading server ([1f37700](https://github.com/framesurge/perseus/commit/1f377003bddc22e4b041961d758ae5bc34b808f2))
* added route rehydration ([101f92a](https://github.com/framesurge/perseus/commit/101f92a9eb9bffb67bfec0a154a4b5dd3dd4e119))
* added router state ([#115](https://github.com/framesurge/perseus/issues/115)) ([9ee6904](https://github.com/framesurge/perseus/commit/9ee69044ef8831d6f977782dba75324b7125aa1f))
* added same-page reloading ([6e32c8f](https://github.com/framesurge/perseus/commit/6e32c8f0d78e28495ac48224e56176a9d91a683f)), closes [#120](https://github.com/framesurge/perseus/issues/120)
* added state freezing ([891f3bb](https://github.com/framesurge/perseus/commit/891f3bb7e02087b450292da7ee591b2e5f206420))
* added support for `#[make_rx(...)]` on unit `struct`s ([cb2f49d](https://github.com/framesurge/perseus/commit/cb2f49d7fd2d6b266246ae426728896ea7dae923))
* added support for templates that take only global state ([c60af8a](https://github.com/framesurge/perseus/commit/c60af8a020480360372443c22e3791949e7c4e07))
* added support for wasm2js ([ce07134](https://github.com/framesurge/perseus/commit/ce071345c32d4a6780ab4c05264db76b38973584))
* improved `template2` ergonomics ([c238df9](https://github.com/framesurge/perseus/commit/c238df9e754848fa570f36013b775c588b588e9e))
* made `web_log!` cross-platform and only needing perseus ([b7e8389](https://github.com/framesurge/perseus/commit/b7e838973fea48e3c844c79195dad2b384d3a3d0))
* passed reload server info to client ([27880a5](https://github.com/framesurge/perseus/commit/27880a5373bbec591893f1418e1fe5dce0d9c165))
* set up functional plugin actions for global state ([6aa45aa](https://github.com/framesurge/perseus/commit/6aa45aa06f1c99ad99477a8c746d15b2b5e499a5))
* typed options system ([#130](https://github.com/framesurge/perseus/issues/130)) ([ccd4c43](https://github.com/framesurge/perseus/commit/ccd4c438fd54511341537740ee214b5c28d2e42d))
* **a11y:** added route announcer ([76c0930](https://github.com/framesurge/perseus/commit/76c093065d6021817326092bb9ed47f4f4084e72)), closes [#124](https://github.com/framesurge/perseus/issues/124)
* **cli:** added custom engines support ([b31855e](https://github.com/framesurge/perseus/commit/b31855eb9f97653d5b67ab278f341213fb1455f7)), closes [#59](https://github.com/framesurge/perseus/issues/59)
* **plugins:** added functional actions for exporting error pages ([36abcc1](https://github.com/framesurge/perseus/commit/36abcc11634cb1ffc8235c6498abd5d6b3140a8b))


### Bug Fixes

* added `Debug` for `TranslationArgs` ([51422ed](https://github.com/framesurge/perseus/commit/51422ed792ec604a1359e0af5631ee85934968f5))
* added utf-8 encoding to default html shell ([fce0db8](https://github.com/framesurge/perseus/commit/fce0db8b6643ca6723328f11d86cf662e88afacf))
* fixed active/global state fallbacks ([193f733](https://github.com/framesurge/perseus/commit/193f733798ff5dc909a30eaf5f71b329756d4e03))
* fixed cli in development for watching ([2941f77](https://github.com/framesurge/perseus/commit/2941f77e8c8259dd9488807a8b40c4bad31145fb))
* fixed clippy lint issues with `wasm-bindgen` ([b2f67e6](https://github.com/framesurge/perseus/commit/b2f67e617ce1b05ff93acaba58d0de39fc87cd21)), closes [rustwasm/wasm-bindgen#2774](https://github.com/rustwasm/wasm-bindgen/issues/2774)
* fixed exporting with typed options system ([18f54a9](https://github.com/framesurge/perseus/commit/18f54a9a27696d46af40762b51d509920dc9403a))
* fixed hsr in deployment ([ec52b1c](https://github.com/framesurge/perseus/commit/ec52b1c97d0aeafa53bfdca805de3690720a46d4))
* fixed includes in docs ([89b420d](https://github.com/framesurge/perseus/commit/89b420defc74411c9f1cad6cb875743ccf63ca6f))
* fixed margin errors in website ([59525b4](https://github.com/framesurge/perseus/commit/59525b49b5a67faa563148ab1a7dcfb8c6927749))
* fixed router ([2260885](https://github.com/framesurge/perseus/commit/2260885d01f550880659d781165b1238a86c39af))
* fixed some trait scoping ([d8416e2](https://github.com/framesurge/perseus/commit/d8416e2d4cb6a88ef93243f6224e2632ab7356dc))
* fixed typo ([48e194b](https://github.com/framesurge/perseus/commit/48e194b2dd1c98bd3c7aeb9e4a094143ab59f30c))
* fixed up tests ([6f979eb](https://github.com/framesurge/perseus/commit/6f979eb4eec85c2e158524de3b730ccc251ff2fb))
* fixed warp integration ([93be5de](https://github.com/framesurge/perseus/commit/93be5de564733069e6a78dea62d6b01e5ae12323))
* made hsr self-clearing ([1936b62](https://github.com/framesurge/perseus/commit/1936b62bdad9ac7cfc799ea3c1648d44165f651e))
* made i18n example use the right locales function ([6a05c63](https://github.com/framesurge/perseus/commit/6a05c6377d5300a47edd75c09bcfaf867e764b7f))
* made logging work again ([47fbef5](https://github.com/framesurge/perseus/commit/47fbef5b4698eca42781c5f8bf4bea8a64a1718c))
* made page state store work with multiple pages in single template ([4c9c1be](https://github.com/framesurge/perseus/commit/4c9c1bedef8a68b9a9d1d395b6da49b04be218a8))
* typo in default index view ([#132](https://github.com/framesurge/perseus/issues/132)) ([1f1522a](https://github.com/framesurge/perseus/commit/1f1522a764245d8b4b92bed516653693f6f908f5))


### Performance Improvements

* **i18n:** added experimental wasm caching ([2d1ca2d](https://github.com/framesurge/perseus/commit/2d1ca2dc88d1fa7aaabdfcdbfcaecff69f0eb469))


### Code Refactorings

* added `Debug` implementations to everything ([2392daa](https://github.com/framesurge/perseus/commit/2392daa06a6b80290b59adc5f17bcdc5e9c772cf))
* broke out some fn args into separate `struct`s ([1e0ed5c](https://github.com/framesurge/perseus/commit/1e0ed5c554d6811def29474606ddb0a6375cff4c))
* changed default live reloading port to 3100 ([a355157](https://github.com/framesurge/perseus/commit/a355157028246a537333533a8784cee2f2f812ef))
* cleaned up ([ee29ba1](https://github.com/framesurge/perseus/commit/ee29ba10413e4a61bb4b077a371739a775793a0c))
* cleaned up from last refactor ([33f439c](https://github.com/framesurge/perseus/commit/33f439c7d631fb3c0c0abab38800e9bb0d281e5d))
* fixed clippy lints ([2f37374](https://github.com/framesurge/perseus/commit/2f373742d28e8726fd662c1aabdf9e12387e61b7))
* improved html shell readability ([#109](https://github.com/framesurge/perseus/issues/109)) ([69e9f72](https://github.com/framesurge/perseus/commit/69e9f7295b197ad59d41ee61c545ed6d04483520))
* made basic examples use reactive state ([1570e5d](https://github.com/framesurge/perseus/commit/1570e5d57c61d7a1c87b848ffc09f35763d11a8c))
* made examples use typed options system ([02c3c03](https://github.com/framesurge/perseus/commit/02c3c033b5398db3577bed86fb812d23a6718110))
* made live reloading have access to render context ([b9b608a](https://github.com/framesurge/perseus/commit/b9b608a8e3d88604f95bd54350cc985d376f08dd)), closes [#121](https://github.com/framesurge/perseus/issues/121)
* minor code improvements ([#110](https://github.com/framesurge/perseus/issues/110)) ([2c0d344](https://github.com/framesurge/perseus/commit/2c0d344950fc7a30bd1b5c6a5384b2ce3bfd7758))
* moved header setting and static content examples into own ([0449fea](https://github.com/framesurge/perseus/commit/0449fea10ccc59d92b7188dc26d709b36d81c8d0))
* moved html shell into one struct ([832e269](https://github.com/framesurge/perseus/commit/832e269259f258d0624b234b670ab8b2cf8cd22a))
* moved router into core ([b1c4746](https://github.com/framesurge/perseus/commit/b1c4746cc9164ddaefcee8b8ab4f8ef307d2234f))
* moved showcase example into state generation example ([25b9808](https://github.com/framesurge/perseus/commit/25b98083e7b10aae74c1967fa242d6e0cfef6ec5))
* partitioned examples and moved in tests ([33887ab](https://github.com/framesurge/perseus/commit/33887ab46ccfac1520c819e3118e91123595e726))
* reduced allocations in engine server ([3422949](https://github.com/framesurge/perseus/commit/34229498d3645b58b25ca4ab8f8cafb12114ef19))
* renamed `template_with_rx_state` to `template2` ([2956009](https://github.com/framesurge/perseus/commit/2956009bdcd36efa86a37f736fb3affa0d189981))
* renamed `template2` to `template_rx` ([2d99a9a](https://github.com/framesurge/perseus/commit/2d99a9a3cf95539c38591e9510e8770877e5bcb8))
* renamed global state to page state store ([3b2401b](https://github.com/framesurge/perseus/commit/3b2401b2aa596a9b64c9c2f2dd71742101adc00a)), closes [#119](https://github.com/framesurge/perseus/issues/119)
* restructed files ([1700dcb](https://github.com/framesurge/perseus/commit/1700dcb0c785724e83688607d79da28fe24b5fbd))
* restructured logic revalidation example to show types clearly ([cbf2629](https://github.com/framesurge/perseus/commit/cbf2629bebf1d7147c888a154193374e8af994b3))
* updated to typed options system in example basis ([7a7dd6c](https://github.com/framesurge/perseus/commit/7a7dd6cde1e28639ce93fe09bd1ffa502f651735))
* **examples:** split `rx_state` into multiple examples ([d58dd29](https://github.com/framesurge/perseus/commit/d58dd296ffda2ca320a249aad9c9a95834746676))
* restructured tests/examples ([14b4b46](https://github.com/framesurge/perseus/commit/14b4b465a1a33f827cb0baf4b114fa49ce43689e))


### Documentation Changes

* added docs for examples ([16c63ef](https://github.com/framesurge/perseus/commit/16c63ef8c279f327f786b4c18b1d2609fc88cac7))
* added section on how to build bleeding-edge cli ([0e6eb5d](https://github.com/framesurge/perseus/commit/0e6eb5d9ddfca941ebf27900b0188b296425fed5))
* edited hsr blog post ([57913b4](https://github.com/framesurge/perseus/commit/57913b459169b1019697ae4de14abb356af0a223))
* fix broken link, typos and add 0.3.3 as stable version ([#129](https://github.com/framesurge/perseus/issues/129)) ([9f3d5a2](https://github.com/framesurge/perseus/commit/9f3d5a220d24ec2bf9f764b4a266afe0e8b889e5))
* fixed readme links ([82dda10](https://github.com/framesurge/perseus/commit/82dda1065e019a7e1f8416400f53e37011f621d3))
* fixed the last bad link in the readme ([5b575cb](https://github.com/framesurge/perseus/commit/5b575cb96b01db7af4453657008ad21f08cd2abc))
* removed outdated text in the readme ([301cfd3](https://github.com/framesurge/perseus/commit/301cfd3ed2bb13d76e2e00980e840f7bc08793e9))
* updated docs for typed options system ([bd0710d](https://github.com/framesurge/perseus/commit/bd0710d57f78808956d86c02ff564e20e5fd6f41))
* updated template macro docs for no args ([c0c30b6](https://github.com/framesurge/perseus/commit/c0c30b6acd83c46dbe0fd307448d4543b36527db))
* updated the readme ([8b6e142](https://github.com/framesurge/perseus/commit/8b6e142d46fdd0eee89ca5bbb9390077f2f48186))
* updated upgrade guide ([286a29c](https://github.com/framesurge/perseus/commit/286a29c34e2999849a581f1ac2a076524af507ff))
* **blog:** added first draft of hsr post ([78fef13](https://github.com/framesurge/perseus/commit/78fef13a5937009d5fcd9201431699b86014b822))
* **book:** added route announcer docs ([30d0839](https://github.com/framesurge/perseus/commit/30d0839641217ca6dd9b7cb0238e14e9f9c7c00e))
* **book:** clarified dark mode with global state ([261ab84](https://github.com/framesurge/perseus/commit/261ab84b8466d06d342edff10dc4b80a4b785229))
* **book:** documented idb state system ([68a467c](https://github.com/framesurge/perseus/commit/68a467c4124707b3e5f70bb36e42d66966492326))
* **book:** documented wasm/js sizing differences ([578b08b](https://github.com/framesurge/perseus/commit/578b08bc78849341304d206560ce8ba7475ed2fd))
* **book:** fixed examples inclusions and partioned version ([55c21eb](https://github.com/framesurge/perseus/commit/55c21ebe1cff965641808d4850803641898b76fb))
* **book:** rewrote fetching docs ([cba35e6](https://github.com/framesurge/perseus/commit/cba35e6b12964cc989e7b25beece9ae5ce2ade20))
* **book:** updated debugging docs ([7adf684](https://github.com/framesurge/perseus/commit/7adf684765ac11a5c661f6c05d27d0e88b120594))
* **book:** updated state/state generation docs for new examples layout ([13c1f20](https://github.com/framesurge/perseus/commit/13c1f20171ab9dc7967d2ccc46b3fd26f0a463b8))
* **book:** updated tutorials for new examples layout ([28f1af1](https://github.com/framesurge/perseus/commit/28f1af17387dd0e92c907da7a7969775596185c5))
* **book:** wrote docs on custom engines ([4741b67](https://github.com/framesurge/perseus/commit/4741b679afa831f4e8b331a35d7b56088d09dd82))
* **book:** wrote docs on live reloading and hsr ([4cf292f](https://github.com/framesurge/perseus/commit/4cf292ff8a48abf43e991c40c8f6d95e206c1f04))
* **book:** wrote docs on reactive state ([f5a7fbd](https://github.com/framesurge/perseus/commit/f5a7fbdb37f0716016986ac1383b2498beae6588))
* **book:** wrote docs on state thawing ([2d67a40](https://github.com/framesurge/perseus/commit/2d67a40e8054ea7831f916fb13e955c398efa1f4))
* **contrib:** finalized shift to tribble ([09af5c6](https://github.com/framesurge/perseus/commit/09af5c683fc8b22163381220be6bbee8ad35187b))
* **contrib:** hid old contributing docs in details block ([6f850c2](https://github.com/framesurge/perseus/commit/6f850c2f7b86a9cb9a95c0e5639c0c9af0bec25f))
* **examples:** added some new examples and a template for more ([07289f6](https://github.com/framesurge/perseus/commit/07289f63722c154d4945471711e5b674f3ef2354))
* **website:** added comparison note for sycamore and perseus ([bc4f821](https://github.com/framesurge/perseus/commit/bc4f821ee4fabcfab622a27c90ff89e3dfbf835c))
* added contrib docs with tribble ([bc8fc3d](https://github.com/framesurge/perseus/commit/bc8fc3d314bb3bab429754090f8736430900ee03))
* added example to styling docs ([606f635](https://github.com/framesurge/perseus/commit/606f6352c93d3d1b3a115c0f54921d9292d2e0c1))
* added missing link to wasm website ([#117](https://github.com/framesurge/perseus/issues/117)) ([a0dad42](https://github.com/framesurge/perseus/commit/a0dad42a0c2b9767daee14686dd404f2f8bf74c1))
* finalized contributing repo docs ([6aece16](https://github.com/framesurge/perseus/commit/6aece167bb64a25bbff67dafe08690512fc1762d))
* fixed link to discord in issue creation links ([2c14352](https://github.com/framesurge/perseus/commit/2c1435270c868739cf73bda101a2993731589ff9))
* merged `next` with `0.3.x` ([487ce2b](https://github.com/framesurge/perseus/commit/487ce2bf85e0abce0d83434e2cd2ddc1b33f72b6))
* miscellaneous fixes to tribble docs ([c0b5f55](https://github.com/framesurge/perseus/commit/c0b5f55eee5984e56debd949e02ad14ee6b51063))
* restructured and wrote core principles docs ([9ee419e](https://github.com/framesurge/perseus/commit/9ee419eefd6329b94edfd726da0af491346af4e7))
* **contrib:** fixed broken link ([9e5c9b3](https://github.com/framesurge/perseus/commit/9e5c9b3664ff33a5368c11072d82811915dcd1de))
* **tribble:** cleaned up section/endpoint naming ([891cd44](https://github.com/framesurge/perseus/commit/891cd4471a3a4400c0b1fa2bab78871c4d0d56bf))
* **tribble:** fixed minor copy-paste error ([b638d25](https://github.com/framesurge/perseus/commit/b638d25a5fcd237fedbd7f70c710d6739bc87588))

### [0.3.3](https://github.com/framesurge/perseus/compare/v0.3.2...v0.3.3) (2022-02-15)


### Bug Fixes

* fixed actix web beta issues ([2c2e460](https://github.com/framesurge/perseus/commit/2c2e46085e55da8d3610902de7c6e0270f063e41)), closes [#125](https://github.com/framesurge/perseus/issues/125)

### [0.3.2](https://github.com/framesurge/perseus/compare/v0.3.1...v0.3.2) (2022-01-11)


### Features

* added ability to export error pages ([624034b](https://github.com/framesurge/perseus/commit/624034bd0788d175aaf60776968cff86d89fb5f4)), closes [#94](https://github.com/framesurge/perseus/issues/94)
* added external request caching ([3ecad15](https://github.com/framesurge/perseus/commit/3ecad150a20f4326a981563d43517bef53874a09))
* modernized host/port setting for `perseus serve` ([19bd87e](https://github.com/framesurge/perseus/commit/19bd87e6c0f9780af572c79a88025ae0b741c4f2)), closes [#107](https://github.com/framesurge/perseus/issues/107)


### Bug Fixes

* **website:** fixed formatting errors ([4139df9](https://github.com/framesurge/perseus/commit/4139df9d055be41c55b8b92abb831ee20ac60af5))


### Documentation Changes

* updated docs to reflect host/port setting changes ([a930ae2](https://github.com/framesurge/perseus/commit/a930ae2002f6ba2df1f4b93b73d64c8fb20a3f2a))

### [0.3.1](https://github.com/framesurge/perseus/compare/v0.3.0...v0.3.1) (2022-01-02)


### Features

* **website:** added highlighting for dockerfiles ([81e2066](https://github.com/framesurge/perseus/commit/81e206605ea72d1c3c24071ee5105963939475cd))
* re-exported `spawn_local` for convenience ([184381f](https://github.com/framesurge/perseus/commit/184381fbfb27baeb2c7399d5ce94c2d60643b07e))
* **cli:** added basic hot reloading ([b4c93f0](https://github.com/framesurge/perseus/commit/b4c93f0a8202422c2f64779d87e7bcc6bcfb217a))
* **cli:** added hot reloading ([61696b3](https://github.com/framesurge/perseus/commit/61696b32becdb925c5e43dcc60c3d4f9dfa51fc8))
* **cli:** added support for wasm profiling builds ([c2de025](https://github.com/framesurge/perseus/commit/c2de025eb858c50339631781ea810b54651c2242))
* add tokio ([#102](https://github.com/framesurge/perseus/issues/102)) ([150fda8](https://github.com/framesurge/perseus/commit/150fda8062e3bd5c97bb57d759b383b64e43d84b))
* made static generation errors display causes ([ab7742a](https://github.com/framesurge/perseus/commit/ab7742a6733dae977bddde86ceaea3e13301cd86)), closes [#101](https://github.com/framesurge/perseus/issues/101)
* **cli:** added inbuilt server for exported apps ([8274678](https://github.com/framesurge/perseus/commit/82746784c2a803b3e41a56f740840767b0d0de10))


### Bug Fixes

* **cli:** made watcher ignore `.git/` as well ([1a7f6ed](https://github.com/framesurge/perseus/commit/1a7f6edccc988dbf4e791853426d434a5066002a))
* **website:** made github button transition work ([efcf16f](https://github.com/framesurge/perseus/commit/efcf16f3532f99958b7126234e0541b48a310ff6))
* added missing cli argument docs ([7c9fb4a](https://github.com/framesurge/perseus/commit/7c9fb4ad050a71675cdee038675689239764cc31))
* **cli:** used `--dev` by default with `wasm-pack` ([55cc681](https://github.com/framesurge/perseus/commit/55cc681650892fe87a07974378d795ee5b7d090b))
* **deps:** locked `indicatif` to `v0.17.0-beta.1` ([5b979bb](https://github.com/framesurge/perseus/commit/5b979bb4589f3f7d758788fb43d906a460b70567))
* **engine:** fixed incomplete error messages ([e445e56](https://github.com/framesurge/perseus/commit/e445e5682ca96aa44918e3a527a5940207ea3731))


### Documentation Changes

* added cargo corruption to common pitfalls ([9fe2b27](https://github.com/framesurge/perseus/commit/9fe2b273be0a7010721620d450c7be42eda194d5))
* added docker deployment docs ([#98](https://github.com/framesurge/perseus/issues/98)) ([93f2c4b](https://github.com/framesurge/perseus/commit/93f2c4b3fd270e353f6387085aed8e82ed0b7958))
* added docs for cli watching ([4a250e9](https://github.com/framesurge/perseus/commit/4a250e9585f34d7cd13b3d92d2c002b692460227))
* added new example for fetching data ([6b08ffe](https://github.com/framesurge/perseus/commit/6b08ffe8e784818653ad5e4f3556da26f49a5b08)), closes [#96](https://github.com/framesurge/perseus/issues/96)
* added preliminary `define_app!` advanced docs ([69721a6](https://github.com/framesurge/perseus/commit/69721a6e625b8d99461519e310a33eecfe3b501d))
* fixed code in docker docs ([ac5aaf9](https://github.com/framesurge/perseus/commit/ac5aaf9ae0a036167876e467e6324f270e1fda72))
* made changelog more readable ([12ecc92](https://github.com/framesurge/perseus/commit/12ecc92c7dc6361c0837169cdff464ac04d26fa5))
* merged `0.3.0` and `next` ([9f17624](https://github.com/framesurge/perseus/commit/9f176243247e525715a6952c848ea50830f80e1e))
* merged last changes into `next` ([5ab9903](https://github.com/framesurge/perseus/commit/5ab99033fa1d186e394219bce8146d933a2eb88d))
* updated contrib docs for new site command ([9246c12](https://github.com/framesurge/perseus/commit/9246c129f8358f3596e3df99b2d7f6ebe054ea0a))

## [0.3.0](https://github.com/framesurge/perseus/compare/v0.3.0-rc.1...v0.3.0) (2021-12-21)


### Documentation Changes

* removed beta warning ([4e4cc18](https://github.com/framesurge/perseus/commit/4e4cc18b1876c49e6235c0fbc09890fe57b285bf))

<details>
<summary>v0.3.0 Beta Versions</summary>

## [0.3.0-rc.1](https://github.com/framesurge/perseus/compare/v0.3.0-beta.26...v0.3.0-rc.1) (2021-12-21)


### Documentation Changes

* updated to reflect that no hydration doesn't change Lighthouse scores ([aabc247](https://github.com/framesurge/perseus/commit/aabc2477436a5fff2062eda31ae7c6662c43b95a))

## [0.3.0-beta.26](https://github.com/framesurge/perseus/compare/v0.3.0-beta.25...v0.3.0-beta.26) (2021-12-21)


### Code Refactorings

* switched default server integration ([eed2cc0](https://github.com/framesurge/perseus/commit/eed2cc08519fe73a5482e8c7482e20ab0e27df45))

## [0.3.0-beta.25](https://github.com/framesurge/perseus/compare/v0.3.0-beta.24...v0.3.0-beta.25) (2021-12-21)


### Features

* **i18n:** made locale redirection much faster ([61aa406](https://github.com/framesurge/perseus/commit/61aa406eef38136a9067e5e5667b7057aa5a25aa)), closes [#61](https://github.com/framesurge/perseus/issues/61)


### Bug Fixes

* **website:** fixed version issues ([85d8236](https://github.com/framesurge/perseus/commit/85d82362e8aa0a9c259c7e8df97119b5216ba715))
* made hydration opt-in ([4fd38a6](https://github.com/framesurge/perseus/commit/4fd38a6e0426406fe29881f949451a6dddc24331))
* **website:** fixed tailwind not purging ([bd58daa](https://github.com/framesurge/perseus/commit/bd58daa22596858794430ad0b2262082c8678a72))
* disabled hydration on website ([3f2d110](https://github.com/framesurge/perseus/commit/3f2d1101b3f55e14f6d871ed6f603a7614b32d38))
* pinned website version to beta 22 ([5141cec](https://github.com/framesurge/perseus/commit/5141cecc668166fe6c85706d8d343330cb66e837))
* properly disabled hydration on website ([65009fa](https://github.com/framesurge/perseus/commit/65009fad04e54051e923f8d1d5cc1d1cc8751368))


### Documentation Changes

* documented hydration ([c22a5f5](https://github.com/framesurge/perseus/commit/c22a5f534e0d82bf76f9b4b9de635278159989c5))


### Code Refactorings

* removed `path_prefix` from `FsTranslationsManager` ([ed48f3d](https://github.com/framesurge/perseus/commit/ed48f3d31396f716c0f977ddb20c352b099aca17))

## [0.3.0-beta.24](https://github.com/framesurge/perseus/compare/v0.3.0-beta.23...v0.3.0-beta.24) (2021-12-17)


### Features

* made hydration the default ([00258dd](https://github.com/framesurge/perseus/commit/00258dd814f9d9b84b7725f39611600d7c6bd796))

## [0.3.0-beta.23](https://github.com/framesurge/perseus/compare/v0.3.0-beta.22...v0.3.0-beta.23) (2021-12-14)


### Bug Fixes

* fixed placement of `standalone` feature in deployment command ([7609ee1](https://github.com/framesurge/perseus/commit/7609ee1ca5c36ec02d195e384e102e3163e7ecc4)), closes [#92](https://github.com/framesurge/perseus/issues/92)


### Documentation Changes

* add `-r` flag to `entr` commands ([#93](https://github.com/framesurge/perseus/issues/93)) ([d0b863e](https://github.com/framesurge/perseus/commit/d0b863e07ddf00166e5002807dcfe76bf96f9a72))

## [0.3.0-beta.22](https://github.com/framesurge/perseus/compare/v0.3.0-beta.21...v0.3.0-beta.22) (2021-12-13)


### ⚠ BREAKING CHANGES

* upgraded to Sycamore v0.7.0 (see [their changelog](https://github.com/sycamore-rs/sycamore/blob/master/CHANGELOG.md))

### Features

* **cli:** added flag to set server integration to use ([b71fa41](https://github.com/framesurge/perseus/commit/b71fa4134564277973effb77cc4a05bf1a4d6d46))
* removed `PERSEUS_STANDALONE` ([d178f5a](https://github.com/framesurge/perseus/commit/d178f5aaaa80f8c89962b5b41693d696863aa922)), closes [#87](https://github.com/framesurge/perseus/issues/87)
* upgraded to sycamore v0.7.0 ([3989241](https://github.com/framesurge/perseus/commit/3989241bb94a62005819ed652b4a15764867b8f8))


### Bug Fixes

* added missing `cfg` macro line ([006523a](https://github.com/framesurge/perseus/commit/006523a26922a86aba830a4dba895829bb71dc3d))
* fixed error page duplication without hydration ([7b3e62f](https://github.com/framesurge/perseus/commit/7b3e62f335f908d72c0de62f4d82592e38ca67ec))
* **deps:** upgraded to `actix-web` v4.0.0-beta.14 ([139d309](https://github.com/framesurge/perseus/commit/139d309997e15146e9277c6f617c88c67d065049))


### Documentation Changes

* added a few more known bugs ([6bae07c](https://github.com/framesurge/perseus/commit/6bae07cf56a5e9d4427a9a4331b32d5c6d23a6cc))
* cleaned up and added page on publishing plugins ([37acece](https://github.com/framesurge/perseus/commit/37acece139f6da9a59e8e3aea0cf039aeafe6b1c))
* merged `next` and `0.3.x` ([dbb47fb](https://github.com/framesurge/perseus/commit/dbb47fb8677e8fb297102a7ed49de59de206194f))
* updated docs for sycamore v0.7.0 ([e840734](https://github.com/framesurge/perseus/commit/e840734c3907ee510f02b611cab15999870336bd))

## [0.3.0-beta.21](https://github.com/framesurge/perseus/compare/v0.3.0-beta.20...v0.3.0-beta.21) (2021-12-12)


### Bug Fixes

* switched to using `warp-fix-171` ([f3f0a43](https://github.com/framesurge/perseus/commit/f3f0a43d3dc5e757e3e476218e588d6c1ad70ded))

## [0.3.0-beta.20](https://github.com/framesurge/perseus/compare/v0.3.0-beta.19...v0.3.0-beta.20) (2021-12-12)


### Bug Fixes

* made cli update local dependencies properly ([3067071](https://github.com/framesurge/perseus/commit/30670715ed3f8e53c6527d96b54e92fe5b6c8173))

## [0.3.0-beta.19](https://github.com/framesurge/perseus/compare/v0.3.0-beta.18...v0.3.0-beta.19) (2021-12-12)


### ⚠ BREAKING CHANGES

* `Options` renamed to `ServerOptions` for all integrations

* feat: made templates and error pages thread-safe

This involved adding an atomic types system.
Also added basics for a Warp integration (which needs this thread-safety).

* feat: made more things thread-safe and made warp integration nearly work

The problem is `Rc<Translator>`s, so some refactoring needs to be done.

* feat: added nearly all handlers to warp integration

BREAKING_CHANGE: `ServerOptions` now only accepts one static content directory

* fix: made `DummyTranslator` `Clone`able

* feat: added support for static aliases in the warp integration

None of this has been tested yet, so there will likely be bugs.
We now depend on my fork of Warp until [this](https://github.com/seanmonstar/warp/pull/924) is merged.

* fix: pinned `clap` version

### Features

* add warp integration ([#86](https://github.com/framesurge/perseus/issues/86)) ([6adf264](https://github.com/framesurge/perseus/commit/6adf264c7474ec1f8bc71fe37e08c2bf132986dd)), closes [#85](https://github.com/framesurge/perseus/issues/85)

## [0.3.0-beta.18](https://github.com/framesurge/perseus/compare/v0.3.0-beta.17...v0.3.0-beta.18) (2021-11-28)


### Features

* **website:** made docs sidebar nicer ([107b9d3](https://github.com/framesurge/perseus/commit/107b9d3264fb30602c672d359eb187d9b4c58f08))
* added `perseus snoop` and docs for common pitfalls ([3c1a919](https://github.com/framesurge/perseus/commit/3c1a919f074a99423f26f54a3761e3468b13d6d0))
* **i18n:** added fallback non-wasm locale redirection ([589ac1b](https://github.com/framesurge/perseus/commit/589ac1b85f4a035dec36aa19c92a0d2157cea71e))
* **website:** added plugins registry ([de1c217](https://github.com/framesurge/perseus/commit/de1c217f1073206bee5e493ca9571325735d0e71))


### Bug Fixes

* **cli:** 🐛 printed `stdout` and well as `stderr` if a stage fails ([ea1f1f1](https://github.com/framesurge/perseus/commit/ea1f1f1f1ca9e45927eacfbbff6e8cd844f40221)), closes [#74](https://github.com/framesurge/perseus/issues/74)
* **exporting:** 🐛 fixed [#73](https://github.com/framesurge/perseus/issues/73) ([a3f879c](https://github.com/framesurge/perseus/commit/a3f879c20eb2bcfc4592cb41ff0e9052a98d4f84))
* **i18n:** fixed fallback locale redirection with relative paths ([5095388](https://github.com/framesurge/perseus/commit/5095388a275332af5069ef6e4fc94a9ad51b37aa))


### Documentation Changes

* **website:** added more comparisons ([d4dabaf](https://github.com/framesurge/perseus/commit/d4dabaf1a7f4e8396fdecee1dfc03ab9fe99cee5))
* made markdown styles more readable and fixed tldr link ([a74b285](https://github.com/framesurge/perseus/commit/a74b2858155706cef6ed83e118062beb40b9f35d))
* **book:** fixed dependency versions in docs ([2171e9c](https://github.com/framesurge/perseus/commit/2171e9c196671a5aa10bffda1413eb9da566a1cf)), closes [#79](https://github.com/framesurge/perseus/issues/79)
* **readme:** updated contact links ([5f74b07](https://github.com/framesurge/perseus/commit/5f74b07ec0c53851e904e5782e37266b33083f92)), closes [#77](https://github.com/framesurge/perseus/issues/77)
* ✏️ fixed typos in contributing guidelines ([#76](https://github.com/framesurge/perseus/issues/76)) ([5dfedc1](https://github.com/framesurge/perseus/commit/5dfedc16864718837be1a273fe0b28b1d1e24e46))

## [0.3.0-beta.17](https://github.com/framesurge/perseus/compare/v0.3.0-beta.16...v0.3.0-beta.17) (2021-11-07)


### Bug Fixes

* **cli:** 🐛 created parent directories with CLI ([#72](https://github.com/framesurge/perseus/issues/72)) ([6dc0aab](https://github.com/framesurge/perseus/commit/6dc0aabaad88df9cb32a72e24f91b31cc7aaefd3)), closes [#69](https://github.com/framesurge/perseus/issues/69)


### Code Refactorings

* **website:** ♻️ refactored website to use new ergonomics macros ([bb879c6](https://github.com/framesurge/perseus/commit/bb879c6476fb68336f0e4afb2d56783cc559f201))

## [0.3.0-beta.16](https://github.com/framesurge/perseus/compare/v0.3.0-beta.15...v0.3.0-beta.16) (2021-11-04)


### Features

* **templates:** ✨ added `autoserde` macro to improve ergonomics ([eb21299](https://github.com/framesurge/perseus/commit/eb212996192749ba3cb370a239ffe0f31a6707e8)), closes [#57](https://github.com/framesurge/perseus/issues/57)
* **templates:** ✨ added `blame_err!` convenience macro ([6ab178a](https://github.com/framesurge/perseus/commit/6ab178a54a95e5a64b918556c803b8f91ce306a6))
* **templates:** ✨ added `head` ergonomics macro ([fb17e03](https://github.com/framesurge/perseus/commit/fb17e03ce614f94e4d84ed7c6aa1ce6bb99a3025)), closes [#57](https://github.com/framesurge/perseus/issues/57)
* **templates:** ✨ added `template` macro to automate template fn creation ([810ae1b](https://github.com/framesurge/perseus/commit/810ae1b1fb17ce52892454cdbbdd5215ae4b3861)), closes [#57](https://github.com/framesurge/perseus/issues/57)
* **website:** ✨ re-added size optimizations plugin to website ([4364d99](https://github.com/framesurge/perseus/commit/4364d99f94ed3f25c13989c2d7ccd020adbafd36))


### Bug Fixes

* **cli:** 🐛 removed distribution artifacts from cli subcrates ([ebca95c](https://github.com/framesurge/perseus/commit/ebca95c7fcb629a5fc8ff1cf5445424553fc0012))
* **examples:** 🐛 fixed type mismatch in `showcase` example ([7a3dd63](https://github.com/framesurge/perseus/commit/7a3dd630b6aae7168a24aff2f167af4b9d552eac))


### Documentation Changes

* **book:** 🐛 fixed broken amalgamation page link ([1966fd1](https://github.com/framesurge/perseus/commit/1966fd1b176e6e98693f25fc06e6063f9274add9))
* **book:** 📝 added docs for new ergonomics macros ([0c4f3b2](https://github.com/framesurge/perseus/commit/0c4f3b22e069020b3c8bc5940252f58b93fae1a0))
* **book:** 📝 updated `next` from `0.3.x` ([7f8e2f2](https://github.com/framesurge/perseus/commit/7f8e2f2af3f8f1d3a8f2e578f1df8b6b8b0031c9))

## [0.3.0-beta.15](https://github.com/framesurge/perseus/compare/v0.3.0-beta.14...v0.3.0-beta.15) (2021-10-30)


### Features

* **plugins:** ✨ added client privileged plugins ([686f369](https://github.com/framesurge/perseus/commit/686f369ca211030566db78295fe19f72ba300f58))


### Code Refactorings

* **website:** 👽️ updated website for 0.3.0-beta.14 ([71b6f42](https://github.com/framesurge/perseus/commit/71b6f42c43faf0f1203ef80279c8e64b6e25de07))


### Documentation Changes

* **book:** 📝 updated docs for plugins system changes ([a85f150](https://github.com/framesurge/perseus/commit/a85f15020e5c344f0a0c821c92473644b42ad405))

## [0.3.0-beta.14](https://github.com/framesurge/perseus/compare/v0.3.0-beta.13...v0.3.0-beta.14) (2021-10-28)


### ⚠ BREAKING CHANGES

* exports now majorly restructured, some exports may be in different places, please check docs.rs

* refactor: ♻️ refactored to remove unnecessary dependencies
* `fs_extra` errors now accepted as `String`s for all relevant plugin actions

* fix(engine): 🐛 removed engine workspace to allow server or client optimizations

Otherwise client size optimizations also affect the server (which reduces its speed).

* feat(i18n): ✨ added dummy translator to use by default
* the `translator-fluent` flag is now required to use i18n

* feat(engine): ✨ added tinker-only plugins and split engine to reduce bundle sizes

The engine is now composed of a server, a builder (new), and a browser client.

* perf(templates): ⚡️ feature-gated templates to decrease bundle sizes

* docs(book): 📝 added docs for tinker-only plugins

### Features

* ✨ trim bundle sizes with feature-gating ([#68](https://github.com/framesurge/perseus/issues/68)) ([ffea205](https://github.com/framesurge/perseus/commit/ffea205d3e0353800db6468c17b7aa857734cd45))
* **website:** ✨ added size optimizations plugin to website ([60e2658](https://github.com/framesurge/perseus/commit/60e265896e7b9fbfeffb459336b038cb1b491550)), closes [#66](https://github.com/framesurge/perseus/issues/66)


### Code Refactorings

* **i18n:** ♻️ fixed clippy warnings and removed an unused import ([c831fe1](https://github.com/framesurge/perseus/commit/c831fe10c400f1b64ef8fe4463f0fbdbd25129ce))


### Documentation Changes

* **book:** 📝 updated docs for size optimizations plugin ([7b2ff84](https://github.com/framesurge/perseus/commit/7b2ff84b28bc3c99ca401c39d4edc6ee0d4f2321))

## [0.3.0-beta.13](https://github.com/framesurge/perseus/compare/v0.3.0-beta.12...v0.3.0-beta.13) (2021-10-18)


### Bug Fixes

* 🚑️ upgraded clap to fix compile errors ([aed12bc](https://github.com/framesurge/perseus/commit/aed12bc44178577d0a60b8cfbb1d78df8fa7cdec))

## [0.3.0-beta.12](https://github.com/framesurge/perseus/compare/v0.3.0-beta.11...v0.3.0-beta.12) (2021-10-17)


### Bug Fixes

* **plugins:** 🐛 fixed `perseus tinker` deleting `.perseus/` without recreating it ([0e9bed5](https://github.com/framesurge/perseus/commit/0e9bed5fa2ee2f918391167eaeb795d50811c496))


### Documentation Changes

* **book:** ✏️ fixed typos in intro ([#53](https://github.com/framesurge/perseus/issues/53)) ([1aff29c](https://github.com/framesurge/perseus/commit/1aff29c8c6aab21da96a61a77fcdb58d419179cf))
* 📝 added docs for contributing to the docs ([7a211eb](https://github.com/framesurge/perseus/commit/7a211ebf5d34354877177dd75fffacf91efff9a5))

## [0.3.0-beta.11](https://github.com/framesurge/perseus/compare/v0.3.0-beta.10...v0.3.0-beta.11) (2021-10-16)


### Bug Fixes

* 🐛 fixed naive current directory handling for standalone deployment binary ([e9e24da](https://github.com/framesurge/perseus/commit/e9e24dad1e70807bf0694a729e619035e8810b3a)), closes [#63](https://github.com/framesurge/perseus/issues/63)

## [0.3.0-beta.10](https://github.com/framesurge/perseus/compare/v0.3.0-beta.9...v0.3.0-beta.10) (2021-10-16)


### ⚠ BREAKING CHANGES

* `build_app`/`export_app`now take a `&TemplateMap` (`get_templates_vec` abolished)

* feat(plugins): ✨ added `tinker` action and command

* feat(examples): ✨ added `plugins` example and removed plugins code from other examples

This includes tests.

* fix(plugins): 🐛 fixed plugin data system

Note that `PluginData` is now replaced by `Any`.

* docs(book): ✏️ fixed missing link to lighthouse in book intro

* refactor(plugins): ♻️ removed plugin type system

Any plugin can now take functional or control actions. Docs still need updating.

* refactor(plugins): 🔥 removed old `get_immutable_store` actions

These are replaced by the `set_immutable_store` settings action

* fix(exporting): 🐛 fixed engine crate name change bug in exporting

* docs(book): 📝 added docs for plugins

### Features

* ✨ add plugins system ([#62](https://github.com/framesurge/perseus/issues/62)) ([ca0aaa2](https://github.com/framesurge/perseus/commit/ca0aaa2cd9cd5c22eb653af820c0e437fa4d9f2b))


### Documentation Changes

* **book:** 📝 merged `next` docs with 0.3.x docs for plugins ([c1e8033](https://github.com/framesurge/perseus/commit/c1e8033687b1aaa5efecefe0502467d2b8ce6694))

## [0.3.0-beta.9](https://github.com/framesurge/perseus/compare/v0.3.0-beta.8...v0.3.0-beta.9) (2021-10-12)


### ⚠ BREAKING CHANGES

* `Rc`s are eliminated and done behind the scenes

### Features

* ✨ removed `Rc`s completely ([d02189b](https://github.com/framesurge/perseus/commit/d02189bc4b0fbec0ddb96ade8fa87275f39f3042))
* **website:** ✨ added comparisons page ([#56](https://github.com/framesurge/perseus/issues/56)) ([61dac01](https://github.com/framesurge/perseus/commit/61dac01b838df23cc0f33b0d65fcb7bf5f252770))
* **website:** ✨ added proper docs links parsing system ([cfa2d60](https://github.com/framesurge/perseus/commit/cfa2d6025e624cf658236bbdc80b8d6470085c6d))


### Bug Fixes

* **i18n:** 🐛 fixed `link!` macro with base path ([d676471](https://github.com/framesurge/perseus/commit/d676471f28608618e7693583f5a0e8bd9bf29805))
* **i18n:** 🐛 fixed locale redirection `//` ([488a9a0](https://github.com/framesurge/perseus/commit/488a9a081429805e25a6415366cd464ee1234fd4))
* **website:** 🐛 fetched examples from git so they don't go obsolete in older versions ([5608a6a](https://github.com/framesurge/perseus/commit/5608a6ad2486909091b067e144607c6a39c56075)), closes [#60](https://github.com/framesurge/perseus/issues/60)
* **website:** 🐛 fixed links in docs version warnings ([295b875](https://github.com/framesurge/perseus/commit/295b8757283a407e321565ae1c15ee4d98ef9125))
* **website:** 🚑️ pinned website to sycamore v0.6.1 to prevent base path problems ([71a142d](https://github.com/framesurge/perseus/commit/71a142dc2496ee020447cda1dde9380365386e68)), closes [#60](https://github.com/framesurge/perseus/issues/60)


### Documentation Changes

* 📝 removed warning about [#60](https://github.com/framesurge/perseus/issues/60) from readme ([4ed3783](https://github.com/framesurge/perseus/commit/4ed37835b79298fc9d07957810ff9efd5fa76794))
* **book:** 📝 merged 0.3.x and next versions of docs ([9a4a956](https://github.com/framesurge/perseus/commit/9a4a9565172afe96ebcaf8e44f9362e09e453d33))
* **book:** 📝 updated docs and added new information on a few things ([8169153](https://github.com/framesurge/perseus/commit/816915333b51b8df21841adbf294462c10c6e3a8)), closes [#46](https://github.com/framesurge/perseus/issues/46)
* **book:** 📝 updated links in docs ([c5398a3](https://github.com/framesurge/perseus/commit/c5398a3b231786d771020532912ef7f80b7e4ac9))
* 📝 removed warning about book being down ([1cb9ec6](https://github.com/framesurge/perseus/commit/1cb9ec6ab4cb76bc144a680bb1d21ff5f1c3c2d2))
* **website:** 📝 mention `browser-sync` as dependency for working with website ([#55](https://github.com/framesurge/perseus/issues/55)) ([a97c325](https://github.com/framesurge/perseus/commit/a97c3251f446c40655edba8d795875a88805fd92))

## [0.3.0-beta.8](https://github.com/framesurge/perseus/compare/v0.3.0-beta.7...v0.3.0-beta.8) (2021-10-08)


### Bug Fixes

* **i18n:** 🐛 fixed path prefixing with locale redirection ([241741f](https://github.com/framesurge/perseus/commit/241741ff3055665f5721635d08b5770910f74add))
* **i18n:** 🐛 made locale redirection work without trailing forward slash ([90b3a99](https://github.com/framesurge/perseus/commit/90b3a990c19baafb763422575a1ef188baacf495))
* **templates:** 🐛 inserted `<base>` element at top of `<head>` ([25959d7](https://github.com/framesurge/perseus/commit/25959d79cf8ab40764100b9ababbe4ede8ededad))
* **website:** 🐛 fixed absolute path links in website ([221fa24](https://github.com/framesurge/perseus/commit/221fa24e48f7374c427256c5d9ab6884d68755e3))
* **website:** 🐛 fixed index page styling on non-firefox browsers ([#54](https://github.com/framesurge/perseus/issues/54)) ([aced234](https://github.com/framesurge/perseus/commit/aced2346fdce10ff0c16daf5c95e73de7120cac4))
* **website:** 🐛 fixed website links ([54de491](https://github.com/framesurge/perseus/commit/54de49130ec253ab61d6217a60379d2fa0eedd97))
* **website:** 💄 made github button same size as get started button on index page ([c472e04](https://github.com/framesurge/perseus/commit/c472e04a0d29615909a49248179ca8b27cdb0f60)), closes [#54](https://github.com/framesurge/perseus/issues/54)


### Performance Improvements

* **website:** ⚡️ added size optimizations on website ([31fb1f8](https://github.com/framesurge/perseus/commit/31fb1f84a0b21f4f5a3da646cd396f58f6dd4c37))


### Code Refactorings

* **website:** ♻️ updated website routes for path prefixing ([28bba42](https://github.com/framesurge/perseus/commit/28bba423a75329f9610f7b61ee7e846e266c3d52))

## [0.3.0-beta.7](https://github.com/framesurge/perseus/compare/v0.3.0-beta.6...v0.3.0-beta.7) (2021-10-06)


### ⚠ BREAKING CHANGES

* **routing:** multiple *internal* function signatures accept exxtra parameter for path prefix

### Features

* **routing:** ✨ added support for relative path hosting with `PERSEUS_BASE_PATH` environment variable ([b7d6eb6](https://github.com/framesurge/perseus/commit/b7d6eb680d3a4368b6d74bfe748fa70207436107)), closes [#48](https://github.com/framesurge/perseus/issues/48)
* ✨ added website ([#47](https://github.com/framesurge/perseus/issues/47)) ([45a0f6c](https://github.com/framesurge/perseus/commit/45a0f6c327fc9386ca31dd6f305cdb387dda5ce0)), closes [#46](https://github.com/framesurge/perseus/issues/46)


### Bug Fixes

* **routing:** 🐛 made back button work with locale redirection ([cf60c12](https://github.com/framesurge/perseus/commit/cf60c123600a1dad936fb0ed0b4855d903ee25a3)), closes [#50](https://github.com/framesurge/perseus/issues/50)


### Documentation Changes

* **book:** 📝 added docs for relative path deployment ([1ecc94f](https://github.com/framesurge/perseus/commit/1ecc94f5fd6a8399fc8ae13e931968c7d1df05b3))

## [0.3.0-beta.6](https://github.com/framesurge/perseus/compare/v0.3.0-beta.5...v0.3.0-beta.6) (2021-10-02)


### Bug Fixes

* **exporting:** 🚑 fixed partial flattening in exporting ([bdbdc56](https://github.com/framesurge/perseus/commit/bdbdc5628591dc33b8b170a74ea5ba647491fae3))

## [0.3.0-beta.5](https://github.com/framesurge/perseus/compare/v0.3.0-beta.4...v0.3.0-beta.5) (2021-10-02)


### Bug Fixes

* 🚑 fixed page encodings ([6d2b7e6](https://github.com/framesurge/perseus/commit/6d2b7e6641d4e59c6c6db2b42af494dbc667e21e))

## [0.3.0-beta.4](https://github.com/framesurge/perseus/compare/v0.3.0-beta.3...v0.3.0-beta.4) (2021-10-02)


### Bug Fixes

* **templates:** 🐛 decoded path before passing to build state ([596f38e](https://github.com/framesurge/perseus/commit/596f38e8684efbe795b6cc3ed2b68b6c3528f3cf)), closes [#44](https://github.com/framesurge/perseus/issues/44)

## [0.3.0-beta.3](https://github.com/framesurge/perseus/compare/v0.3.0-beta.2...v0.3.0-beta.3) (2021-10-02)


### ⚠ BREAKING CHANGES

* **i18n:** build/request state now take locale as second parameter (request state takes request as third now)

### Features

* **i18n:** ✨ passed locale to build and request state ([#43](https://github.com/framesurge/perseus/issues/43)) ([95d28bb](https://github.com/framesurge/perseus/commit/95d28bb2525feb3eb332666d9c66f713bfd06fa3))


### Documentation Changes

* **book:** 📝 updated migration guide for beta ([643e51e](https://github.com/framesurge/perseus/commit/643e51efc0da3f2d212cbcb1e9e83d3361d1c923))

## [0.3.0-beta.2](https://github.com/framesurge/perseus/compare/v0.3.0-beta.1...v0.3.0-beta.2) (2021-10-01)


### Bug Fixes

* 🐛 fixed build paths issues ([#41](https://github.com/framesurge/perseus/issues/41)) ([532243e](https://github.com/framesurge/perseus/commit/532243e07a1b70d41fe841444fc62d382c2d6a31)), closes [#40](https://github.com/framesurge/perseus/issues/40)

## [0.3.0-beta.1](https://github.com/framesurge/perseus/compare/v0.2.3...v0.3.0-beta.1) (2021-09-30)


### ⚠ BREAKING CHANGES

* removed `ConfigManager` in favor of `ImmutableStore`, replaced `config_manager` with `dist_path` in `define_app!`

* feat: ✨ created `MutableStore` for mutable build artifacts

This replaces `ConfigManager` fully.
* many function signatures now include `MutableStore`, changes to `dist/` structure, `mutable_store` now in `define_app!`, `RouteInfo` includes `was_incremental_match`

* docs(book): 📝 added docs for new stores system

* refactor(examples): ♻️ refactored perseus idioms to make more sense

Specifically, template functions are now defined inside the `get_template` function.

* docs(book): 📝 updated docs for current state of features

* fix: 🐛 fixed inconsistencies in paths given to build paths vs incremental

Build paths used to get locale as well in path, not anymore.

* chore: 🙈 ignored testing deployments

* fix: 🐛 fixed content being interpolated in head in production

Just a missing `.head.html` rather than `.html`.
* `StringResult`/`StringResultWithCause` are replaced by `RenderFnResult`/`RenderFnResultWithCause`

* fix: 🐛 fixed newlines/tabs in initial state causing serialization errors

We're now using JS raw strings, escaping as necessary, and then escaping control characters in the shell.

* docs(book): 📝 updated docs fro new error systems

### Features

* ✨ added deployment ([#37](https://github.com/framesurge/perseus/issues/37)) ([a8989dd](https://github.com/framesurge/perseus/commit/a8989ddba203b4825531419cc29b0e6e0ab61ae0))
* **cli:** ✨ added `--release` mode to cli ([#35](https://github.com/framesurge/perseus/issues/35)) ([f66bbb9](https://github.com/framesurge/perseus/commit/f66bbb9b9ae7030a22bd3f7320a83ef7cfe79f37))
* ✨ switched to new error systems, added `is_server!`, and improved render function return types ([#33](https://github.com/framesurge/perseus/issues/33)) ([53bb61e](https://github.com/framesurge/perseus/commit/53bb61e6b9595f7746d0454355569ba79082b069))


### Code Refactorings

* **cli:** ♻️ migrated cli to `clap` ([#34](https://github.com/framesurge/perseus/issues/34)) ([83e365c](https://github.com/framesurge/perseus/commit/83e365c37cfa19a39edcc69562833052edfe8f1c))


### Documentation Changes

* **book:** 📝 added docs for v0.3.x and deprecated v0.2.x ([b2e3c57](https://github.com/framesurge/perseus/commit/b2e3c57cb0da5a58141500a876e32542be49adb6))
* **book:** 📝 added migration page for upgrading from v0.2.x ([df00cf3](https://github.com/framesurge/perseus/commit/df00cf388b95c9705c487b97c0e6e14fa3e445b7))
* **book:** 📝 updated latest stable version of docs ([ab19e78](https://github.com/framesurge/perseus/commit/ab19e7883e9c57b55e9b780ea292aa10c6bd2763))

</details>

### [0.2.3](https://github.com/framesurge/perseus/compare/v0.2.2...v0.2.3) (2021-09-26)


### Features

* **templates:** ✨ added context to templates if they're beeing rendered on the server or client ([7600c95](https://github.com/framesurge/perseus/commit/7600c95b6f7e10574b4597bda268cb0391810c99)), closes [#26](https://github.com/framesurge/perseus/issues/26)
* ✨ made initial content container invisible for errors as well ([0150c8d](https://github.com/framesurge/perseus/commit/0150c8d376d39f355ee7c475f0529671e80915d4))
* ✨ made initial content container invisible once content has loaded ([4daa8c2](https://github.com/framesurge/perseus/commit/4daa8c2a4ec912bde118006dd4329cfa69d5a168))
* ✨ renamed `__perseus_content` to `__perseus_content_initial` and made `__perseus_content` a class ([7242d74](https://github.com/framesurge/perseus/commit/7242d74291e447d448640fc249c489515acc3abe))


### Bug Fixes

* 🚑 changed browser-checking logic to not use context ([4cd06c5](https://github.com/framesurge/perseus/commit/4cd06c5a4e9d52fef53d7cbce8dbcee1348d21e9))
* **i18n:** 🐛 used absolute paths in translation macros ([a413e85](https://github.com/framesurge/perseus/commit/a413e85e683fd0dfa0ca0471c565432cec6eef6d))
* 🐛 changed `__perseus_content_rx` to use `id` instead of `class` ([e504f6d](https://github.com/framesurge/perseus/commit/e504f6d15ee4faaac7e34921fa3ef969210cbb38))


### Documentation Changes

* 📝 added docs for styling pitfalls ([66b43e1](https://github.com/framesurge/perseus/commit/66b43e16b14d615c04fb5eb180d4c9530f9ac590)), closes [#28](https://github.com/framesurge/perseus/issues/28)

### [0.2.2](https://github.com/framesurge/perseus/compare/v0.2.1...v0.2.2) (2021-09-25)


### Features

* **templates:** ✨ added ability to set http headers for templates ([#25](https://github.com/framesurge/perseus/issues/25)) ([058d625](https://github.com/framesurge/perseus/commit/058d625575e28460004a6114c6fa6bacedf76515))
* ✨ added static exporting ([#23](https://github.com/framesurge/perseus/issues/23)) ([4838ba4](https://github.com/framesurge/perseus/commit/4838ba43611b0156afa5c84d2454ca6cbbf5f5a1)), closes [#22](https://github.com/framesurge/perseus/issues/22)


### Bug Fixes

* **cli:** 🐛 surrounded url with angular brackets ([7688d7d](https://github.com/framesurge/perseus/commit/7688d7d4ebab0682dbdd1422f7df3feca117a30f)), closes [#24](https://github.com/framesurge/perseus/issues/24)


### Documentation Changes

* 📝 removed duplication in changelog ([0ba3e2c](https://github.com/framesurge/perseus/commit/0ba3e2c698fa880405f9ef930bfee0c227e8c886))
* **book:** 📝 added docs on header modification ([bca6430](https://github.com/framesurge/perseus/commit/bca6430ca0abeb1afdb2d48abfad414be6bf4ef4))
* 📝 added badges to readme ([0441f80](https://github.com/framesurge/perseus/commit/0441f80a2fcd43fd15e94c4baa56bfc9e11f0788))
* 📝 removed unnecessary readme links ([295a7b5](https://github.com/framesurge/perseus/commit/295a7b5c6c8404ef977c3d1924513103d94acd79))

### [0.2.1](https://github.com/framesurge/perseus/compare/v0.2.0...v0.2.1) (2021-09-23)

### Features

-   **testing:** ✨ added testing harness and tests for examples ([#21](https://github.com/framesurge/perseus/issues/21)) ([4cca6f7](https://github.com/framesurge/perseus/commit/4cca6f7403e6c566592468a2d5d0a836c8ec06fa))

### Code Refactorings

-   **routing:** ♻️ refactored to eliminate only remaining js ([dc21490](https://github.com/framesurge/perseus/commit/dc21490d462654ef6fad3abc3cd3e322e0b2bb1f))

### Documentation Changes

-   📝 updated readme to reflect js elimination ([4d5cf2a](https://github.com/framesurge/perseus/commit/4d5cf2add178277446b67b46e599c8a144dd8e8e))
-   **book:** ✏️ fixed typos in the book ([f84cfb0](https://github.com/framesurge/perseus/commit/f84cfb097129f97509ced5c0d9da1a881eb4b53a))

## [0.2.0](https://github.com/framesurge/perseus/compare/v0.1.4...v0.2.0) (2021-09-21)

### ⚠ BREAKING CHANGES

-   renamed `incremental_path_rendering` to `incremental_generation`, and the corresponding template function no longer takes a value
-   actix web integration now takes `static_dirs` and `static_aliases` options
-   `js_init` no longer an option in actix web integration
-   `error_pages` now comes after `templates` and `no_i18n` apps should not define `locales` at all
-   error pages use `Rc`s now, new options for actix web integration, app root must be of `<div>` form
-   **routing:** `define_app!` no longer takes routing paths, just templates
-   **i18n:** templates no longer take translator (access via context instead)
-   **routing:** `define_app!` redesigned, special meaning for `index` template name, app shell takes full templates, `Locales` has new property
-   all `Arc<T>`s are now `Rc<T>`s
-   **i18n:** `Translator` no longer `Serialize`/`Deserialize`
-   **i18n:** `FsTranslationsManager` now takes a vector of locales to initially cache
-   **i18n:** common locales no longer exist
-   all user-facing interfaces take new i18n parameters

### Features

-   **book:** ✨ added versions for book ([bbdcea2](https://github.com/framesurge/perseus/commit/bbdcea24b942a53e1c538cfb79ba63161bff9d4a))
-   **cli:** ✨ added `eject` command ([b747152](https://github.com/framesurge/perseus/commit/b7471522ee167cf798a2a76084ca18d21b1be678)), closes [#14](https://github.com/framesurge/perseus/issues/14)
-   **routing:** ✨ moved subsequent load head generation to server-side ([1e02ca4](https://github.com/framesurge/perseus/commit/1e02ca4e5a753e4de699dfd21d215aa0d996d05c)), closes [#15](https://github.com/framesurge/perseus/issues/15)
-   ✨ added initial load control ([7335418](https://github.com/framesurge/perseus/commit/733541811b5bf5300c46c72c755cb2ef120d9829)), closes [#2](https://github.com/framesurge/perseus/issues/2)
-   ✨ added metadata modification systems ([bb847aa](https://github.com/framesurge/perseus/commit/bb847aaedbaa3cc0bb340bd54a597a1a599230f4)), closes [#2](https://github.com/framesurge/perseus/issues/2) [#13](https://github.com/framesurge/perseus/issues/13)
-   ✨ added support for static content and aliases ([7f38ea7](https://github.com/framesurge/perseus/commit/7f38ea7be28c6b6ae29c8bfb050db81246d67c9f))
-   ✨ improved `define_app!` macro ([8bf6dd5](https://github.com/framesurge/perseus/commit/8bf6dd53a23694270c10f3c913fda2b051638bba))
-   **cli:** ✨ added single-threaded mode for the CLI ([5cb465a](https://github.com/framesurge/perseus/commit/5cb465aab460a2c11db9a89a7290faeb53243be2)), closes [#11](https://github.com/framesurge/perseus/issues/11)
-   **cli:** ✨ parallelized cli stages and removed rollup ([7693ebf](https://github.com/framesurge/perseus/commit/7693ebf524cb5c499bb5ec51ae7ce9f505660e6e)), closes [#7](https://github.com/framesurge/perseus/issues/7) [#9](https://github.com/framesurge/perseus/issues/9)
-   **i18n:** ✨ added dummy translator to support not using i18n ([803b4f6](https://github.com/framesurge/perseus/commit/803b4f6cce0ba55eb050e454d6359e8cf8a962c3))
-   **i18n:** ✨ added fn on translations manager to get string translations ([649a65d](https://github.com/framesurge/perseus/commit/649a65d59f480bd2f0bd18320113b67cb8651d0a))
-   **i18n:** ✨ added i18n to error pages and integrated fluent ([89fa00e](https://github.com/framesurge/perseus/commit/89fa00eeafa55c986cd6cc784e63bf3bbf57a61b))
-   **i18n:** ✨ added locale detection ([b7ad607](https://github.com/framesurge/perseus/commit/b7ad607861340c56bbfd504d6d2880108dbb0116))
-   **i18n:** ✨ added macros for translation and moved translator into context ([cbfe50c](https://github.com/framesurge/perseus/commit/cbfe50c92ecbbbf860d03194fbbe23fa35302750))
-   **i18n:** ✨ added method to get url in same locale as user currently in ([fc8eeaf](https://github.com/framesurge/perseus/commit/fc8eeafe598aaf8d0ba2c9b8e9dd1d0722b23bf8))
-   **i18n:** ✨ added server-side translations caching ([06b5fa4](https://github.com/framesurge/perseus/commit/06b5fa443fe93a01e34d8b803f4b1a6eb25a98b2))
-   **i18n:** ✨ feature-gated translators ([a123f0d](https://github.com/framesurge/perseus/commit/a123f0dc7e0381a10eba9a863938e1a4eedf1eab))
-   **i18n:** ✨ removed concept of common locales ([95b476f](https://github.com/framesurge/perseus/commit/95b476f9b4f34fbff98a10dff18851c833f7e817))
-   **routing:** ✨ added perseus routing systems and simplified app definition ([49aa2b9](https://github.com/framesurge/perseus/commit/49aa2b9d998871101f7fc2ef7c1a9c45d7873b8c))
-   **routing:** ✨ switched to template-based routing ([78688c1](https://github.com/framesurge/perseus/commit/78688c13e840e9d364d61a3173a08ec5c70ae126)), closes [#12](https://github.com/framesurge/perseus/issues/12)
-   ✨ added build artifact purging to cli ([ef0cf76](https://github.com/framesurge/perseus/commit/ef0cf766b15232e68c2d775c84006b22413f87d2))
-   ✨ added i18n ([a4402c0](https://github.com/framesurge/perseus/commit/a4402c04970019b9b965e4aaf6a38edbae2fc4ce))
-   ✨ made cli preserve relative paths in development ([d79f029](https://github.com/framesurge/perseus/commit/d79f029c9fec5acae96194d1eb8de09a60a9157f))

### Bug Fixes

-   🐛 added `$crate` to invocation of `define_app!` ([c2a4560](https://github.com/framesurge/perseus/commit/c2a4560a0bc60b98cb3ea04f49a62a08b3f2b59e))
-   🐛 handled page rendering errors properly at initial load ([3a9f44a](https://github.com/framesurge/perseus/commit/3a9f44a39573ef2eb362f002b176652985aa7966))
-   🐛 removed deliberately inserted error for debugging ([a1fec62](https://github.com/framesurge/perseus/commit/a1fec6216a2f60d14acc54e351c970ab307ee1a1))
-   🔒 disallowed `static_aliases` outside current directory ([08971ca](https://github.com/framesurge/perseus/commit/08971caa5afde082de9e95c333c0f32fe76698a8))
-   **cli:** 🐛 fixed cli `--no-build` option ([9890457](https://github.com/framesurge/perseus/commit/98904572698b60de566a5283d25b868cd3ef2abf))
-   **routing:** 🐛 fixed [#8](https://github.com/framesurge/perseus/issues/8) ([5a787c4](https://github.com/framesurge/perseus/commit/5a787c4965c30a9d9d7ac338dbd8bbf1de39aefd))
-   **routing:** 🐛 fixed error duplication on initial load ([53058ba](https://github.com/framesurge/perseus/commit/53058ba025750e5eb5508c19a40e2977acaeda45))
-   **routing:** 🐛 fixed link handling errors in [#8](https://github.com/framesurge/perseus/issues/8) ([197956b](https://github.com/framesurge/perseus/commit/197956bc734bc1d85f56bcfc7b327bb1ed1f4c07))
-   ✏️ fixed displayed number of steps in cli serving (4 -> 5) ([d1a6bb8](https://github.com/framesurge/perseus/commit/d1a6bb86bef8eeb67f682f2aac719623400dd2e2))
-   ✏️ updated all instances of _WASM_ to _Wasm_ ([f7ec1aa](https://github.com/framesurge/perseus/commit/f7ec1aa9227592e04370dd9c5b85ab577193330b))
-   🐛 used absolute paths in `web_log!` macro ([945bd2a](https://github.com/framesurge/perseus/commit/945bd2a82ff0884df362ec303c38731d9b470ed8))

### Performance Improvements

-   ⚡️ inlined wasm load script to reduce full requests ([6cfe8e1](https://github.com/framesurge/perseus/commit/6cfe8e15d812400c5bff387cffd8a6dd715ce59b))
-   **cli:** ⚡️ created workspace in cli subcrates ([3e11ecd](https://github.com/framesurge/perseus/commit/3e11ecd6da6b618a5b94c5abfc33264e37304482))
-   **i18n:** ⚡️ removed needless translations fetch if not using i18n ([7c6f697](https://github.com/framesurge/perseus/commit/7c6f697dfceff6b93a8ad87d13924510f7174ad7))
-   ⚡️ switched to `Rc<ErrorPages>` to avoid producing unnecessary `ErrorPages` ([6786ff4](https://github.com/framesurge/perseus/commit/6786ff4c6781e020af3bfd6d3306c8f899c11c85))
-   ⚡️ switched to `Rc<T>`s instead of `Arc<T>`s ([8d70599](https://github.com/framesurge/perseus/commit/8d70599f803c22ff4a7eaa03b074480d0b5b6e74))

### Code Refactorings

-   ♻️ cleaned up macros ([30345f0](https://github.com/framesurge/perseus/commit/30345f085f7183e85d3acf3be3c0d4ce7f92790a))
-   ♻️ renamed `incremental_path_rendering` to `incremental_generation` and improved interface ([cb60be0](https://github.com/framesurge/perseus/commit/cb60be025039d4808aeb8429ed67a885625b117e))
-   ♻️ rewrote `showcase` example to use cli ([c2f1091](https://github.com/framesurge/perseus/commit/c2f109157f5f3848c195ef6f55373b34f24e67b7))
-   🎨 cleaned a few things up ([0ab791f](https://github.com/framesurge/perseus/commit/0ab791fb8bc4cf8e7f07e19cc4f3e2420f4230d2))
-   🔥 removed unnecessary `X-UA-Compatible` headers ([73643b8](https://github.com/framesurge/perseus/commit/73643b8c54091533790a09e54d2c53e3b5f62a9b))
-   **i18n:** 🚚 refactored to prepare for future multi-translator support ([24f4362](https://github.com/framesurge/perseus/commit/24f4362c6abeb4b72ef499f32edc6349fda5891d))

### Documentation Changes

-   **book:** 📝 added docs on migrating from 0.1.x ([056fb58](https://github.com/framesurge/perseus/commit/056fb5830d848510a00f42dd69f304145d364429))
-   **book:** 📝 added full intro to perseus ([424e3f4](https://github.com/framesurge/perseus/commit/424e3f4a5b1bb0a8fb11c7c23e4337b8ff35a982))
-   **book:** 📝 added hello world and second app tutorials to book ([58eb92d](https://github.com/framesurge/perseus/commit/58eb92db00608736cb8ebfc795cd568a053288b4))
-   **book:** 📝 finished docs for v0.2.x ([c7d3ea2](https://github.com/framesurge/perseus/commit/c7d3ea25862fbb9f8a1bad84bb6d866b5cd6cbdd))
-   **book:** 📝 fixed relative paths in docs and added docs about `StringResultWithCause<T>` ([39b3ce1](https://github.com/framesurge/perseus/commit/39b3ce197580bf430afd5140867e5632dcc081fc))
-   **book:** 📝 wrote advanced docs on routing ([31497ab](https://github.com/framesurge/perseus/commit/31497ab26de444c2d32c9903326ecea0d1172a60))
-   **book:** 📝 wrote book initial reference sections ([f7f7892](https://github.com/framesurge/perseus/commit/f7f7892fbf124a7d887b1f22a1641c79773d6246))
-   **book:** 📝 wrote cli docs ([e321c38](https://github.com/framesurge/perseus/commit/e321c389c17b93675bca1bc93eacaf1ba4da30aa))
-   **book:** 📝 wrote docs for i18n, error pages, and static content ([0375f01](https://github.com/framesurge/perseus/commit/0375f013e0f02778829b5ec8903a10ecfbe4d127))
-   **book:** 📝 wrote large parts of advanced docs and some other pages ([d8fd43f](https://github.com/framesurge/perseus/commit/d8fd43f75385c72a17627cc0d5f71c4496d95c42))
-   **book:** 🔖 released v0.2.x docs ([3cd80d0](https://github.com/framesurge/perseus/commit/3cd80d0fb2f0ae2e5fbb14295f37181f4778161b))
-   ✏️ fixed some typos and clarified things in readmes ([5c59ae6](https://github.com/framesurge/perseus/commit/5c59ae6855aa22874314abccdc968cb58345ffba))
-   💡 removed duplicate link typo in comment ([379d549](https://github.com/framesurge/perseus/commit/379d549b31d3929dc383cb852c623f39e91c0201))
-   💡 removed entirely useless comment in showcase example ([2105f5a](https://github.com/framesurge/perseus/commit/2105f5a79061ecbc871aa489db644e62e3d52692))
-   📝 added explanation for 0.1% js to readme ([6f0bd08](https://github.com/framesurge/perseus/commit/6f0bd088af2bed928ba95f963c3defa20eef3460))
-   📝 cleaned up docs ([b6a6b72](https://github.com/framesurge/perseus/commit/b6a6b72b7b47937f9d60306524d75678154255fc))
-   **book:** 🚑 updated versions of sycamore in book ([e41d3e5](https://github.com/framesurge/perseus/commit/e41d3e5a3173979548adee165453a73e60d99173))
-   **examples:** ✨ added new `tiny` example and updated readme with it ([2c2d06b](https://github.com/framesurge/perseus/commit/2c2d06b3ee8cdc49614c42ee2a82c923af131be6))
-   **examples:** 🚚 merged basic/cli examples and cleaned up examples ([db6fbdd](https://github.com/framesurge/perseus/commit/db6fbdd4047044acff51a1cc3e6564661fe22016))
-   📝 updated roadmap in readme ([c3ad018](https://github.com/framesurge/perseus/commit/c3ad0185b40df84efef10862f9fb150e6610bd2f))
-   📝 wrote tutorial on building first app ([19f0458](https://github.com/framesurge/perseus/commit/19f045840e1cf6e9191aaaf3e98d15b5a98d8370))

### [0.1.4](https://github.com/framesurge/perseus/compare/v0.1.3...v0.1.4) (2021-09-11)

### Bug Fixes

-   🐛 updated `basic` example perseus version ([1d8d895](https://github.com/framesurge/perseus/commit/1d8d895a0c6ed5d9cb96a14d06c702917c3837c1))
-   🚑 allowed env var specification of command paths in building/serving ([5a2e494](https://github.com/framesurge/perseus/commit/5a2e49475a9e6ef1e1d25491530f8be9b22f74f5))

### [0.1.3](https://github.com/framesurge/perseus/compare/v0.1.2...v0.1.3) (2021-09-11)

### Bug Fixes

-   🚑 commands now executed in shells ([80604a4](https://github.com/framesurge/perseus/commit/80604a4b1323ec322e875bb6bdc7e05b4768b1a6))
-   🚑 fixed windows cli bug ([1b6ef16](https://github.com/framesurge/perseus/commit/1b6ef164ebf6a8c9f3c2f9c27488d181b0760b36))

### [0.1.2](https://github.com/framesurge/perseus/compare/v0.1.1...v0.1.2) (2021-09-03)

### Bug Fixes

-   🐛 fixed cli executable name ([573fc2f](https://github.com/framesurge/perseus/commit/573fc2f962039d91fb08e49a162d4972a7a935df))

### Documentation Changes

-   📝 added crate docs for `perseus-actix-web` ([f5036e7](https://github.com/framesurge/perseus/commit/f5036e756ce789812e08752b1e7e31b0c70d4c1c))
-   📝 added crate docs for `perseus` package ([61ca6c0](https://github.com/framesurge/perseus/commit/61ca6c080931b5a67e82403e0c32de5934e8781d))
-   📝 added crate documentation for `perseus-cli` and fixed doc typos ([b3ec9ac](https://github.com/framesurge/perseus/commit/b3ec9aca0a5f08fb91d411f54964e4a02ffa2066))
-   📝 updated readme with contact links ([a2bc5f2](https://github.com/framesurge/perseus/commit/a2bc5f271263d5ed85618b818d5e27d1d2dde191))

### [0.1.1](https://github.com/framesurge/perseus/compare/v0.1.0...v0.1.1) (2021-09-03)

### Bug Fixes

-   🐛 added version numbers for local package imports ([b700cf7](https://github.com/framesurge/perseus/commit/b700cf72325b54a987c87415de3f119273690650))
-   🐛 fixed cli packaging issues ([dd43e81](https://github.com/framesurge/perseus/commit/dd43e8132d9b6cde82874883291c79e6d1ba6676))

## 0.1.0 (2021-09-02)

### Features

-   ✨ added access to request data in ssr ([02ce425](https://github.com/framesurge/perseus/commit/02ce42573ff5cf6f279c3932b68901bfd48922dc))
-   ✨ added actix-web integration ([0e0f2f1](https://github.com/framesurge/perseus/commit/0e0f2f19463c9f04ea7d886e3d41672ab74bfb17))
-   ✨ added basic cli ([5e7a867](https://github.com/framesurge/perseus/commit/5e7a867965f93ec16128e2b07cae91dc7d8b907e))
-   ✨ added basic sycamore ssg systems ([c8530cf](https://github.com/framesurge/perseus/commit/c8530cf47afcc45585ac346e3e717f516361ca7e))
-   ✨ added build command to cli ([66dc282](https://github.com/framesurge/perseus/commit/66dc28273d17d6e763aac52da8d23c9595c8deab))
-   ✨ added isr ([5baf9bf](https://github.com/framesurge/perseus/commit/5baf9bf0eb92031f4e5fee0158403ada376f4bf3))
-   ✨ added page path matching logic ([734f9df](https://github.com/framesurge/perseus/commit/734f9df6c7f84902c9a3975bf3138f6442a08697))
-   ✨ added request conversion logic for actix web ([71a5445](https://github.com/framesurge/perseus/commit/71a54454bfeaf537bae4bbce639d513f02be88be))
-   ✨ added revalidation and refactored a fully modular rendering system ([c9df616](https://github.com/framesurge/perseus/commit/c9df616983d3ef240ea63059eb1fa45b8e92f1d4))
-   ✨ added serving systems to cli ([335ff5d](https://github.com/framesurge/perseus/commit/335ff5d7d3f61cf8aea90b9d9e4071b5c0739701))
-   ✨ added ssr ([ac79996](https://github.com/framesurge/perseus/commit/ac799966a684595d4a28750a043a1ae172fad527))
-   ✨ added template method to define function for amalgamating states ([1cb4356](https://github.com/framesurge/perseus/commit/1cb435663a09a78c9444ef05a2bbf7e5a15a1e99))
-   ✨ allowed user render functions to return errors ([fa50d4c](https://github.com/framesurge/perseus/commit/fa50d4cd1e05470386dc3aad0020f21970c62a80))
-   ✨ built subcrate tro underlie cli functionality ([1e7e355](https://github.com/framesurge/perseus/commit/1e7e3551f229504ef92077f8047710b7d502a2d8))
-   ✨ made config managers async ([5e03cad](https://github.com/framesurge/perseus/commit/5e03cad26b3164d5c831adfe187240fa5ddb73dc))
-   ✨ made rendering functions asynchronous ([5b403b2](https://github.com/framesurge/perseus/commit/5b403b2d5181256d0aaf0f23f880fc8d5aade0c8))
-   ✨ props now passed around as strings ([7a334cf](https://github.com/framesurge/perseus/commit/7a334cf39a76230a9cc3ca3c797768a182a8bdc5))
-   ✨ re-exported sycamore `GenericNode` ([8b79be8](https://github.com/framesurge/perseus/commit/8b79be86c9deb941f3d743abfac12c31d0c0db8e))
-   ✨ refactored examples and created preparation system in cli ([8aa3d0f](https://github.com/framesurge/perseus/commit/8aa3d0f9db5020f4befcb5845ac3a851cb40c8c5))
-   ✨ set up cli systems for preparation and directory cleaning ([36660f8](https://github.com/framesurge/perseus/commit/36660f899d0dc2dd389173b1299de36f4fa3c8dc))
-   🎉 added readme and license ([0306a10](https://github.com/framesurge/perseus/commit/0306a10da1bcffcc4d2426da365c76a465795ab4))
-   🥅 set up proper error handling ([7ea3ec0](https://github.com/framesurge/perseus/commit/7ea3ec0c3fa59b1e1e028cba45217ddd9e3320ce))

### Bug Fixes

-   🐛 allowed build state to return `ErrorCause` for incremental generation ([dd4d60f](https://github.com/framesurge/perseus/commit/dd4d60ff9f925b592c4359ae7e76f0a9eee1a752))
-   🐛 fixed inconsistent path prefixing in `build_state` calls ([96066d0](https://github.com/framesurge/perseus/commit/96066d0019f2e68c79349886a4af1f5f37248c62))
-   🐛 fixed recursive extraction and excluded subcrates from workspaces ([c745cf2](https://github.com/framesurge/perseus/commit/c745cf2b4381918c821accc351dbff368fd453a1))
-   🐛 removed old debug log ([ed4f43a](https://github.com/framesurge/perseus/commit/ed4f43a75550faa781c261edf6caafd688f32961))
-   🐛 used config manager instead of raw fs in `get_render_cfg()` ([e75de5a](https://github.com/framesurge/perseus/commit/e75de5a1bcdd48f67a288e0fb89bde0a6e959a83))

### Code Refactorings

-   ♻️ changed `define_app!`'s `router` to use curly brackets ([d5519b9](https://github.com/framesurge/perseus/commit/d5519b9fb6c4e3909248acabeb8088d853468c6c))
-   ♻️ created sane library interface ([51284a8](https://github.com/framesurge/perseus/commit/51284a86bf5e33730768cc3946af3d2ac848b695))
-   ♻️ moved logic into core package from example ([b2e9a68](https://github.com/framesurge/perseus/commit/b2e9a683211c798c6254e2ae328f97d37bec5d29))
-   ♻️ removed useless render options system ([1af26dc](https://github.com/framesurge/perseus/commit/1af26dcf78b95b57a45c2b086e234d21a5932763))
-   🚚 moved everything into packages ([dcbabc0](https://github.com/framesurge/perseus/commit/dcbabc0c4c504911c13da166992bcbe072ca163d))
-   🚚 renamed pages to templates for clarity ([7c9e433](https://github.com/framesurge/perseus/commit/7c9e4337f06412c739e050d3bbfd3d6c4d56f69c))

### Documentation Changes

-   💡 removed old todos ([9464ee5](https://github.com/framesurge/perseus/commit/9464ee5f854c9f81840acf4a32a8707c5e926ca5))
-   📝 added docs for cli ([e4f9cce](https://github.com/framesurge/perseus/commit/e4f9cce19cadd9af91aea47f02d47aebddbc1014))
-   📝 added documentation for actix-web integration ([1877c13](https://github.com/framesurge/perseus/commit/1877c130a3fb4c6e6e593ba439d818fc24121c17))
-   📝 added example of state amalgamation ([cd93fdc](https://github.com/framesurge/perseus/commit/cd93fdca3d5ab9f96af5c3d846c69fa68d94b3ac))
-   📝 added link to percy in readme ([2072b9b](https://github.com/framesurge/perseus/commit/2072b9b5537e2058d05c09cc0386931995753906))
-   📝 added repo docs ([043b65f](https://github.com/framesurge/perseus/commit/043b65f8b5094e4207c4304968c4863feb08e42c))
-   📝 added scaffold for basic tutorial docs ([23fd0a6](https://github.com/framesurge/perseus/commit/23fd0a6c087402a7c5aec0d60a9181d37f519b3c))
-   📝 fixed syntax highlighting in cli docs ([3242409](https://github.com/framesurge/perseus/commit/32424094363a8112d0cbfa6ddad7321938b93b12))
-   📝 updated docs for v0.1.0 ([bf931e4](https://github.com/framesurge/perseus/commit/bf931e4909b398f94b70ad37994497e1f9cab4ca))
-   📝 updated readme for significant dependency changes ([1d424b5](https://github.com/framesurge/perseus/commit/1d424b55065f520f967001db45bc81630ba3aa43))
-   📝 wrote large sections of the book ([a548531](https://github.com/framesurge/perseus/commit/a548531f882750699bca73f9db54741854dc9ef3))
