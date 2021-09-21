# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.2.0](https://github.com/arctic-hen7/perseus/compare/v0.1.4...v0.2.0) (2021-09-21)


### ⚠ BREAKING CHANGES

* renamed `incremental_path_rendering` to `incremental_generation`, and the corresponding template function no longer takes a value
* actix web integration now takes `static_dirs` and `static_aliases` options
* `js_init` no longer an option in actix web integration
* `error_pages` now comes after `templates` and `no_i18n` apps should not define `locales` at all
* error pages use `Rc`s now, new options for actix web integration, app root must be of `<div>` form
* **routing:** `define_app!` no longer takes routing paths, just templates
* **i18n:** templates no longer take translator (access via context instead)
* **routing:** `define_app!` redesigned, special meaning for `index` template name, app shell takes full templates, `Locales` has new property
* all `Arc<T>`s are now `Rc<T>`s
* **i18n:** `Translator` no longer `Serialize`/`Deserialize`
* **i18n:** `FsTranslationsManager` now takes a vector of locales to initially cache
* **i18n:** common locales no longer exist
* all user-facing interfaces take new i18n parameters

### Features

* **book:** ✨ added versions for book ([bbdcea2](https://github.com/arctic-hen7/perseus/commit/bbdcea24b942a53e1c538cfb79ba63161bff9d4a))
* **cli:** ✨ added `eject` command ([b747152](https://github.com/arctic-hen7/perseus/commit/b7471522ee167cf798a2a76084ca18d21b1be678)), closes [#14](https://github.com/arctic-hen7/perseus/issues/14)
* **routing:** ✨ moved subsequent load head generation to server-side ([1e02ca4](https://github.com/arctic-hen7/perseus/commit/1e02ca4e5a753e4de699dfd21d215aa0d996d05c)), closes [#15](https://github.com/arctic-hen7/perseus/issues/15)
* ✨ added initial load control ([7335418](https://github.com/arctic-hen7/perseus/commit/733541811b5bf5300c46c72c755cb2ef120d9829)), closes [#2](https://github.com/arctic-hen7/perseus/issues/2)
* ✨ added metadata modification systems ([bb847aa](https://github.com/arctic-hen7/perseus/commit/bb847aaedbaa3cc0bb340bd54a597a1a599230f4)), closes [#2](https://github.com/arctic-hen7/perseus/issues/2) [#13](https://github.com/arctic-hen7/perseus/issues/13)
* ✨ added support for static content and aliases ([7f38ea7](https://github.com/arctic-hen7/perseus/commit/7f38ea7be28c6b6ae29c8bfb050db81246d67c9f))
* ✨ improved `define_app!` macro ([8bf6dd5](https://github.com/arctic-hen7/perseus/commit/8bf6dd53a23694270c10f3c913fda2b051638bba))
* **cli:** ✨ added single-threaded mode for the CLI ([5cb465a](https://github.com/arctic-hen7/perseus/commit/5cb465aab460a2c11db9a89a7290faeb53243be2)), closes [#11](https://github.com/arctic-hen7/perseus/issues/11)
* **cli:** ✨ parallelized cli stages and removed rollup ([7693ebf](https://github.com/arctic-hen7/perseus/commit/7693ebf524cb5c499bb5ec51ae7ce9f505660e6e)), closes [#7](https://github.com/arctic-hen7/perseus/issues/7) [#9](https://github.com/arctic-hen7/perseus/issues/9)
* **i18n:** ✨ added dummy translator to support not using i18n ([803b4f6](https://github.com/arctic-hen7/perseus/commit/803b4f6cce0ba55eb050e454d6359e8cf8a962c3))
* **i18n:** ✨ added fn on translations manager to get string translations ([649a65d](https://github.com/arctic-hen7/perseus/commit/649a65d59f480bd2f0bd18320113b67cb8651d0a))
* **i18n:** ✨ added i18n to error pages and integrated fluent ([89fa00e](https://github.com/arctic-hen7/perseus/commit/89fa00eeafa55c986cd6cc784e63bf3bbf57a61b))
* **i18n:** ✨ added locale detection ([b7ad607](https://github.com/arctic-hen7/perseus/commit/b7ad607861340c56bbfd504d6d2880108dbb0116))
* **i18n:** ✨ added macros for translation and moved translator into context ([cbfe50c](https://github.com/arctic-hen7/perseus/commit/cbfe50c92ecbbbf860d03194fbbe23fa35302750))
* **i18n:** ✨ added method to get url in same locale as user currently in ([fc8eeaf](https://github.com/arctic-hen7/perseus/commit/fc8eeafe598aaf8d0ba2c9b8e9dd1d0722b23bf8))
* **i18n:** ✨ added server-side translations caching ([06b5fa4](https://github.com/arctic-hen7/perseus/commit/06b5fa443fe93a01e34d8b803f4b1a6eb25a98b2))
* **i18n:** ✨ feature-gated translators ([a123f0d](https://github.com/arctic-hen7/perseus/commit/a123f0dc7e0381a10eba9a863938e1a4eedf1eab))
* **i18n:** ✨ removed concept of common locales ([95b476f](https://github.com/arctic-hen7/perseus/commit/95b476f9b4f34fbff98a10dff18851c833f7e817))
* **routing:** ✨ added perseus routing systems and simplified app definition ([49aa2b9](https://github.com/arctic-hen7/perseus/commit/49aa2b9d998871101f7fc2ef7c1a9c45d7873b8c))
* **routing:** ✨ switched to template-based routing ([78688c1](https://github.com/arctic-hen7/perseus/commit/78688c13e840e9d364d61a3173a08ec5c70ae126)), closes [#12](https://github.com/arctic-hen7/perseus/issues/12)
* ✨ added build artifact purging to cli ([ef0cf76](https://github.com/arctic-hen7/perseus/commit/ef0cf766b15232e68c2d775c84006b22413f87d2))
* ✨ added i18n ([a4402c0](https://github.com/arctic-hen7/perseus/commit/a4402c04970019b9b965e4aaf6a38edbae2fc4ce))
* ✨ made cli preserve relative paths in development ([d79f029](https://github.com/arctic-hen7/perseus/commit/d79f029c9fec5acae96194d1eb8de09a60a9157f))


### Bug Fixes

* 🐛 added `$crate` to invocation of `define_app!` ([c2a4560](https://github.com/arctic-hen7/perseus/commit/c2a4560a0bc60b98cb3ea04f49a62a08b3f2b59e))
* 🐛 handled page rendering errors properly at initial load ([3a9f44a](https://github.com/arctic-hen7/perseus/commit/3a9f44a39573ef2eb362f002b176652985aa7966))
* 🐛 removed deliberately inserted error for debugging ([a1fec62](https://github.com/arctic-hen7/perseus/commit/a1fec6216a2f60d14acc54e351c970ab307ee1a1))
* 🔒 disallowed `static_aliases` outside current directory ([08971ca](https://github.com/arctic-hen7/perseus/commit/08971caa5afde082de9e95c333c0f32fe76698a8))
* **cli:** 🐛 fixed cli `--no-build` option ([9890457](https://github.com/arctic-hen7/perseus/commit/98904572698b60de566a5283d25b868cd3ef2abf))
* **routing:** 🐛 fixed [#8](https://github.com/arctic-hen7/perseus/issues/8) ([5a787c4](https://github.com/arctic-hen7/perseus/commit/5a787c4965c30a9d9d7ac338dbd8bbf1de39aefd))
* **routing:** 🐛 fixed error duplication on initial load ([53058ba](https://github.com/arctic-hen7/perseus/commit/53058ba025750e5eb5508c19a40e2977acaeda45))
* **routing:** 🐛 fixed link handling errors in [#8](https://github.com/arctic-hen7/perseus/issues/8) ([197956b](https://github.com/arctic-hen7/perseus/commit/197956bc734bc1d85f56bcfc7b327bb1ed1f4c07))
* ✏️ fixed displayed number of steps in cli serving (4 -> 5) ([d1a6bb8](https://github.com/arctic-hen7/perseus/commit/d1a6bb86bef8eeb67f682f2aac719623400dd2e2))
* ✏️ updated all instances of *WASM* to *Wasm* ([f7ec1aa](https://github.com/arctic-hen7/perseus/commit/f7ec1aa9227592e04370dd9c5b85ab577193330b))
* 🐛 used absolute paths in `web_log!` macro ([945bd2a](https://github.com/arctic-hen7/perseus/commit/945bd2a82ff0884df362ec303c38731d9b470ed8))


### Performance Improvements

* ⚡️ inlined wasm load script to reduce full requests ([6cfe8e1](https://github.com/arctic-hen7/perseus/commit/6cfe8e15d812400c5bff387cffd8a6dd715ce59b))
* **cli:** ⚡️ created workspace in cli subcrates ([3e11ecd](https://github.com/arctic-hen7/perseus/commit/3e11ecd6da6b618a5b94c5abfc33264e37304482))
* **i18n:** ⚡️ removed needless translations fetch if not using i18n ([7c6f697](https://github.com/arctic-hen7/perseus/commit/7c6f697dfceff6b93a8ad87d13924510f7174ad7))
* ⚡️ switched to `Rc<ErrorPages>` to avoid producing unnecessary `ErrorPages` ([6786ff4](https://github.com/arctic-hen7/perseus/commit/6786ff4c6781e020af3bfd6d3306c8f899c11c85))
* ⚡️ switched to `Rc<T>`s instead of `Arc<T>`s ([8d70599](https://github.com/arctic-hen7/perseus/commit/8d70599f803c22ff4a7eaa03b074480d0b5b6e74))


### Code Refactorings

* ♻️ cleaned up macros ([30345f0](https://github.com/arctic-hen7/perseus/commit/30345f085f7183e85d3acf3be3c0d4ce7f92790a))
* ♻️ renamed `incremental_path_rendering` to `incremental_generation` and improved interface ([cb60be0](https://github.com/arctic-hen7/perseus/commit/cb60be025039d4808aeb8429ed67a885625b117e))
* ♻️ rewrote `showcase` example to use cli ([c2f1091](https://github.com/arctic-hen7/perseus/commit/c2f109157f5f3848c195ef6f55373b34f24e67b7))
* 🎨 cleaned a few things up ([0ab791f](https://github.com/arctic-hen7/perseus/commit/0ab791fb8bc4cf8e7f07e19cc4f3e2420f4230d2))
* 🔥 removed unnecessary `X-UA-Compatible` headers ([73643b8](https://github.com/arctic-hen7/perseus/commit/73643b8c54091533790a09e54d2c53e3b5f62a9b))
* **i18n:** 🚚 refactored to prepare for future multi-translator support ([24f4362](https://github.com/arctic-hen7/perseus/commit/24f4362c6abeb4b72ef499f32edc6349fda5891d))


### Documentation Changes

* **book:** 📝 added docs on migrating from 0.1.x ([056fb58](https://github.com/arctic-hen7/perseus/commit/056fb5830d848510a00f42dd69f304145d364429))
* **book:** 📝 added full intro to perseus ([424e3f4](https://github.com/arctic-hen7/perseus/commit/424e3f4a5b1bb0a8fb11c7c23e4337b8ff35a982))
* **book:** 📝 added hello world and second app tutorials to book ([58eb92d](https://github.com/arctic-hen7/perseus/commit/58eb92db00608736cb8ebfc795cd568a053288b4))
* **book:** 📝 finished docs for v0.2.x ([c7d3ea2](https://github.com/arctic-hen7/perseus/commit/c7d3ea25862fbb9f8a1bad84bb6d866b5cd6cbdd))
* **book:** 📝 fixed relative paths in docs and added docs about `StringResultWithCause<T>` ([39b3ce1](https://github.com/arctic-hen7/perseus/commit/39b3ce197580bf430afd5140867e5632dcc081fc))
* **book:** 📝 wrote advanced docs on routing ([31497ab](https://github.com/arctic-hen7/perseus/commit/31497ab26de444c2d32c9903326ecea0d1172a60))
* **book:** 📝 wrote book initial reference sections ([f7f7892](https://github.com/arctic-hen7/perseus/commit/f7f7892fbf124a7d887b1f22a1641c79773d6246))
* **book:** 📝 wrote cli docs ([e321c38](https://github.com/arctic-hen7/perseus/commit/e321c389c17b93675bca1bc93eacaf1ba4da30aa))
* **book:** 📝 wrote docs for i18n, error pages, and static content ([0375f01](https://github.com/arctic-hen7/perseus/commit/0375f013e0f02778829b5ec8903a10ecfbe4d127))
* **book:** 📝 wrote large parts of advanced docs and some other pages ([d8fd43f](https://github.com/arctic-hen7/perseus/commit/d8fd43f75385c72a17627cc0d5f71c4496d95c42))
* **book:** 🔖 released v0.2.x docs ([3cd80d0](https://github.com/arctic-hen7/perseus/commit/3cd80d0fb2f0ae2e5fbb14295f37181f4778161b))
* ✏️ fixed some typos and clarified things in readmes ([5c59ae6](https://github.com/arctic-hen7/perseus/commit/5c59ae6855aa22874314abccdc968cb58345ffba))
* 💡 removed duplicate link typo in comment ([379d549](https://github.com/arctic-hen7/perseus/commit/379d549b31d3929dc383cb852c623f39e91c0201))
* 💡 removed entirely useless comment in showcase example ([2105f5a](https://github.com/arctic-hen7/perseus/commit/2105f5a79061ecbc871aa489db644e62e3d52692))
* 📝 added explanation for 0.1% js to readme ([6f0bd08](https://github.com/arctic-hen7/perseus/commit/6f0bd088af2bed928ba95f963c3defa20eef3460))
* 📝 cleaned up docs ([b6a6b72](https://github.com/arctic-hen7/perseus/commit/b6a6b72b7b47937f9d60306524d75678154255fc))
* **book:** 🚑 updated versions of sycamore in book ([e41d3e5](https://github.com/arctic-hen7/perseus/commit/e41d3e5a3173979548adee165453a73e60d99173))
* **examples:** ✨ added new `tiny` example and updated readme with it ([2c2d06b](https://github.com/arctic-hen7/perseus/commit/2c2d06b3ee8cdc49614c42ee2a82c923af131be6))
* **examples:** 🚚 merged basic/cli examples and cleaned up examples ([db6fbdd](https://github.com/arctic-hen7/perseus/commit/db6fbdd4047044acff51a1cc3e6564661fe22016))
* 📝 updated roadmap in readme ([c3ad018](https://github.com/arctic-hen7/perseus/commit/c3ad0185b40df84efef10862f9fb150e6610bd2f))
* 📝 wrote tutorial on building first app ([19f0458](https://github.com/arctic-hen7/perseus/commit/19f045840e1cf6e9191aaaf3e98d15b5a98d8370))

### [0.1.4](https://github.com/arctic-hen7/perseus/compare/v0.1.3...v0.1.4) (2021-09-11)


### Bug Fixes

* 🐛 updated `basic` example perseus version ([1d8d895](https://github.com/arctic-hen7/perseus/commit/1d8d895a0c6ed5d9cb96a14d06c702917c3837c1))
* 🚑 allowed env var specification of command paths in building/serving ([5a2e494](https://github.com/arctic-hen7/perseus/commit/5a2e49475a9e6ef1e1d25491530f8be9b22f74f5))

### [0.1.3](https://github.com/arctic-hen7/perseus/compare/v0.1.2...v0.1.3) (2021-09-11)


### Bug Fixes

* 🚑 commands now executed in shells ([80604a4](https://github.com/arctic-hen7/perseus/commit/80604a4b1323ec322e875bb6bdc7e05b4768b1a6))
* 🚑 fixed windows cli bug ([1b6ef16](https://github.com/arctic-hen7/perseus/commit/1b6ef164ebf6a8c9f3c2f9c27488d181b0760b36))

### [0.1.2](https://github.com/arctic-hen7/perseus/compare/v0.1.1...v0.1.2) (2021-09-03)


### Bug Fixes

* 🐛 fixed cli executable name ([573fc2f](https://github.com/arctic-hen7/perseus/commit/573fc2f962039d91fb08e49a162d4972a7a935df))


### Documentation Changes

* 📝 added crate docs for `perseus-actix-web` ([f5036e7](https://github.com/arctic-hen7/perseus/commit/f5036e756ce789812e08752b1e7e31b0c70d4c1c))
* 📝 added crate docs for `perseus` package ([61ca6c0](https://github.com/arctic-hen7/perseus/commit/61ca6c080931b5a67e82403e0c32de5934e8781d))
* 📝 added crate documentation for `perseus-cli` and fixed doc typos ([b3ec9ac](https://github.com/arctic-hen7/perseus/commit/b3ec9aca0a5f08fb91d411f54964e4a02ffa2066))
* 📝 updated readme with contact links ([a2bc5f2](https://github.com/arctic-hen7/perseus/commit/a2bc5f271263d5ed85618b818d5e27d1d2dde191))

### [0.1.1](https://github.com/arctic-hen7/perseus/compare/v0.1.0...v0.1.1) (2021-09-03)


### Bug Fixes

* 🐛 added version numbers for local package imports ([b700cf7](https://github.com/arctic-hen7/perseus/commit/b700cf72325b54a987c87415de3f119273690650))
* 🐛 fixed cli packaging issues ([dd43e81](https://github.com/arctic-hen7/perseus/commit/dd43e8132d9b6cde82874883291c79e6d1ba6676))

## 0.1.0 (2021-09-02)


### Features

* ✨ added access to request data in ssr ([02ce425](https://github.com/arctic-hen7/perseus/commit/02ce42573ff5cf6f279c3932b68901bfd48922dc))
* ✨ added actix-web integration ([0e0f2f1](https://github.com/arctic-hen7/perseus/commit/0e0f2f19463c9f04ea7d886e3d41672ab74bfb17))
* ✨ added basic cli ([5e7a867](https://github.com/arctic-hen7/perseus/commit/5e7a867965f93ec16128e2b07cae91dc7d8b907e))
* ✨ added basic sycamore ssg systems ([c8530cf](https://github.com/arctic-hen7/perseus/commit/c8530cf47afcc45585ac346e3e717f516361ca7e))
* ✨ added build command to cli ([66dc282](https://github.com/arctic-hen7/perseus/commit/66dc28273d17d6e763aac52da8d23c9595c8deab))
* ✨ added isr ([5baf9bf](https://github.com/arctic-hen7/perseus/commit/5baf9bf0eb92031f4e5fee0158403ada376f4bf3))
* ✨ added page path matching logic ([734f9df](https://github.com/arctic-hen7/perseus/commit/734f9df6c7f84902c9a3975bf3138f6442a08697))
* ✨ added request conversion logic for actix web ([71a5445](https://github.com/arctic-hen7/perseus/commit/71a54454bfeaf537bae4bbce639d513f02be88be))
* ✨ added revalidation and refactored a fully modular rendering system ([c9df616](https://github.com/arctic-hen7/perseus/commit/c9df616983d3ef240ea63059eb1fa45b8e92f1d4))
* ✨ added serving systems to cli ([335ff5d](https://github.com/arctic-hen7/perseus/commit/335ff5d7d3f61cf8aea90b9d9e4071b5c0739701))
* ✨ added ssr ([ac79996](https://github.com/arctic-hen7/perseus/commit/ac799966a684595d4a28750a043a1ae172fad527))
* ✨ added template method to define function for amalgamating states ([1cb4356](https://github.com/arctic-hen7/perseus/commit/1cb435663a09a78c9444ef05a2bbf7e5a15a1e99))
* ✨ allowed user render functions to return errors ([fa50d4c](https://github.com/arctic-hen7/perseus/commit/fa50d4cd1e05470386dc3aad0020f21970c62a80))
* ✨ built subcrate tro underlie cli functionality ([1e7e355](https://github.com/arctic-hen7/perseus/commit/1e7e3551f229504ef92077f8047710b7d502a2d8))
* ✨ made config managers async ([5e03cad](https://github.com/arctic-hen7/perseus/commit/5e03cad26b3164d5c831adfe187240fa5ddb73dc))
* ✨ made rendering functions asynchronous ([5b403b2](https://github.com/arctic-hen7/perseus/commit/5b403b2d5181256d0aaf0f23f880fc8d5aade0c8))
* ✨ props now passed around as strings ([7a334cf](https://github.com/arctic-hen7/perseus/commit/7a334cf39a76230a9cc3ca3c797768a182a8bdc5))
* ✨ re-exported sycamore `GenericNode` ([8b79be8](https://github.com/arctic-hen7/perseus/commit/8b79be86c9deb941f3d743abfac12c31d0c0db8e))
* ✨ refactored examples and created preparation system in cli ([8aa3d0f](https://github.com/arctic-hen7/perseus/commit/8aa3d0f9db5020f4befcb5845ac3a851cb40c8c5))
* ✨ set up cli systems for preparation and directory cleaning ([36660f8](https://github.com/arctic-hen7/perseus/commit/36660f899d0dc2dd389173b1299de36f4fa3c8dc))
* 🎉 added readme and license ([0306a10](https://github.com/arctic-hen7/perseus/commit/0306a10da1bcffcc4d2426da365c76a465795ab4))
* 🥅 set up proper error handling ([7ea3ec0](https://github.com/arctic-hen7/perseus/commit/7ea3ec0c3fa59b1e1e028cba45217ddd9e3320ce))


### Bug Fixes

* 🐛 allowed build state to return `ErrorCause` for incremental generation ([dd4d60f](https://github.com/arctic-hen7/perseus/commit/dd4d60ff9f925b592c4359ae7e76f0a9eee1a752))
* 🐛 fixed inconsistent path prefixing in `build_state` calls ([96066d0](https://github.com/arctic-hen7/perseus/commit/96066d0019f2e68c79349886a4af1f5f37248c62))
* 🐛 fixed recursive extraction and excluded subcrates from workspaces ([c745cf2](https://github.com/arctic-hen7/perseus/commit/c745cf2b4381918c821accc351dbff368fd453a1))
* 🐛 removed old debug log ([ed4f43a](https://github.com/arctic-hen7/perseus/commit/ed4f43a75550faa781c261edf6caafd688f32961))
* 🐛 used config manager instead of raw fs in `get_render_cfg()` ([e75de5a](https://github.com/arctic-hen7/perseus/commit/e75de5a1bcdd48f67a288e0fb89bde0a6e959a83))


### Code Refactorings

* ♻️ changed `define_app!`'s `router` to use curly brackets ([d5519b9](https://github.com/arctic-hen7/perseus/commit/d5519b9fb6c4e3909248acabeb8088d853468c6c))
* ♻️ created sane library interface ([51284a8](https://github.com/arctic-hen7/perseus/commit/51284a86bf5e33730768cc3946af3d2ac848b695))
* ♻️ moved logic into core package from example ([b2e9a68](https://github.com/arctic-hen7/perseus/commit/b2e9a683211c798c6254e2ae328f97d37bec5d29))
* ♻️ removed useless render options system ([1af26dc](https://github.com/arctic-hen7/perseus/commit/1af26dcf78b95b57a45c2b086e234d21a5932763))
* 🚚 moved everything into packages ([dcbabc0](https://github.com/arctic-hen7/perseus/commit/dcbabc0c4c504911c13da166992bcbe072ca163d))
* 🚚 renamed pages to templates for clarity ([7c9e433](https://github.com/arctic-hen7/perseus/commit/7c9e4337f06412c739e050d3bbfd3d6c4d56f69c))


### Documentation Changes

* 💡 removed old todos ([9464ee5](https://github.com/arctic-hen7/perseus/commit/9464ee5f854c9f81840acf4a32a8707c5e926ca5))
* 📝 added docs for cli ([e4f9cce](https://github.com/arctic-hen7/perseus/commit/e4f9cce19cadd9af91aea47f02d47aebddbc1014))
* 📝 added documentation for actix-web integration ([1877c13](https://github.com/arctic-hen7/perseus/commit/1877c130a3fb4c6e6e593ba439d818fc24121c17))
* 📝 added example of state amalgamation ([cd93fdc](https://github.com/arctic-hen7/perseus/commit/cd93fdca3d5ab9f96af5c3d846c69fa68d94b3ac))
* 📝 added link to percy in readme ([2072b9b](https://github.com/arctic-hen7/perseus/commit/2072b9b5537e2058d05c09cc0386931995753906))
* 📝 added repo docs ([043b65f](https://github.com/arctic-hen7/perseus/commit/043b65f8b5094e4207c4304968c4863feb08e42c))
* 📝 added scaffold for basic tutorial docs ([23fd0a6](https://github.com/arctic-hen7/perseus/commit/23fd0a6c087402a7c5aec0d60a9181d37f519b3c))
* 📝 fixed syntax highlighting in cli docs ([3242409](https://github.com/arctic-hen7/perseus/commit/32424094363a8112d0cbfa6ddad7321938b93b12))
* 📝 updated docs for v0.1.0 ([bf931e4](https://github.com/arctic-hen7/perseus/commit/bf931e4909b398f94b70ad37994497e1f9cab4ca))
* 📝 updated readme for significant dependency changes ([1d424b5](https://github.com/arctic-hen7/perseus/commit/1d424b55065f520f967001db45bc81630ba3aa43))
* 📝 wrote large sections of the book ([a548531](https://github.com/arctic-hen7/perseus/commit/a548531f882750699bca73f9db54741854dc9ef3))
