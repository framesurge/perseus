# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

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
