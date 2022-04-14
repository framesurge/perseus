# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.3.4-rc.4](https://github.com/arctic-hen7/perseus/compare/v0.3.4-rc.3...v0.3.4-rc.4) (2022-04-14)

### [0.3.4-rc.3](https://github.com/arctic-hen7/perseus/compare/v0.3.4-rc.2...v0.3.4-rc.3) (2022-04-13)


### Bug Fixes

* fixed versioning for `perseus-macro` dependencies ([e46c3ca](https://github.com/arctic-hen7/perseus/commit/e46c3caf0e36dfc6ec8a0a99a88ee83b99ceb2be))

### [0.3.4-rc.2](https://github.com/arctic-hen7/perseus/compare/v0.3.4-rc.1...v0.3.4-rc.2) (2022-04-13)

### [0.3.4-rc.1](https://github.com/arctic-hen7/perseus/compare/v0.3.3...v0.3.4-rc.1) (2022-04-13)


### Features

* **examples:** added auth example and docs ([e02088c](https://github.com/arctic-hen7/perseus/commit/e02088cf3ed7669b73792acaa9febc600de82437))
* added `.make_unrx()` ([b974576](https://github.com/arctic-hen7/perseus/commit/b974576eaac7fd4aa0b533ec63d688bd24ab0733))
* added better errors when no state generation functions are provided ([e565632](https://github.com/arctic-hen7/perseus/commit/e5656320c780048596dba6cad3aff8307968df69))
* added component name inference to `template_rx` ([d1ba2ef](https://github.com/arctic-hen7/perseus/commit/d1ba2ef82d7519d4a28e6d392303f49d89ff3d8c))
* added examples for and finalized idb wrapper ([362d5ca](https://github.com/arctic-hen7/perseus/commit/362d5caf0dbb7ccdc6a85a4706f5e5ab15d7294c))
* added global and reactive state ([90288f6](https://github.com/arctic-hen7/perseus/commit/90288f65fe3f64575cb3a4dd6e133da9f99a49bf)), closes [#103](https://github.com/arctic-hen7/perseus/issues/103)
* added global state ([a5fcc56](https://github.com/arctic-hen7/perseus/commit/a5fcc567166dfd1710cdaad925b764ab5881c8c1)), closes [#119](https://github.com/arctic-hen7/perseus/issues/119)
* added global state rehydration ([10634fb](https://github.com/arctic-hen7/perseus/commit/10634fb7046438ca518ef6f40133220b06887422))
* added hot state reloading ([9805a7b](https://github.com/arctic-hen7/perseus/commit/9805a7bfead8f24793c0b7e29f90d84470d910c4)), closes [#121](https://github.com/arctic-hen7/perseus/issues/121)
* added idb wrapper for state freezing ([9d2a729](https://github.com/arctic-hen7/perseus/commit/9d2a729ff9f370630ca8023c36d05bb9b5d6f7ee))
* added lazy global state instantiation ([82430db](https://github.com/arctic-hen7/perseus/commit/82430db463769e9f329aebd8057f46b45562e6be))
* added live reloading ([2e33424](https://github.com/arctic-hen7/perseus/commit/2e3342498b585aa10ef96933fe834986db92b8d5)), closes [#122](https://github.com/arctic-hen7/perseus/issues/122)
* added macro to enable fine-grained reactive state ([e12d15c](https://github.com/arctic-hen7/perseus/commit/e12d15c2a48962b55cb9126ce818436f6b78da6d))
* added page state store rehydration ([d95e355](https://github.com/arctic-hen7/perseus/commit/d95e3550ed89a7091e20922f6e5c3e1af01b06e9))
* added proper error handling to hsr ([469732a](https://github.com/arctic-hen7/perseus/commit/469732aede593bbb4aa36dda5873d6176573138c))
* added proper state thawing ([ea545a9](https://github.com/arctic-hen7/perseus/commit/ea545a9d9b9bd30fdfaf26c1cfeddccdc55751ce))
* added reloading server ([1f37700](https://github.com/arctic-hen7/perseus/commit/1f377003bddc22e4b041961d758ae5bc34b808f2))
* added route rehydration ([101f92a](https://github.com/arctic-hen7/perseus/commit/101f92a9eb9bffb67bfec0a154a4b5dd3dd4e119))
* added router state ([#115](https://github.com/arctic-hen7/perseus/issues/115)) ([9ee6904](https://github.com/arctic-hen7/perseus/commit/9ee69044ef8831d6f977782dba75324b7125aa1f))
* added same-page reloading ([6e32c8f](https://github.com/arctic-hen7/perseus/commit/6e32c8f0d78e28495ac48224e56176a9d91a683f)), closes [#120](https://github.com/arctic-hen7/perseus/issues/120)
* added state freezing ([891f3bb](https://github.com/arctic-hen7/perseus/commit/891f3bb7e02087b450292da7ee591b2e5f206420))
* added support for `#[make_rx(...)]` on unit `struct`s ([cb2f49d](https://github.com/arctic-hen7/perseus/commit/cb2f49d7fd2d6b266246ae426728896ea7dae923))
* added support for templates that take only global state ([c60af8a](https://github.com/arctic-hen7/perseus/commit/c60af8a020480360372443c22e3791949e7c4e07))
* added support for wasm2js ([ce07134](https://github.com/arctic-hen7/perseus/commit/ce071345c32d4a6780ab4c05264db76b38973584))
* improved `template2` ergonomics ([c238df9](https://github.com/arctic-hen7/perseus/commit/c238df9e754848fa570f36013b775c588b588e9e))
* made `web_log!` cross-platform and only needing perseus ([b7e8389](https://github.com/arctic-hen7/perseus/commit/b7e838973fea48e3c844c79195dad2b384d3a3d0))
* passed reload server info to client ([27880a5](https://github.com/arctic-hen7/perseus/commit/27880a5373bbec591893f1418e1fe5dce0d9c165))
* set up functional plugin actions for global state ([6aa45aa](https://github.com/arctic-hen7/perseus/commit/6aa45aa06f1c99ad99477a8c746d15b2b5e499a5))
* typed options system ([#130](https://github.com/arctic-hen7/perseus/issues/130)) ([ccd4c43](https://github.com/arctic-hen7/perseus/commit/ccd4c438fd54511341537740ee214b5c28d2e42d))
* **a11y:** added route announcer ([76c0930](https://github.com/arctic-hen7/perseus/commit/76c093065d6021817326092bb9ed47f4f4084e72)), closes [#124](https://github.com/arctic-hen7/perseus/issues/124)
* **cli:** added custom engines support ([b31855e](https://github.com/arctic-hen7/perseus/commit/b31855eb9f97653d5b67ab278f341213fb1455f7)), closes [#59](https://github.com/arctic-hen7/perseus/issues/59)
* **plugins:** added functional actions for exporting error pages ([36abcc1](https://github.com/arctic-hen7/perseus/commit/36abcc11634cb1ffc8235c6498abd5d6b3140a8b))


### Bug Fixes

* added `Debug` for `TranslationArgs` ([51422ed](https://github.com/arctic-hen7/perseus/commit/51422ed792ec604a1359e0af5631ee85934968f5))
* added utf-8 encoding to default html shell ([fce0db8](https://github.com/arctic-hen7/perseus/commit/fce0db8b6643ca6723328f11d86cf662e88afacf))
* fixed active/global state fallbacks ([193f733](https://github.com/arctic-hen7/perseus/commit/193f733798ff5dc909a30eaf5f71b329756d4e03))
* fixed cli in development for watching ([2941f77](https://github.com/arctic-hen7/perseus/commit/2941f77e8c8259dd9488807a8b40c4bad31145fb))
* fixed clippy lint issues with `wasm-bindgen` ([b2f67e6](https://github.com/arctic-hen7/perseus/commit/b2f67e617ce1b05ff93acaba58d0de39fc87cd21)), closes [rustwasm/wasm-bindgen#2774](https://github.com/rustwasm/wasm-bindgen/issues/2774)
* fixed exporting with typed options system ([18f54a9](https://github.com/arctic-hen7/perseus/commit/18f54a9a27696d46af40762b51d509920dc9403a))
* fixed hsr in deployment ([ec52b1c](https://github.com/arctic-hen7/perseus/commit/ec52b1c97d0aeafa53bfdca805de3690720a46d4))
* fixed includes in docs ([89b420d](https://github.com/arctic-hen7/perseus/commit/89b420defc74411c9f1cad6cb875743ccf63ca6f))
* fixed margin errors in website ([59525b4](https://github.com/arctic-hen7/perseus/commit/59525b49b5a67faa563148ab1a7dcfb8c6927749))
* fixed router ([2260885](https://github.com/arctic-hen7/perseus/commit/2260885d01f550880659d781165b1238a86c39af))
* fixed some trait scoping ([d8416e2](https://github.com/arctic-hen7/perseus/commit/d8416e2d4cb6a88ef93243f6224e2632ab7356dc))
* fixed typo ([48e194b](https://github.com/arctic-hen7/perseus/commit/48e194b2dd1c98bd3c7aeb9e4a094143ab59f30c))
* fixed up tests ([6f979eb](https://github.com/arctic-hen7/perseus/commit/6f979eb4eec85c2e158524de3b730ccc251ff2fb))
* fixed warp integration ([93be5de](https://github.com/arctic-hen7/perseus/commit/93be5de564733069e6a78dea62d6b01e5ae12323))
* made hsr self-clearing ([1936b62](https://github.com/arctic-hen7/perseus/commit/1936b62bdad9ac7cfc799ea3c1648d44165f651e))
* made i18n example use the right locales function ([6a05c63](https://github.com/arctic-hen7/perseus/commit/6a05c6377d5300a47edd75c09bcfaf867e764b7f))
* made logging work again ([47fbef5](https://github.com/arctic-hen7/perseus/commit/47fbef5b4698eca42781c5f8bf4bea8a64a1718c))
* made page state store work with multiple pages in single template ([4c9c1be](https://github.com/arctic-hen7/perseus/commit/4c9c1bedef8a68b9a9d1d395b6da49b04be218a8))
* typo in default index view ([#132](https://github.com/arctic-hen7/perseus/issues/132)) ([1f1522a](https://github.com/arctic-hen7/perseus/commit/1f1522a764245d8b4b92bed516653693f6f908f5))


### Performance Improvements

* **i18n:** added experimental wasm caching ([2d1ca2d](https://github.com/arctic-hen7/perseus/commit/2d1ca2dc88d1fa7aaabdfcdbfcaecff69f0eb469))


### Code Refactorings

* added `Debug` implementations to everything ([2392daa](https://github.com/arctic-hen7/perseus/commit/2392daa06a6b80290b59adc5f17bcdc5e9c772cf))
* broke out some fn args into separate `struct`s ([1e0ed5c](https://github.com/arctic-hen7/perseus/commit/1e0ed5c554d6811def29474606ddb0a6375cff4c))
* changed default live reloading port to 3100 ([a355157](https://github.com/arctic-hen7/perseus/commit/a355157028246a537333533a8784cee2f2f812ef))
* cleaned up ([ee29ba1](https://github.com/arctic-hen7/perseus/commit/ee29ba10413e4a61bb4b077a371739a775793a0c))
* cleaned up from last refactor ([33f439c](https://github.com/arctic-hen7/perseus/commit/33f439c7d631fb3c0c0abab38800e9bb0d281e5d))
* fixed clippy lints ([2f37374](https://github.com/arctic-hen7/perseus/commit/2f373742d28e8726fd662c1aabdf9e12387e61b7))
* improved html shell readability ([#109](https://github.com/arctic-hen7/perseus/issues/109)) ([69e9f72](https://github.com/arctic-hen7/perseus/commit/69e9f7295b197ad59d41ee61c545ed6d04483520))
* made basic examples use reactive state ([1570e5d](https://github.com/arctic-hen7/perseus/commit/1570e5d57c61d7a1c87b848ffc09f35763d11a8c))
* made examples use typed options system ([02c3c03](https://github.com/arctic-hen7/perseus/commit/02c3c033b5398db3577bed86fb812d23a6718110))
* made live reloading have access to render context ([b9b608a](https://github.com/arctic-hen7/perseus/commit/b9b608a8e3d88604f95bd54350cc985d376f08dd)), closes [#121](https://github.com/arctic-hen7/perseus/issues/121)
* minor code improvements ([#110](https://github.com/arctic-hen7/perseus/issues/110)) ([2c0d344](https://github.com/arctic-hen7/perseus/commit/2c0d344950fc7a30bd1b5c6a5384b2ce3bfd7758))
* moved header setting and static content examples into own ([0449fea](https://github.com/arctic-hen7/perseus/commit/0449fea10ccc59d92b7188dc26d709b36d81c8d0))
* moved html shell into one struct ([832e269](https://github.com/arctic-hen7/perseus/commit/832e269259f258d0624b234b670ab8b2cf8cd22a))
* moved router into core ([b1c4746](https://github.com/arctic-hen7/perseus/commit/b1c4746cc9164ddaefcee8b8ab4f8ef307d2234f))
* moved showcase example into state generation example ([25b9808](https://github.com/arctic-hen7/perseus/commit/25b98083e7b10aae74c1967fa242d6e0cfef6ec5))
* partitioned examples and moved in tests ([33887ab](https://github.com/arctic-hen7/perseus/commit/33887ab46ccfac1520c819e3118e91123595e726))
* reduced allocations in engine server ([3422949](https://github.com/arctic-hen7/perseus/commit/34229498d3645b58b25ca4ab8f8cafb12114ef19))
* renamed `template_with_rx_state` to `template2` ([2956009](https://github.com/arctic-hen7/perseus/commit/2956009bdcd36efa86a37f736fb3affa0d189981))
* renamed `template2` to `template_rx` ([2d99a9a](https://github.com/arctic-hen7/perseus/commit/2d99a9a3cf95539c38591e9510e8770877e5bcb8))
* renamed global state to page state store ([3b2401b](https://github.com/arctic-hen7/perseus/commit/3b2401b2aa596a9b64c9c2f2dd71742101adc00a)), closes [#119](https://github.com/arctic-hen7/perseus/issues/119)
* restructed files ([1700dcb](https://github.com/arctic-hen7/perseus/commit/1700dcb0c785724e83688607d79da28fe24b5fbd))
* restructured logic revalidation example to show types clearly ([cbf2629](https://github.com/arctic-hen7/perseus/commit/cbf2629bebf1d7147c888a154193374e8af994b3))
* updated to typed options system in example basis ([7a7dd6c](https://github.com/arctic-hen7/perseus/commit/7a7dd6cde1e28639ce93fe09bd1ffa502f651735))
* **examples:** split `rx_state` into multiple examples ([d58dd29](https://github.com/arctic-hen7/perseus/commit/d58dd296ffda2ca320a249aad9c9a95834746676))
* restructured tests/examples ([14b4b46](https://github.com/arctic-hen7/perseus/commit/14b4b465a1a33f827cb0baf4b114fa49ce43689e))


### Documentation Changes

* added docs for examples ([16c63ef](https://github.com/arctic-hen7/perseus/commit/16c63ef8c279f327f786b4c18b1d2609fc88cac7))
* added section on how to build bleeding-edge cli ([0e6eb5d](https://github.com/arctic-hen7/perseus/commit/0e6eb5d9ddfca941ebf27900b0188b296425fed5))
* edited hsr blog post ([57913b4](https://github.com/arctic-hen7/perseus/commit/57913b459169b1019697ae4de14abb356af0a223))
* fix broken link, typos and add 0.3.3 as stable version ([#129](https://github.com/arctic-hen7/perseus/issues/129)) ([9f3d5a2](https://github.com/arctic-hen7/perseus/commit/9f3d5a220d24ec2bf9f764b4a266afe0e8b889e5))
* fixed readme links ([82dda10](https://github.com/arctic-hen7/perseus/commit/82dda1065e019a7e1f8416400f53e37011f621d3))
* fixed the last bad link in the readme ([5b575cb](https://github.com/arctic-hen7/perseus/commit/5b575cb96b01db7af4453657008ad21f08cd2abc))
* removed outdated text in the readme ([301cfd3](https://github.com/arctic-hen7/perseus/commit/301cfd3ed2bb13d76e2e00980e840f7bc08793e9))
* updated docs for typed options system ([bd0710d](https://github.com/arctic-hen7/perseus/commit/bd0710d57f78808956d86c02ff564e20e5fd6f41))
* updated template macro docs for no args ([c0c30b6](https://github.com/arctic-hen7/perseus/commit/c0c30b6acd83c46dbe0fd307448d4543b36527db))
* updated the readme ([8b6e142](https://github.com/arctic-hen7/perseus/commit/8b6e142d46fdd0eee89ca5bbb9390077f2f48186))
* updated upgrade guide ([286a29c](https://github.com/arctic-hen7/perseus/commit/286a29c34e2999849a581f1ac2a076524af507ff))
* **blog:** added first draft of hsr post ([78fef13](https://github.com/arctic-hen7/perseus/commit/78fef13a5937009d5fcd9201431699b86014b822))
* **book:** added route announcer docs ([30d0839](https://github.com/arctic-hen7/perseus/commit/30d0839641217ca6dd9b7cb0238e14e9f9c7c00e))
* **book:** clarified dark mode with global state ([261ab84](https://github.com/arctic-hen7/perseus/commit/261ab84b8466d06d342edff10dc4b80a4b785229))
* **book:** documented idb state system ([68a467c](https://github.com/arctic-hen7/perseus/commit/68a467c4124707b3e5f70bb36e42d66966492326))
* **book:** documented wasm/js sizing differences ([578b08b](https://github.com/arctic-hen7/perseus/commit/578b08bc78849341304d206560ce8ba7475ed2fd))
* **book:** fixed examples inclusions and partioned version ([55c21eb](https://github.com/arctic-hen7/perseus/commit/55c21ebe1cff965641808d4850803641898b76fb))
* **book:** rewrote fetching docs ([cba35e6](https://github.com/arctic-hen7/perseus/commit/cba35e6b12964cc989e7b25beece9ae5ce2ade20))
* **book:** updated debugging docs ([7adf684](https://github.com/arctic-hen7/perseus/commit/7adf684765ac11a5c661f6c05d27d0e88b120594))
* **book:** updated state/state generation docs for new examples layout ([13c1f20](https://github.com/arctic-hen7/perseus/commit/13c1f20171ab9dc7967d2ccc46b3fd26f0a463b8))
* **book:** updated tutorials for new examples layout ([28f1af1](https://github.com/arctic-hen7/perseus/commit/28f1af17387dd0e92c907da7a7969775596185c5))
* **book:** wrote docs on custom engines ([4741b67](https://github.com/arctic-hen7/perseus/commit/4741b679afa831f4e8b331a35d7b56088d09dd82))
* **book:** wrote docs on live reloading and hsr ([4cf292f](https://github.com/arctic-hen7/perseus/commit/4cf292ff8a48abf43e991c40c8f6d95e206c1f04))
* **book:** wrote docs on reactive state ([f5a7fbd](https://github.com/arctic-hen7/perseus/commit/f5a7fbdb37f0716016986ac1383b2498beae6588))
* **book:** wrote docs on state thawing ([2d67a40](https://github.com/arctic-hen7/perseus/commit/2d67a40e8054ea7831f916fb13e955c398efa1f4))
* **contrib:** finalized shift to tribble ([09af5c6](https://github.com/arctic-hen7/perseus/commit/09af5c683fc8b22163381220be6bbee8ad35187b))
* **contrib:** hid old contributing docs in details block ([6f850c2](https://github.com/arctic-hen7/perseus/commit/6f850c2f7b86a9cb9a95c0e5639c0c9af0bec25f))
* **examples:** added some new examples and a template for more ([07289f6](https://github.com/arctic-hen7/perseus/commit/07289f63722c154d4945471711e5b674f3ef2354))
* **website:** added comparison note for sycamore and perseus ([bc4f821](https://github.com/arctic-hen7/perseus/commit/bc4f821ee4fabcfab622a27c90ff89e3dfbf835c))
* added contrib docs with tribble ([bc8fc3d](https://github.com/arctic-hen7/perseus/commit/bc8fc3d314bb3bab429754090f8736430900ee03))
* added example to styling docs ([606f635](https://github.com/arctic-hen7/perseus/commit/606f6352c93d3d1b3a115c0f54921d9292d2e0c1))
* added missing link to wasm website ([#117](https://github.com/arctic-hen7/perseus/issues/117)) ([a0dad42](https://github.com/arctic-hen7/perseus/commit/a0dad42a0c2b9767daee14686dd404f2f8bf74c1))
* finalized contributing repo docs ([6aece16](https://github.com/arctic-hen7/perseus/commit/6aece167bb64a25bbff67dafe08690512fc1762d))
* fixed link to discord in issue creation links ([2c14352](https://github.com/arctic-hen7/perseus/commit/2c1435270c868739cf73bda101a2993731589ff9))
* merged `next` with `0.3.x` ([487ce2b](https://github.com/arctic-hen7/perseus/commit/487ce2bf85e0abce0d83434e2cd2ddc1b33f72b6))
* miscellaneous fixes to tribble docs ([c0b5f55](https://github.com/arctic-hen7/perseus/commit/c0b5f55eee5984e56debd949e02ad14ee6b51063))
* restructured and wrote core principles docs ([9ee419e](https://github.com/arctic-hen7/perseus/commit/9ee419eefd6329b94edfd726da0af491346af4e7))
* **contrib:** fixed broken link ([9e5c9b3](https://github.com/arctic-hen7/perseus/commit/9e5c9b3664ff33a5368c11072d82811915dcd1de))
* **tribble:** cleaned up section/endpoint naming ([891cd44](https://github.com/arctic-hen7/perseus/commit/891cd4471a3a4400c0b1fa2bab78871c4d0d56bf))
* **tribble:** fixed minor copy-paste error ([b638d25](https://github.com/arctic-hen7/perseus/commit/b638d25a5fcd237fedbd7f70c710d6739bc87588))

### [0.3.3](https://github.com/arctic-hen7/perseus/compare/v0.3.2...v0.3.3) (2022-02-15)


### Bug Fixes

* fixed actix web beta issues ([2c2e460](https://github.com/arctic-hen7/perseus/commit/2c2e46085e55da8d3610902de7c6e0270f063e41)), closes [#125](https://github.com/arctic-hen7/perseus/issues/125)

### [0.3.2](https://github.com/arctic-hen7/perseus/compare/v0.3.1...v0.3.2) (2022-01-11)


### Features

* added ability to export error pages ([624034b](https://github.com/arctic-hen7/perseus/commit/624034bd0788d175aaf60776968cff86d89fb5f4)), closes [#94](https://github.com/arctic-hen7/perseus/issues/94)
* added external request caching ([3ecad15](https://github.com/arctic-hen7/perseus/commit/3ecad150a20f4326a981563d43517bef53874a09))
* modernized host/port setting for `perseus serve` ([19bd87e](https://github.com/arctic-hen7/perseus/commit/19bd87e6c0f9780af572c79a88025ae0b741c4f2)), closes [#107](https://github.com/arctic-hen7/perseus/issues/107)


### Bug Fixes

* **website:** fixed formatting errors ([4139df9](https://github.com/arctic-hen7/perseus/commit/4139df9d055be41c55b8b92abb831ee20ac60af5))


### Documentation Changes

* updated docs to reflect host/port setting changes ([a930ae2](https://github.com/arctic-hen7/perseus/commit/a930ae2002f6ba2df1f4b93b73d64c8fb20a3f2a))

### [0.3.1](https://github.com/arctic-hen7/perseus/compare/v0.3.0...v0.3.1) (2022-01-02)


### Features

* **website:** added highlighting for dockerfiles ([81e2066](https://github.com/arctic-hen7/perseus/commit/81e206605ea72d1c3c24071ee5105963939475cd))
* re-exported `spawn_local` for convenience ([184381f](https://github.com/arctic-hen7/perseus/commit/184381fbfb27baeb2c7399d5ce94c2d60643b07e))
* **cli:** added basic hot reloading ([b4c93f0](https://github.com/arctic-hen7/perseus/commit/b4c93f0a8202422c2f64779d87e7bcc6bcfb217a))
* **cli:** added hot reloading ([61696b3](https://github.com/arctic-hen7/perseus/commit/61696b32becdb925c5e43dcc60c3d4f9dfa51fc8))
* **cli:** added support for wasm profiling builds ([c2de025](https://github.com/arctic-hen7/perseus/commit/c2de025eb858c50339631781ea810b54651c2242))
* add tokio ([#102](https://github.com/arctic-hen7/perseus/issues/102)) ([150fda8](https://github.com/arctic-hen7/perseus/commit/150fda8062e3bd5c97bb57d759b383b64e43d84b))
* made static generation errors display causes ([ab7742a](https://github.com/arctic-hen7/perseus/commit/ab7742a6733dae977bddde86ceaea3e13301cd86)), closes [#101](https://github.com/arctic-hen7/perseus/issues/101)
* **cli:** added inbuilt server for exported apps ([8274678](https://github.com/arctic-hen7/perseus/commit/82746784c2a803b3e41a56f740840767b0d0de10))


### Bug Fixes

* **cli:** made watcher ignore `.git/` as well ([1a7f6ed](https://github.com/arctic-hen7/perseus/commit/1a7f6edccc988dbf4e791853426d434a5066002a))
* **website:** made github button transition work ([efcf16f](https://github.com/arctic-hen7/perseus/commit/efcf16f3532f99958b7126234e0541b48a310ff6))
* added missing cli argument docs ([7c9fb4a](https://github.com/arctic-hen7/perseus/commit/7c9fb4ad050a71675cdee038675689239764cc31))
* **cli:** used `--dev` by default with `wasm-pack` ([55cc681](https://github.com/arctic-hen7/perseus/commit/55cc681650892fe87a07974378d795ee5b7d090b))
* **deps:** locked `indicatif` to `v0.17.0-beta.1` ([5b979bb](https://github.com/arctic-hen7/perseus/commit/5b979bb4589f3f7d758788fb43d906a460b70567))
* **engine:** fixed incomplete error messages ([e445e56](https://github.com/arctic-hen7/perseus/commit/e445e5682ca96aa44918e3a527a5940207ea3731))


### Documentation Changes

* added cargo corruption to common pitfalls ([9fe2b27](https://github.com/arctic-hen7/perseus/commit/9fe2b273be0a7010721620d450c7be42eda194d5))
* added docker deployment docs ([#98](https://github.com/arctic-hen7/perseus/issues/98)) ([93f2c4b](https://github.com/arctic-hen7/perseus/commit/93f2c4b3fd270e353f6387085aed8e82ed0b7958))
* added docs for cli watching ([4a250e9](https://github.com/arctic-hen7/perseus/commit/4a250e9585f34d7cd13b3d92d2c002b692460227))
* added new example for fetching data ([6b08ffe](https://github.com/arctic-hen7/perseus/commit/6b08ffe8e784818653ad5e4f3556da26f49a5b08)), closes [#96](https://github.com/arctic-hen7/perseus/issues/96)
* added preliminary `define_app!` advanced docs ([69721a6](https://github.com/arctic-hen7/perseus/commit/69721a6e625b8d99461519e310a33eecfe3b501d))
* fixed code in docker docs ([ac5aaf9](https://github.com/arctic-hen7/perseus/commit/ac5aaf9ae0a036167876e467e6324f270e1fda72))
* made changelog more readable ([12ecc92](https://github.com/arctic-hen7/perseus/commit/12ecc92c7dc6361c0837169cdff464ac04d26fa5))
* merged `0.3.0` and `next` ([9f17624](https://github.com/arctic-hen7/perseus/commit/9f176243247e525715a6952c848ea50830f80e1e))
* merged last changes into `next` ([5ab9903](https://github.com/arctic-hen7/perseus/commit/5ab99033fa1d186e394219bce8146d933a2eb88d))
* updated contrib docs for new site command ([9246c12](https://github.com/arctic-hen7/perseus/commit/9246c129f8358f3596e3df99b2d7f6ebe054ea0a))

## [0.3.0](https://github.com/arctic-hen7/perseus/compare/v0.3.0-rc.1...v0.3.0) (2021-12-21)


### Documentation Changes

* removed beta warning ([4e4cc18](https://github.com/arctic-hen7/perseus/commit/4e4cc18b1876c49e6235c0fbc09890fe57b285bf))

<details>
<summary>v0.3.0 Beta Versions</summary>

## [0.3.0-rc.1](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.26...v0.3.0-rc.1) (2021-12-21)


### Documentation Changes

* updated to reflect that no hydration doesn't change Lighthouse scores ([aabc247](https://github.com/arctic-hen7/perseus/commit/aabc2477436a5fff2062eda31ae7c6662c43b95a))

## [0.3.0-beta.26](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.25...v0.3.0-beta.26) (2021-12-21)


### Code Refactorings

* switched default server integration ([eed2cc0](https://github.com/arctic-hen7/perseus/commit/eed2cc08519fe73a5482e8c7482e20ab0e27df45))

## [0.3.0-beta.25](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.24...v0.3.0-beta.25) (2021-12-21)


### Features

* **i18n:** made locale redirection much faster ([61aa406](https://github.com/arctic-hen7/perseus/commit/61aa406eef38136a9067e5e5667b7057aa5a25aa)), closes [#61](https://github.com/arctic-hen7/perseus/issues/61)


### Bug Fixes

* **website:** fixed version issues ([85d8236](https://github.com/arctic-hen7/perseus/commit/85d82362e8aa0a9c259c7e8df97119b5216ba715))
* made hydration opt-in ([4fd38a6](https://github.com/arctic-hen7/perseus/commit/4fd38a6e0426406fe29881f949451a6dddc24331))
* **website:** fixed tailwind not purging ([bd58daa](https://github.com/arctic-hen7/perseus/commit/bd58daa22596858794430ad0b2262082c8678a72))
* disabled hydration on website ([3f2d110](https://github.com/arctic-hen7/perseus/commit/3f2d1101b3f55e14f6d871ed6f603a7614b32d38))
* pinned website version to beta 22 ([5141cec](https://github.com/arctic-hen7/perseus/commit/5141cecc668166fe6c85706d8d343330cb66e837))
* properly disabled hydration on website ([65009fa](https://github.com/arctic-hen7/perseus/commit/65009fad04e54051e923f8d1d5cc1d1cc8751368))


### Documentation Changes

* documented hydration ([c22a5f5](https://github.com/arctic-hen7/perseus/commit/c22a5f534e0d82bf76f9b4b9de635278159989c5))


### Code Refactorings

* removed `path_prefix` from `FsTranslationsManager` ([ed48f3d](https://github.com/arctic-hen7/perseus/commit/ed48f3d31396f716c0f977ddb20c352b099aca17))

## [0.3.0-beta.24](https://github.com/arctic-hen7/perseus/compare/v0.3.0-beta.23...v0.3.0-beta.24) (2021-12-17)


### Features

* made hydration the default ([00258dd](https://github.com/arctic-hen7/perseus/commit/00258dd814f9d9b84b7725f39611600d7c6bd796))

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

</details>

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
