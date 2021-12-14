# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.3.0-beta.23](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.22...v0.3.0-beta.23) (2021-12-14)


### Bug Fixes

* fixed placement of `standalone` feature in deployment command ([7609ee1](https://github.com/arctic-hen7/perseus/commit/7609ee1ca5c36ec02d195e384e102e3163e7ecc4)), closes [#92](https://github.com/arctic-hen7/perseus/issues/92)


### Documentation Changes

* add `-r` flag to `entr` commands ([#93](https://github.com/arctic-hen7/perseus/issues/93)) ([d0b863e](https://github.com/arctic-hen7/perseus/commit/d0b863e07ddf00166e5002807dcfe76bf96f9a72))

## [0.3.0-beta.22](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.21...v0.3.0-beta.22) (2021-12-13)


### ⚠ BREAKING CHANGES

* upgraded to Sycamore v0.7.0 (see [their changelog](https://github.com/sycamore-rs/sycamore/blob/master/CHANGELOG.md))

### Features

* **cli:** added flag to set server integration to use ([b71fa41](https://github.com/arctic-hen7/perseus/commit/b71fa4134564277973effb77cc4a05bf1a4d6d46))
* removed `PERSEUS_STANDALONE` ([d178f5a](https://github.com/arctic-hen7/perseus/commit/d178f5aaaa80f8c89962b5b41693d696863aa922)), closes [#87](https://github.com/arctic-hen7/perseus/issues/87)
* upgraded to sycamore v0.7.0 ([3989241](https://github.com/arctic-hen7/perseus/commit/3989241bb94a62005819ed652b4a15764867b8f8))


### Bug Fixes

* added missing `cfg` macro line ([006523a](https://github.com/arctic-hen7/perseus/commit/006523a26922a86aba830a4dba895829bb71dc3d))
* fixed error page duplication without hydration ([7b3e62f](https://github.com/arctic-hen7/perseus/commit/7b3e62f335f908d72c0de62f4d82592e38ca67ec))
* **deps:** upgraded to `actix-web` v4.0.0-beta.14 ([139d309](https://github.com/arctic-hen7/perseus/commit/139d309997e15146e9277c6f617c88c67d065049))


### Documentation Changes

* added a few more known bugs ([6bae07c](https://github.com/arctic-hen7/perseus/commit/6bae07cf56a5e9d4427a9a4331b32d5c6d23a6cc))
* cleaned up and added page on publishing plugins ([37acece](https://github.com/arctic-hen7/perseus/commit/37acece139f6da9a59e8e3aea0cf039aeafe6b1c))
* merged `next` and `0.3.x` ([dbb47fb](https://github.com/arctic-hen7/perseus/commit/dbb47fb8677e8fb297102a7ed49de59de206194f))
* updated docs for sycamore v0.7.0 ([e840734](https://github.com/arctic-hen7/perseus/commit/e840734c3907ee510f02b611cab15999870336bd))

## [0.3.0-beta.21](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.20...v0.3.0-beta.21) (2021-12-12)


### Bug Fixes

* switched to using `warp-fix-171` ([f3f0a43](https://github.com/arctic-hen7/perseus/commit/f3f0a43d3dc5e757e3e476218e588d6c1ad70ded))

## [0.3.0-beta.20](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.19...v0.3.0-beta.20) (2021-12-12)


### Bug Fixes

* made cli update local dependencies properly ([3067071](https://github.com/arctic-hen7/perseus/commit/30670715ed3f8e53c6527d96b54e92fe5b6c8173))

## [0.3.0-beta.19](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.18...v0.3.0-beta.19) (2021-12-12)


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

* add warp integration ([#86](https://github.com/arctic-hen7/perseus/issues/86)) ([6adf264](https://github.com/arctic-hen7/perseus/commit/6adf264c7474ec1f8bc71fe37e08c2bf132986dd)), closes [#85](https://github.com/arctic-hen7/perseus/issues/85)

## [0.3.0-beta.18](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.17...v0.3.0-beta.18) (2021-11-28)


### Features

* **website:** made docs sidebar nicer ([107b9d3](https://github.com/arctic-hen7/perseus/commit/107b9d3264fb30602c672d359eb187d9b4c58f08))
* added `perseus snoop` and docs for common pitfalls ([3c1a919](https://github.com/arctic-hen7/perseus/commit/3c1a919f074a99423f26f54a3761e3468b13d6d0))
* **i18n:** added fallback non-wasm locale redirection ([589ac1b](https://github.com/arctic-hen7/perseus/commit/589ac1b85f4a035dec36aa19c92a0d2157cea71e))
* **website:** added plugins registry ([de1c217](https://github.com/arctic-hen7/perseus/commit/de1c217f1073206bee5e493ca9571325735d0e71))


### Bug Fixes

* **cli:** 🐛 printed `stdout` and well as `stderr` if a stage fails ([ea1f1f1](https://github.com/arctic-hen7/perseus/commit/ea1f1f1f1ca9e45927eacfbbff6e8cd844f40221)), closes [#74](https://github.com/arctic-hen7/perseus/issues/74)
* **exporting:** 🐛 fixed [#73](https://github.com/arctic-hen7/perseus/issues/73) ([a3f879c](https://github.com/arctic-hen7/perseus/commit/a3f879c20eb2bcfc4592cb41ff0e9052a98d4f84))
* **i18n:** fixed fallback locale redirection with relative paths ([5095388](https://github.com/arctic-hen7/perseus/commit/5095388a275332af5069ef6e4fc94a9ad51b37aa))


### Documentation Changes

* **website:** added more comparisons ([d4dabaf](https://github.com/arctic-hen7/perseus/commit/d4dabaf1a7f4e8396fdecee1dfc03ab9fe99cee5))
* made markdown styles more readable and fixed tldr link ([a74b285](https://github.com/arctic-hen7/perseus/commit/a74b2858155706cef6ed83e118062beb40b9f35d))
* **book:** fixed dependency versions in docs ([2171e9c](https://github.com/arctic-hen7/perseus/commit/2171e9c196671a5aa10bffda1413eb9da566a1cf)), closes [#79](https://github.com/arctic-hen7/perseus/issues/79)
* **readme:** updated contact links ([5f74b07](https://github.com/arctic-hen7/perseus/commit/5f74b07ec0c53851e904e5782e37266b33083f92)), closes [#77](https://github.com/arctic-hen7/perseus/issues/77)
* ✏️ fixed typos in contributing guidelines ([#76](https://github.com/arctic-hen7/perseus/issues/76)) ([5dfedc1](https://github.com/arctic-hen7/perseus/commit/5dfedc16864718837be1a273fe0b28b1d1e24e46))

## [0.3.0-beta.17](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.16...v0.3.0-beta.17) (2021-11-07)


### Bug Fixes

* **cli:** 🐛 created parent directories with CLI ([#72](https://github.com/arctic-hen7/perseus/issues/72)) ([6dc0aab](https://github.com/arctic-hen7/perseus/commit/6dc0aabaad88df9cb32a72e24f91b31cc7aaefd3)), closes [#69](https://github.com/arctic-hen7/perseus/issues/69)


### Code Refactorings

* **website:** ♻️ refactored website to use new ergonomics macros ([bb879c6](https://github.com/arctic-hen7/perseus/commit/bb879c6476fb68336f0e4afb2d56783cc559f201))

## [0.3.0-beta.16](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.15...v0.3.0-beta.16) (2021-11-04)


### Features

* **templates:** ✨ added `autoserde` macro to improve ergonomics ([eb21299](https://github.com/arctic-hen7/perseus/commit/eb212996192749ba3cb370a239ffe0f31a6707e8)), closes [#57](https://github.com/arctic-hen7/perseus/issues/57)
* **templates:** ✨ added `blame_err!` convenience macro ([6ab178a](https://github.com/arctic-hen7/perseus/commit/6ab178a54a95e5a64b918556c803b8f91ce306a6))
* **templates:** ✨ added `head` ergonomics macro ([fb17e03](https://github.com/arctic-hen7/perseus/commit/fb17e03ce614f94e4d84ed7c6aa1ce6bb99a3025)), closes [#57](https://github.com/arctic-hen7/perseus/issues/57)
* **templates:** ✨ added `template` macro to automate template fn creation ([810ae1b](https://github.com/arctic-hen7/perseus/commit/810ae1b1fb17ce52892454cdbbdd5215ae4b3861)), closes [#57](https://github.com/arctic-hen7/perseus/issues/57)
* **website:** ✨ re-added size optimizations plugin to website ([4364d99](https://github.com/arctic-hen7/perseus/commit/4364d99f94ed3f25c13989c2d7ccd020adbafd36))


### Bug Fixes

* **cli:** 🐛 removed distribution artifacts from cli subcrates ([ebca95c](https://github.com/arctic-hen7/perseus/commit/ebca95c7fcb629a5fc8ff1cf5445424553fc0012))
* **examples:** 🐛 fixed type mismatch in `showcase` example ([7a3dd63](https://github.com/arctic-hen7/perseus/commit/7a3dd630b6aae7168a24aff2f167af4b9d552eac))


### Documentation Changes

* **book:** 🐛 fixed broken amalgamation page link ([1966fd1](https://github.com/arctic-hen7/perseus/commit/1966fd1b176e6e98693f25fc06e6063f9274add9))
* **book:** 📝 added docs for new ergonomics macros ([0c4f3b2](https://github.com/arctic-hen7/perseus/commit/0c4f3b22e069020b3c8bc5940252f58b93fae1a0))
* **book:** 📝 updated `next` from `0.3.x` ([7f8e2f2](https://github.com/arctic-hen7/perseus/commit/7f8e2f2af3f8f1d3a8f2e578f1df8b6b8b0031c9))

## [0.3.0-beta.15](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.14...v0.3.0-beta.15) (2021-10-30)


### Features

* **plugins:** ✨ added client privileged plugins ([686f369](https://github.com/arctic-hen7/perseus/commit/686f369ca211030566db78295fe19f72ba300f58))


### Code Refactorings

* **website:** 👽️ updated website for 0.3.0-beta.14 ([71b6f42](https://github.com/arctic-hen7/perseus/commit/71b6f42c43faf0f1203ef80279c8e64b6e25de07))


### Documentation Changes

* **book:** 📝 updated docs for plugins system changes ([a85f150](https://github.com/arctic-hen7/perseus/commit/a85f15020e5c344f0a0c821c92473644b42ad405))

## [0.3.0-beta.14](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.13...v0.3.0-beta.14) (2021-10-28)


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

* ✨ trim bundle sizes with feature-gating ([#68](https://github.com/arctic-hen7/perseus/issues/68)) ([ffea205](https://github.com/arctic-hen7/perseus/commit/ffea205d3e0353800db6468c17b7aa857734cd45))
* **website:** ✨ added size optimizations plugin to website ([60e2658](https://github.com/arctic-hen7/perseus/commit/60e265896e7b9fbfeffb459336b038cb1b491550)), closes [#66](https://github.com/arctic-hen7/perseus/issues/66)


### Code Refactorings

* **i18n:** ♻️ fixed clippy warnings and removed an unused import ([c831fe1](https://github.com/arctic-hen7/perseus/commit/c831fe10c400f1b64ef8fe4463f0fbdbd25129ce))


### Documentation Changes

* **book:** 📝 updated docs for size optimizations plugin ([7b2ff84](https://github.com/arctic-hen7/perseus/commit/7b2ff84b28bc3c99ca401c39d4edc6ee0d4f2321))

## [0.3.0-beta.13](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.12...v0.3.0-beta.13) (2021-10-18)


### Bug Fixes

* 🚑️ upgraded clap to fix compile errors ([aed12bc](https://github.com/arctic-hen7/perseus/commit/aed12bc44178577d0a60b8cfbb1d78df8fa7cdec))

## [0.3.0-beta.12](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.11...v0.3.0-beta.12) (2021-10-17)


### Bug Fixes

* **plugins:** 🐛 fixed `perseus tinker` deleting `.perseus/` without recreating it ([0e9bed5](https://github.com/arctic-hen7/perseus/commit/0e9bed5fa2ee2f918391167eaeb795d50811c496))


### Documentation Changes

* **book:** ✏️ fixed typos in intro ([#53](https://github.com/arctic-hen7/perseus/issues/53)) ([1aff29c](https://github.com/arctic-hen7/perseus/commit/1aff29c8c6aab21da96a61a77fcdb58d419179cf))
* 📝 added docs for contributing to the docs ([7a211eb](https://github.com/arctic-hen7/perseus/commit/7a211ebf5d34354877177dd75fffacf91efff9a5))

## [0.3.0-beta.11](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.10...v0.3.0-beta.11) (2021-10-16)


### Bug Fixes

* 🐛 fixed naive current directory handling for standalone deployment binary ([e9e24da](https://github.com/arctic-hen7/perseus/commit/e9e24dad1e70807bf0694a729e619035e8810b3a)), closes [#63](https://github.com/arctic-hen7/perseus/issues/63)

## [0.3.0-beta.10](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.9...v0.3.0-beta.10) (2021-10-16)


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

* ✨ add plugins system ([#62](https://github.com/arctic-hen7/perseus/issues/62)) ([ca0aaa2](https://github.com/arctic-hen7/perseus/commit/ca0aaa2cd9cd5c22eb653af820c0e437fa4d9f2b))


### Documentation Changes

* **book:** 📝 merged `next` docs with 0.3.x docs for plugins ([c1e8033](https://github.com/arctic-hen7/perseus/commit/c1e8033687b1aaa5efecefe0502467d2b8ce6694))

## [0.3.0-beta.9](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.8...v0.3.0-beta.9) (2021-10-12)


### ⚠ BREAKING CHANGES

* `Rc`s are eliminated and done behind the scenes

### Features

* ✨ removed `Rc`s completely ([d02189b](https://github.com/arctic-hen7/perseus/commit/d02189bc4b0fbec0ddb96ade8fa87275f39f3042))
* **website:** ✨ added comparisons page ([#56](https://github.com/arctic-hen7/perseus/issues/56)) ([61dac01](https://github.com/arctic-hen7/perseus/commit/61dac01b838df23cc0f33b0d65fcb7bf5f252770))
* **website:** ✨ added proper docs links parsing system ([cfa2d60](https://github.com/arctic-hen7/perseus/commit/cfa2d6025e624cf658236bbdc80b8d6470085c6d))


### Bug Fixes

* **i18n:** 🐛 fixed `link!` macro with base path ([d676471](https://github.com/arctic-hen7/perseus/commit/d676471f28608618e7693583f5a0e8bd9bf29805))
* **i18n:** 🐛 fixed locale redirection `//` ([488a9a0](https://github.com/arctic-hen7/perseus/commit/488a9a081429805e25a6415366cd464ee1234fd4))
* **website:** 🐛 fetched examples from git so they don't go obsolete in older versions ([5608a6a](https://github.com/arctic-hen7/perseus/commit/5608a6ad2486909091b067e144607c6a39c56075)), closes [#60](https://github.com/arctic-hen7/perseus/issues/60)
* **website:** 🐛 fixed links in docs version warnings ([295b875](https://github.com/arctic-hen7/perseus/commit/295b8757283a407e321565ae1c15ee4d98ef9125))
* **website:** 🚑️ pinned website to sycamore v0.6.1 to prevent base path problems ([71a142d](https://github.com/arctic-hen7/perseus/commit/71a142dc2496ee020447cda1dde9380365386e68)), closes [#60](https://github.com/arctic-hen7/perseus/issues/60)


### Documentation Changes

* 📝 removed warning about [#60](https://github.com/arctic-hen7/perseus/issues/60) from readme ([4ed3783](https://github.com/arctic-hen7/perseus/commit/4ed37835b79298fc9d07957810ff9efd5fa76794))
* **book:** 📝 merged 0.3.x and next versions of docs ([9a4a956](https://github.com/arctic-hen7/perseus/commit/9a4a9565172afe96ebcaf8e44f9362e09e453d33))
* **book:** 📝 updated docs and added new information on a few things ([8169153](https://github.com/arctic-hen7/perseus/commit/816915333b51b8df21841adbf294462c10c6e3a8)), closes [#46](https://github.com/arctic-hen7/perseus/issues/46)
* **book:** 📝 updated links in docs ([c5398a3](https://github.com/arctic-hen7/perseus/commit/c5398a3b231786d771020532912ef7f80b7e4ac9))
* 📝 removed warning about book being down ([1cb9ec6](https://github.com/arctic-hen7/perseus/commit/1cb9ec6ab4cb76bc144a680bb1d21ff5f1c3c2d2))
* **website:** 📝 mention `browser-sync` as dependency for working with website ([#55](https://github.com/arctic-hen7/perseus/issues/55)) ([a97c325](https://github.com/arctic-hen7/perseus/commit/a97c3251f446c40655edba8d795875a88805fd92))

## [0.3.0-beta.8](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.7...v0.3.0-beta.8) (2021-10-08)


### Bug Fixes

* **i18n:** 🐛 fixed path prefixing with locale redirection ([241741f](https://github.com/arctic-hen7/perseus/commit/241741ff3055665f5721635d08b5770910f74add))
* **i18n:** 🐛 made locale redirection work without trailing forward slash ([90b3a99](https://github.com/arctic-hen7/perseus/commit/90b3a990c19baafb763422575a1ef188baacf495))
* **templates:** 🐛 inserted `<base>` element at top of `<head>` ([25959d7](https://github.com/arctic-hen7/perseus/commit/25959d79cf8ab40764100b9ababbe4ede8ededad))
* **website:** 🐛 fixed absolute path links in website ([221fa24](https://github.com/arctic-hen7/perseus/commit/221fa24e48f7374c427256c5d9ab6884d68755e3))
* **website:** 🐛 fixed index page styling on non-firefox browsers ([#54](https://github.com/arctic-hen7/perseus/issues/54)) ([aced234](https://github.com/arctic-hen7/perseus/commit/aced2346fdce10ff0c16daf5c95e73de7120cac4))
* **website:** 🐛 fixed website links ([54de491](https://github.com/arctic-hen7/perseus/commit/54de49130ec253ab61d6217a60379d2fa0eedd97))
* **website:** 💄 made github button same size as get started button on index page ([c472e04](https://github.com/arctic-hen7/perseus/commit/c472e04a0d29615909a49248179ca8b27cdb0f60)), closes [#54](https://github.com/arctic-hen7/perseus/issues/54)


### Performance Improvements

* **website:** ⚡️ added size optimizations on website ([31fb1f8](https://github.com/arctic-hen7/perseus/commit/31fb1f84a0b21f4f5a3da646cd396f58f6dd4c37))


### Code Refactorings

* **website:** ♻️ updated website routes for path prefixing ([28bba42](https://github.com/arctic-hen7/perseus/commit/28bba423a75329f9610f7b61ee7e846e266c3d52))

## [0.3.0-beta.7](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.6...v0.3.0-beta.7) (2021-10-06)


### ⚠ BREAKING CHANGES

* **routing:** multiple *internal* function signatures accept exxtra parameter for path prefix

### Features

* **routing:** ✨ added support for relative path hosting with `PERSEUS_BASE_PATH` environment variable ([b7d6eb6](https://github.com/arctic-hen7/perseus/commit/b7d6eb680d3a4368b6d74bfe748fa70207436107)), closes [#48](https://github.com/arctic-hen7/perseus/issues/48)
* ✨ added website ([#47](https://github.com/arctic-hen7/perseus/issues/47)) ([45a0f6c](https://github.com/arctic-hen7/perseus/commit/45a0f6c327fc9386ca31dd6f305cdb387dda5ce0)), closes [#46](https://github.com/arctic-hen7/perseus/issues/46)


### Bug Fixes

* **routing:** 🐛 made back button work with locale redirection ([cf60c12](https://github.com/arctic-hen7/perseus/commit/cf60c123600a1dad936fb0ed0b4855d903ee25a3)), closes [#50](https://github.com/arctic-hen7/perseus/issues/50)


### Documentation Changes

* **book:** 📝 added docs for relative path deployment ([1ecc94f](https://github.com/arctic-hen7/perseus/commit/1ecc94f5fd6a8399fc8ae13e931968c7d1df05b3))

## [0.3.0-beta.6](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.5...v0.3.0-beta.6) (2021-10-02)


### Bug Fixes

* **exporting:** 🚑 fixed partial flattening in exporting ([bdbdc56](https://github.com/arctic-hen7/perseus/commit/bdbdc5628591dc33b8b170a74ea5ba647491fae3))

## [0.3.0-beta.5](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.4...v0.3.0-beta.5) (2021-10-02)


### Bug Fixes

* 🚑 fixed page encodings ([6d2b7e6](https://github.com/arctic-hen7/perseus/commit/6d2b7e6641d4e59c6c6db2b42af494dbc667e21e))

## [0.3.0-beta.4](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.3...v0.3.0-beta.4) (2021-10-02)


### Bug Fixes

* **templates:** 🐛 decoded path before passing to build state ([596f38e](https://github.com/arctic-hen7/perseus/commit/596f38e8684efbe795b6cc3ed2b68b6c3528f3cf)), closes [#44](https://github.com/arctic-hen7/perseus/issues/44)

## [0.3.0-beta.3](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.2...v0.3.0-beta.3) (2021-10-02)


### ⚠ BREAKING CHANGES

* **i18n:** build/request state now take locale as second parameter (request state takes request as third now)

### Features

* **i18n:** ✨ passed locale to build and request state ([#43](https://github.com/arctic-hen7/perseus/issues/43)) ([95d28bb](https://github.com/arctic-hen7/perseus/commit/95d28bb2525feb3eb332666d9c66f713bfd06fa3))


### Documentation Changes

* **book:** 📝 updated migration guide for beta ([643e51e](https://github.com/arctic-hen7/perseus/commit/643e51efc0da3f2d212cbcb1e9e83d3361d1c923))

## [0.3.0-beta.2](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.1...v0.3.0-beta.2) (2021-10-01)


### Bug Fixes

* 🐛 fixed build paths issues ([#41](https://github.com/arctic-hen7/perseus/issues/41)) ([532243e](https://github.com/arctic-hen7/perseus/commit/532243e07a1b70d41fe841444fc62d382c2d6a31)), closes [#40](https://github.com/arctic-hen7/perseus/issues/40)

## [0.3.0-beta.1](https://github.com/arctic-hen7/perseus/compare/v0.2.3...v0.3.0-beta.1) (2021-09-30)


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

* ✨ added deployment ([#37](https://github.com/arctic-hen7/perseus/issues/37)) ([a8989dd](https://github.com/arctic-hen7/perseus/commit/a8989ddba203b4825531419cc29b0e6e0ab61ae0))
* **cli:** ✨ added `--release` mode to cli ([#35](https://github.com/arctic-hen7/perseus/issues/35)) ([f66bbb9](https://github.com/arctic-hen7/perseus/commit/f66bbb9b9ae7030a22bd3f7320a83ef7cfe79f37))
* ✨ switched to new error systems, added `is_server!`, and improved render function return types ([#33](https://github.com/arctic-hen7/perseus/issues/33)) ([53bb61e](https://github.com/arctic-hen7/perseus/commit/53bb61e6b9595f7746d0454355569ba79082b069))


### Code Refactorings

* **cli:** ♻️ migrated cli to `clap` ([#34](https://github.com/arctic-hen7/perseus/issues/34)) ([83e365c](https://github.com/arctic-hen7/perseus/commit/83e365c37cfa19a39edcc69562833052edfe8f1c))


### Documentation Changes

* **book:** 📝 added docs for v0.3.x and deprecated v0.2.x ([b2e3c57](https://github.com/arctic-hen7/perseus/commit/b2e3c57cb0da5a58141500a876e32542be49adb6))
* **book:** 📝 added migration page for upgrading from v0.2.x ([df00cf3](https://github.com/arctic-hen7/perseus/commit/df00cf388b95c9705c487b97c0e6e14fa3e445b7))
* **book:** 📝 updated latest stable version of docs ([ab19e78](https://github.com/arctic-hen7/perseus/commit/ab19e7883e9c57b55e9b780ea292aa10c6bd2763))

### [0.2.3](https://github.com/arctic-hen7/perseus/compare/v0.2.2...v0.2.3) (2021-09-26)


### Features

* **templates:** ✨ added context to templates if they're beeing rendered on the server or client ([7600c95](https://github.com/arctic-hen7/perseus/commit/7600c95b6f7e10574b4597bda268cb0391810c99)), closes [#26](https://github.com/arctic-hen7/perseus/issues/26)
* ✨ made initial content container invisible for errors as well ([0150c8d](https://github.com/arctic-hen7/perseus/commit/0150c8d376d39f355ee7c475f0529671e80915d4))
* ✨ made initial content container invisible once content has loaded ([4daa8c2](https://github.com/arctic-hen7/perseus/commit/4daa8c2a4ec912bde118006dd4329cfa69d5a168))
* ✨ renamed `__perseus_content` to `__perseus_content_initial` and made `__perseus_content` a class ([7242d74](https://github.com/arctic-hen7/perseus/commit/7242d74291e447d448640fc249c489515acc3abe))


### Bug Fixes

* 🚑 changed browser-checking logic to not use context ([4cd06c5](https://github.com/arctic-hen7/perseus/commit/4cd06c5a4e9d52fef53d7cbce8dbcee1348d21e9))
* **i18n:** 🐛 used absolute paths in translation macros ([a413e85](https://github.com/arctic-hen7/perseus/commit/a413e85e683fd0dfa0ca0471c565432cec6eef6d))
* 🐛 changed `__perseus_content_rx` to use `id` instead of `class` ([e504f6d](https://github.com/arctic-hen7/perseus/commit/e504f6d15ee4faaac7e34921fa3ef969210cbb38))


### Documentation Changes

* 📝 added docs for styling pitfalls ([66b43e1](https://github.com/arctic-hen7/perseus/commit/66b43e16b14d615c04fb5eb180d4c9530f9ac590)), closes [#28](https://github.com/arctic-hen7/perseus/issues/28)

### [0.2.2](https://github.com/arctic-hen7/perseus/compare/v0.2.1...v0.2.2) (2021-09-25)


### Features

* **templates:** ✨ added ability to set http headers for templates ([#25](https://github.com/arctic-hen7/perseus/issues/25)) ([058d625](https://github.com/arctic-hen7/perseus/commit/058d625575e28460004a6114c6fa6bacedf76515))
* ✨ added static exporting ([#23](https://github.com/arctic-hen7/perseus/issues/23)) ([4838ba4](https://github.com/arctic-hen7/perseus/commit/4838ba43611b0156afa5c84d2454ca6cbbf5f5a1)), closes [#22](https://github.com/arctic-hen7/perseus/issues/22)


### Bug Fixes

* **cli:** 🐛 surrounded url with angular brackets ([7688d7d](https://github.com/arctic-hen7/perseus/commit/7688d7d4ebab0682dbdd1422f7df3feca117a30f)), closes [#24](https://github.com/arctic-hen7/perseus/issues/24)


### Documentation Changes

* 📝 removed duplication in changelog ([0ba3e2c](https://github.com/arctic-hen7/perseus/commit/0ba3e2c698fa880405f9ef930bfee0c227e8c886))
* **book:** 📝 added docs on header modification ([bca6430](https://github.com/arctic-hen7/perseus/commit/bca6430ca0abeb1afdb2d48abfad414be6bf4ef4))
* 📝 added badges to readme ([0441f80](https://github.com/arctic-hen7/perseus/commit/0441f80a2fcd43fd15e94c4baa56bfc9e11f0788))
* 📝 removed unnecessary readme links ([295a7b5](https://github.com/arctic-hen7/perseus/commit/295a7b5c6c8404ef977c3d1924513103d94acd79))

### [0.2.1](https://github.com/arctic-hen7/perseus/compare/v0.2.0...v0.2.1) (2021-09-23)

### Features

-   **testing:** ✨ added testing harness and tests for examples ([#21](https://github.com/arctic-hen7/perseus/issues/21)) ([4cca6f7](https://github.com/arctic-hen7/perseus/commit/4cca6f7403e6c566592468a2d5d0a836c8ec06fa))

### Code Refactorings

-   **routing:** ♻️ refactored to eliminate only remaining js ([dc21490](https://github.com/arctic-hen7/perseus/commit/dc21490d462654ef6fad3abc3cd3e322e0b2bb1f))

### Documentation Changes

-   📝 updated readme to reflect js elimination ([4d5cf2a](https://github.com/arctic-hen7/perseus/commit/4d5cf2add178277446b67b46e599c8a144dd8e8e))
-   **book:** ✏️ fixed typos in the book ([f84cfb0](https://github.com/arctic-hen7/perseus/commit/f84cfb097129f97509ced5c0d9da1a881eb4b53a))

## [0.2.0](https://github.com/arctic-hen7/perseus/compare/v0.1.4...v0.2.0) (2021-09-21)

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

-   **book:** ✨ added versions for book ([bbdcea2](https://github.com/arctic-hen7/perseus/commit/bbdcea24b942a53e1c538cfb79ba63161bff9d4a))
-   **cli:** ✨ added `eject` command ([b747152](https://github.com/arctic-hen7/perseus/commit/b7471522ee167cf798a2a76084ca18d21b1be678)), closes [#14](https://github.com/arctic-hen7/perseus/issues/14)
-   **routing:** ✨ moved subsequent load head generation to server-side ([1e02ca4](https://github.com/arctic-hen7/perseus/commit/1e02ca4e5a753e4de699dfd21d215aa0d996d05c)), closes [#15](https://github.com/arctic-hen7/perseus/issues/15)
-   ✨ added initial load control ([7335418](https://github.com/arctic-hen7/perseus/commit/733541811b5bf5300c46c72c755cb2ef120d9829)), closes [#2](https://github.com/arctic-hen7/perseus/issues/2)
-   ✨ added metadata modification systems ([bb847aa](https://github.com/arctic-hen7/perseus/commit/bb847aaedbaa3cc0bb340bd54a597a1a599230f4)), closes [#2](https://github.com/arctic-hen7/perseus/issues/2) [#13](https://github.com/arctic-hen7/perseus/issues/13)
-   ✨ added support for static content and aliases ([7f38ea7](https://github.com/arctic-hen7/perseus/commit/7f38ea7be28c6b6ae29c8bfb050db81246d67c9f))
-   ✨ improved `define_app!` macro ([8bf6dd5](https://github.com/arctic-hen7/perseus/commit/8bf6dd53a23694270c10f3c913fda2b051638bba))
-   **cli:** ✨ added single-threaded mode for the CLI ([5cb465a](https://github.com/arctic-hen7/perseus/commit/5cb465aab460a2c11db9a89a7290faeb53243be2)), closes [#11](https://github.com/arctic-hen7/perseus/issues/11)
-   **cli:** ✨ parallelized cli stages and removed rollup ([7693ebf](https://github.com/arctic-hen7/perseus/commit/7693ebf524cb5c499bb5ec51ae7ce9f505660e6e)), closes [#7](https://github.com/arctic-hen7/perseus/issues/7) [#9](https://github.com/arctic-hen7/perseus/issues/9)
-   **i18n:** ✨ added dummy translator to support not using i18n ([803b4f6](https://github.com/arctic-hen7/perseus/commit/803b4f6cce0ba55eb050e454d6359e8cf8a962c3))
-   **i18n:** ✨ added fn on translations manager to get string translations ([649a65d](https://github.com/arctic-hen7/perseus/commit/649a65d59f480bd2f0bd18320113b67cb8651d0a))
-   **i18n:** ✨ added i18n to error pages and integrated fluent ([89fa00e](https://github.com/arctic-hen7/perseus/commit/89fa00eeafa55c986cd6cc784e63bf3bbf57a61b))
-   **i18n:** ✨ added locale detection ([b7ad607](https://github.com/arctic-hen7/perseus/commit/b7ad607861340c56bbfd504d6d2880108dbb0116))
-   **i18n:** ✨ added macros for translation and moved translator into context ([cbfe50c](https://github.com/arctic-hen7/perseus/commit/cbfe50c92ecbbbf860d03194fbbe23fa35302750))
-   **i18n:** ✨ added method to get url in same locale as user currently in ([fc8eeaf](https://github.com/arctic-hen7/perseus/commit/fc8eeafe598aaf8d0ba2c9b8e9dd1d0722b23bf8))
-   **i18n:** ✨ added server-side translations caching ([06b5fa4](https://github.com/arctic-hen7/perseus/commit/06b5fa443fe93a01e34d8b803f4b1a6eb25a98b2))
-   **i18n:** ✨ feature-gated translators ([a123f0d](https://github.com/arctic-hen7/perseus/commit/a123f0dc7e0381a10eba9a863938e1a4eedf1eab))
-   **i18n:** ✨ removed concept of common locales ([95b476f](https://github.com/arctic-hen7/perseus/commit/95b476f9b4f34fbff98a10dff18851c833f7e817))
-   **routing:** ✨ added perseus routing systems and simplified app definition ([49aa2b9](https://github.com/arctic-hen7/perseus/commit/49aa2b9d998871101f7fc2ef7c1a9c45d7873b8c))
-   **routing:** ✨ switched to template-based routing ([78688c1](https://github.com/arctic-hen7/perseus/commit/78688c13e840e9d364d61a3173a08ec5c70ae126)), closes [#12](https://github.com/arctic-hen7/perseus/issues/12)
-   ✨ added build artifact purging to cli ([ef0cf76](https://github.com/arctic-hen7/perseus/commit/ef0cf766b15232e68c2d775c84006b22413f87d2))
-   ✨ added i18n ([a4402c0](https://github.com/arctic-hen7/perseus/commit/a4402c04970019b9b965e4aaf6a38edbae2fc4ce))
-   ✨ made cli preserve relative paths in development ([d79f029](https://github.com/arctic-hen7/perseus/commit/d79f029c9fec5acae96194d1eb8de09a60a9157f))

### Bug Fixes

-   🐛 added `$crate` to invocation of `define_app!` ([c2a4560](https://github.com/arctic-hen7/perseus/commit/c2a4560a0bc60b98cb3ea04f49a62a08b3f2b59e))
-   🐛 handled page rendering errors properly at initial load ([3a9f44a](https://github.com/arctic-hen7/perseus/commit/3a9f44a39573ef2eb362f002b176652985aa7966))
-   🐛 removed deliberately inserted error for debugging ([a1fec62](https://github.com/arctic-hen7/perseus/commit/a1fec6216a2f60d14acc54e351c970ab307ee1a1))
-   🔒 disallowed `static_aliases` outside current directory ([08971ca](https://github.com/arctic-hen7/perseus/commit/08971caa5afde082de9e95c333c0f32fe76698a8))
-   **cli:** 🐛 fixed cli `--no-build` option ([9890457](https://github.com/arctic-hen7/perseus/commit/98904572698b60de566a5283d25b868cd3ef2abf))
-   **routing:** 🐛 fixed [#8](https://github.com/arctic-hen7/perseus/issues/8) ([5a787c4](https://github.com/arctic-hen7/perseus/commit/5a787c4965c30a9d9d7ac338dbd8bbf1de39aefd))
-   **routing:** 🐛 fixed error duplication on initial load ([53058ba](https://github.com/arctic-hen7/perseus/commit/53058ba025750e5eb5508c19a40e2977acaeda45))
-   **routing:** 🐛 fixed link handling errors in [#8](https://github.com/arctic-hen7/perseus/issues/8) ([197956b](https://github.com/arctic-hen7/perseus/commit/197956bc734bc1d85f56bcfc7b327bb1ed1f4c07))
-   ✏️ fixed displayed number of steps in cli serving (4 -> 5) ([d1a6bb8](https://github.com/arctic-hen7/perseus/commit/d1a6bb86bef8eeb67f682f2aac719623400dd2e2))
-   ✏️ updated all instances of _WASM_ to _Wasm_ ([f7ec1aa](https://github.com/arctic-hen7/perseus/commit/f7ec1aa9227592e04370dd9c5b85ab577193330b))
-   🐛 used absolute paths in `web_log!` macro ([945bd2a](https://github.com/arctic-hen7/perseus/commit/945bd2a82ff0884df362ec303c38731d9b470ed8))

### Performance Improvements

-   ⚡️ inlined wasm load script to reduce full requests ([6cfe8e1](https://github.com/arctic-hen7/perseus/commit/6cfe8e15d812400c5bff387cffd8a6dd715ce59b))
-   **cli:** ⚡️ created workspace in cli subcrates ([3e11ecd](https://github.com/arctic-hen7/perseus/commit/3e11ecd6da6b618a5b94c5abfc33264e37304482))
-   **i18n:** ⚡️ removed needless translations fetch if not using i18n ([7c6f697](https://github.com/arctic-hen7/perseus/commit/7c6f697dfceff6b93a8ad87d13924510f7174ad7))
-   ⚡️ switched to `Rc<ErrorPages>` to avoid producing unnecessary `ErrorPages` ([6786ff4](https://github.com/arctic-hen7/perseus/commit/6786ff4c6781e020af3bfd6d3306c8f899c11c85))
-   ⚡️ switched to `Rc<T>`s instead of `Arc<T>`s ([8d70599](https://github.com/arctic-hen7/perseus/commit/8d70599f803c22ff4a7eaa03b074480d0b5b6e74))

### Code Refactorings

-   ♻️ cleaned up macros ([30345f0](https://github.com/arctic-hen7/perseus/commit/30345f085f7183e85d3acf3be3c0d4ce7f92790a))
-   ♻️ renamed `incremental_path_rendering` to `incremental_generation` and improved interface ([cb60be0](https://github.com/arctic-hen7/perseus/commit/cb60be025039d4808aeb8429ed67a885625b117e))
-   ♻️ rewrote `showcase` example to use cli ([c2f1091](https://github.com/arctic-hen7/perseus/commit/c2f109157f5f3848c195ef6f55373b34f24e67b7))
-   🎨 cleaned a few things up ([0ab791f](https://github.com/arctic-hen7/perseus/commit/0ab791fb8bc4cf8e7f07e19cc4f3e2420f4230d2))
-   🔥 removed unnecessary `X-UA-Compatible` headers ([73643b8](https://github.com/arctic-hen7/perseus/commit/73643b8c54091533790a09e54d2c53e3b5f62a9b))
-   **i18n:** 🚚 refactored to prepare for future multi-translator support ([24f4362](https://github.com/arctic-hen7/perseus/commit/24f4362c6abeb4b72ef499f32edc6349fda5891d))

### Documentation Changes

-   **book:** 📝 added docs on migrating from 0.1.x ([056fb58](https://github.com/arctic-hen7/perseus/commit/056fb5830d848510a00f42dd69f304145d364429))
-   **book:** 📝 added full intro to perseus ([424e3f4](https://github.com/arctic-hen7/perseus/commit/424e3f4a5b1bb0a8fb11c7c23e4337b8ff35a982))
-   **book:** 📝 added hello world and second app tutorials to book ([58eb92d](https://github.com/arctic-hen7/perseus/commit/58eb92db00608736cb8ebfc795cd568a053288b4))
-   **book:** 📝 finished docs for v0.2.x ([c7d3ea2](https://github.com/arctic-hen7/perseus/commit/c7d3ea25862fbb9f8a1bad84bb6d866b5cd6cbdd))
-   **book:** 📝 fixed relative paths in docs and added docs about `StringResultWithCause<T>` ([39b3ce1](https://github.com/arctic-hen7/perseus/commit/39b3ce197580bf430afd5140867e5632dcc081fc))
-   **book:** 📝 wrote advanced docs on routing ([31497ab](https://github.com/arctic-hen7/perseus/commit/31497ab26de444c2d32c9903326ecea0d1172a60))
-   **book:** 📝 wrote book initial reference sections ([f7f7892](https://github.com/arctic-hen7/perseus/commit/f7f7892fbf124a7d887b1f22a1641c79773d6246))
-   **book:** 📝 wrote cli docs ([e321c38](https://github.com/arctic-hen7/perseus/commit/e321c389c17b93675bca1bc93eacaf1ba4da30aa))
-   **book:** 📝 wrote docs for i18n, error pages, and static content ([0375f01](https://github.com/arctic-hen7/perseus/commit/0375f013e0f02778829b5ec8903a10ecfbe4d127))
-   **book:** 📝 wrote large parts of advanced docs and some other pages ([d8fd43f](https://github.com/arctic-hen7/perseus/commit/d8fd43f75385c72a17627cc0d5f71c4496d95c42))
-   **book:** 🔖 released v0.2.x docs ([3cd80d0](https://github.com/arctic-hen7/perseus/commit/3cd80d0fb2f0ae2e5fbb14295f37181f4778161b))
-   ✏️ fixed some typos and clarified things in readmes ([5c59ae6](https://github.com/arctic-hen7/perseus/commit/5c59ae6855aa22874314abccdc968cb58345ffba))
-   💡 removed duplicate link typo in comment ([379d549](https://github.com/arctic-hen7/perseus/commit/379d549b31d3929dc383cb852c623f39e91c0201))
-   💡 removed entirely useless comment in showcase example ([2105f5a](https://github.com/arctic-hen7/perseus/commit/2105f5a79061ecbc871aa489db644e62e3d52692))
-   📝 added explanation for 0.1% js to readme ([6f0bd08](https://github.com/arctic-hen7/perseus/commit/6f0bd088af2bed928ba95f963c3defa20eef3460))
-   📝 cleaned up docs ([b6a6b72](https://github.com/arctic-hen7/perseus/commit/b6a6b72b7b47937f9d60306524d75678154255fc))
-   **book:** 🚑 updated versions of sycamore in book ([e41d3e5](https://github.com/arctic-hen7/perseus/commit/e41d3e5a3173979548adee165453a73e60d99173))
-   **examples:** ✨ added new `tiny` example and updated readme with it ([2c2d06b](https://github.com/arctic-hen7/perseus/commit/2c2d06b3ee8cdc49614c42ee2a82c923af131be6))
-   **examples:** 🚚 merged basic/cli examples and cleaned up examples ([db6fbdd](https://github.com/arctic-hen7/perseus/commit/db6fbdd4047044acff51a1cc3e6564661fe22016))
-   📝 updated roadmap in readme ([c3ad018](https://github.com/arctic-hen7/perseus/commit/c3ad0185b40df84efef10862f9fb150e6610bd2f))
-   📝 wrote tutorial on building first app ([19f0458](https://github.com/arctic-hen7/perseus/commit/19f045840e1cf6e9191aaaf3e98d15b5a98d8370))

### [0.1.4](https://github.com/arctic-hen7/perseus/compare/v0.1.3...v0.1.4) (2021-09-11)

### Bug Fixes

-   🐛 updated `basic` example perseus version ([1d8d895](https://github.com/arctic-hen7/perseus/commit/1d8d895a0c6ed5d9cb96a14d06c702917c3837c1))
-   🚑 allowed env var specification of command paths in building/serving ([5a2e494](https://github.com/arctic-hen7/perseus/commit/5a2e49475a9e6ef1e1d25491530f8be9b22f74f5))

### [0.1.3](https://github.com/arctic-hen7/perseus/compare/v0.1.2...v0.1.3) (2021-09-11)

### Bug Fixes

-   🚑 commands now executed in shells ([80604a4](https://github.com/arctic-hen7/perseus/commit/80604a4b1323ec322e875bb6bdc7e05b4768b1a6))
-   🚑 fixed windows cli bug ([1b6ef16](https://github.com/arctic-hen7/perseus/commit/1b6ef164ebf6a8c9f3c2f9c27488d181b0760b36))

### [0.1.2](https://github.com/arctic-hen7/perseus/compare/v0.1.1...v0.1.2) (2021-09-03)

### Bug Fixes

-   🐛 fixed cli executable name ([573fc2f](https://github.com/arctic-hen7/perseus/commit/573fc2f962039d91fb08e49a162d4972a7a935df))

### Documentation Changes

-   📝 added crate docs for `perseus-actix-web` ([f5036e7](https://github.com/arctic-hen7/perseus/commit/f5036e756ce789812e08752b1e7e31b0c70d4c1c))
-   📝 added crate docs for `perseus` package ([61ca6c0](https://github.com/arctic-hen7/perseus/commit/61ca6c080931b5a67e82403e0c32de5934e8781d))
-   📝 added crate documentation for `perseus-cli` and fixed doc typos ([b3ec9ac](https://github.com/arctic-hen7/perseus/commit/b3ec9aca0a5f08fb91d411f54964e4a02ffa2066))
-   📝 updated readme with contact links ([a2bc5f2](https://github.com/arctic-hen7/perseus/commit/a2bc5f271263d5ed85618b818d5e27d1d2dde191))

### [0.1.1](https://github.com/arctic-hen7/perseus/compare/v0.1.0...v0.1.1) (2021-09-03)

### Bug Fixes

-   🐛 added version numbers for local package imports ([b700cf7](https://github.com/arctic-hen7/perseus/commit/b700cf72325b54a987c87415de3f119273690650))
-   🐛 fixed cli packaging issues ([dd43e81](https://github.com/arctic-hen7/perseus/commit/dd43e8132d9b6cde82874883291c79e6d1ba6676))

## 0.1.0 (2021-09-02)

### Features

-   ✨ added access to request data in ssr ([02ce425](https://github.com/arctic-hen7/perseus/commit/02ce42573ff5cf6f279c3932b68901bfd48922dc))
-   ✨ added actix-web integration ([0e0f2f1](https://github.com/arctic-hen7/perseus/commit/0e0f2f19463c9f04ea7d886e3d41672ab74bfb17))
-   ✨ added basic cli ([5e7a867](https://github.com/arctic-hen7/perseus/commit/5e7a867965f93ec16128e2b07cae91dc7d8b907e))
-   ✨ added basic sycamore ssg systems ([c8530cf](https://github.com/arctic-hen7/perseus/commit/c8530cf47afcc45585ac346e3e717f516361ca7e))
-   ✨ added build command to cli ([66dc282](https://github.com/arctic-hen7/perseus/commit/66dc28273d17d6e763aac52da8d23c9595c8deab))
-   ✨ added isr ([5baf9bf](https://github.com/arctic-hen7/perseus/commit/5baf9bf0eb92031f4e5fee0158403ada376f4bf3))
-   ✨ added page path matching logic ([734f9df](https://github.com/arctic-hen7/perseus/commit/734f9df6c7f84902c9a3975bf3138f6442a08697))
-   ✨ added request conversion logic for actix web ([71a5445](https://github.com/arctic-hen7/perseus/commit/71a54454bfeaf537bae4bbce639d513f02be88be))
-   ✨ added revalidation and refactored a fully modular rendering system ([c9df616](https://github.com/arctic-hen7/perseus/commit/c9df616983d3ef240ea63059eb1fa45b8e92f1d4))
-   ✨ added serving systems to cli ([335ff5d](https://github.com/arctic-hen7/perseus/commit/335ff5d7d3f61cf8aea90b9d9e4071b5c0739701))
-   ✨ added ssr ([ac79996](https://github.com/arctic-hen7/perseus/commit/ac799966a684595d4a28750a043a1ae172fad527))
-   ✨ added template method to define function for amalgamating states ([1cb4356](https://github.com/arctic-hen7/perseus/commit/1cb435663a09a78c9444ef05a2bbf7e5a15a1e99))
-   ✨ allowed user render functions to return errors ([fa50d4c](https://github.com/arctic-hen7/perseus/commit/fa50d4cd1e05470386dc3aad0020f21970c62a80))
-   ✨ built subcrate tro underlie cli functionality ([1e7e355](https://github.com/arctic-hen7/perseus/commit/1e7e3551f229504ef92077f8047710b7d502a2d8))
-   ✨ made config managers async ([5e03cad](https://github.com/arctic-hen7/perseus/commit/5e03cad26b3164d5c831adfe187240fa5ddb73dc))
-   ✨ made rendering functions asynchronous ([5b403b2](https://github.com/arctic-hen7/perseus/commit/5b403b2d5181256d0aaf0f23f880fc8d5aade0c8))
-   ✨ props now passed around as strings ([7a334cf](https://github.com/arctic-hen7/perseus/commit/7a334cf39a76230a9cc3ca3c797768a182a8bdc5))
-   ✨ re-exported sycamore `GenericNode` ([8b79be8](https://github.com/arctic-hen7/perseus/commit/8b79be86c9deb941f3d743abfac12c31d0c0db8e))
-   ✨ refactored examples and created preparation system in cli ([8aa3d0f](https://github.com/arctic-hen7/perseus/commit/8aa3d0f9db5020f4befcb5845ac3a851cb40c8c5))
-   ✨ set up cli systems for preparation and directory cleaning ([36660f8](https://github.com/arctic-hen7/perseus/commit/36660f899d0dc2dd389173b1299de36f4fa3c8dc))
-   🎉 added readme and license ([0306a10](https://github.com/arctic-hen7/perseus/commit/0306a10da1bcffcc4d2426da365c76a465795ab4))
-   🥅 set up proper error handling ([7ea3ec0](https://github.com/arctic-hen7/perseus/commit/7ea3ec0c3fa59b1e1e028cba45217ddd9e3320ce))

### Bug Fixes

-   🐛 allowed build state to return `ErrorCause` for incremental generation ([dd4d60f](https://github.com/arctic-hen7/perseus/commit/dd4d60ff9f925b592c4359ae7e76f0a9eee1a752))
-   🐛 fixed inconsistent path prefixing in `build_state` calls ([96066d0](https://github.com/arctic-hen7/perseus/commit/96066d0019f2e68c79349886a4af1f5f37248c62))
-   🐛 fixed recursive extraction and excluded subcrates from workspaces ([c745cf2](https://github.com/arctic-hen7/perseus/commit/c745cf2b4381918c821accc351dbff368fd453a1))
-   🐛 removed old debug log ([ed4f43a](https://github.com/arctic-hen7/perseus/commit/ed4f43a75550faa781c261edf6caafd688f32961))
-   🐛 used config manager instead of raw fs in `get_render_cfg()` ([e75de5a](https://github.com/arctic-hen7/perseus/commit/e75de5a1bcdd48f67a288e0fb89bde0a6e959a83))

### Code Refactorings

-   ♻️ changed `define_app!`'s `router` to use curly brackets ([d5519b9](https://github.com/arctic-hen7/perseus/commit/d5519b9fb6c4e3909248acabeb8088d853468c6c))
-   ♻️ created sane library interface ([51284a8](https://github.com/arctic-hen7/perseus/commit/51284a86bf5e33730768cc3946af3d2ac848b695))
-   ♻️ moved logic into core package from example ([b2e9a68](https://github.com/arctic-hen7/perseus/commit/b2e9a683211c798c6254e2ae328f97d37bec5d29))
-   ♻️ removed useless render options system ([1af26dc](https://github.com/arctic-hen7/perseus/commit/1af26dcf78b95b57a45c2b086e234d21a5932763))
-   🚚 moved everything into packages ([dcbabc0](https://github.com/arctic-hen7/perseus/commit/dcbabc0c4c504911c13da166992bcbe072ca163d))
-   🚚 renamed pages to templates for clarity ([7c9e433](https://github.com/arctic-hen7/perseus/commit/7c9e4337f06412c739e050d3bbfd3d6c4d56f69c))

### Documentation Changes

-   💡 removed old todos ([9464ee5](https://github.com/arctic-hen7/perseus/commit/9464ee5f854c9f81840acf4a32a8707c5e926ca5))
-   📝 added docs for cli ([e4f9cce](https://github.com/arctic-hen7/perseus/commit/e4f9cce19cadd9af91aea47f02d47aebddbc1014))
-   📝 added documentation for actix-web integration ([1877c13](https://github.com/arctic-hen7/perseus/commit/1877c130a3fb4c6e6e593ba439d818fc24121c17))
-   📝 added example of state amalgamation ([cd93fdc](https://github.com/arctic-hen7/perseus/commit/cd93fdca3d5ab9f96af5c3d846c69fa68d94b3ac))
-   📝 added link to percy in readme ([2072b9b](https://github.com/arctic-hen7/perseus/commit/2072b9b5537e2058d05c09cc0386931995753906))
-   📝 added repo docs ([043b65f](https://github.com/arctic-hen7/perseus/commit/043b65f8b5094e4207c4304968c4863feb08e42c))
-   📝 added scaffold for basic tutorial docs ([23fd0a6](https://github.com/arctic-hen7/perseus/commit/23fd0a6c087402a7c5aec0d60a9181d37f519b3c))
-   📝 fixed syntax highlighting in cli docs ([3242409](https://github.com/arctic-hen7/perseus/commit/32424094363a8112d0cbfa6ddad7321938b93b12))
-   📝 updated docs for v0.1.0 ([bf931e4](https://github.com/arctic-hen7/perseus/commit/bf931e4909b398f94b70ad37994497e1f9cab4ca))
-   📝 updated readme for significant dependency changes ([1d424b5](https://github.com/arctic-hen7/perseus/commit/1d424b55065f520f967001db45bc81630ba3aa43))
-   📝 wrote large sections of the book ([a548531](https://github.com/arctic-hen7/perseus/commit/a548531f882750699bca73f9db54741854dc9ef3))
