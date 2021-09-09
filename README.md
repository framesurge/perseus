<h1 style="text-align: center;">Perseus</h1>

[Book][book] • [Crate Page][crate] • [API Documentation][docs] • [Contributing][contrib]

Perseus is a blazingly fast frontend web development framework built in Rust with support for major rendering strategies, reactivity without a virtual DOM, and extreme customizability. It wraps the lower-level capabilities of [Sycamore](https://github.com/sycamore-rs/sycamore) and provides a NextJS-like API!

-   ✨ Supports static generation (serving only static resources)
-   ✨ Supports server-side rendering (serving dynamic resources)
-   ✨ Supports revalidation after time and/or with custom logic (updating rendered pages)
-   ✨ Supports incremental regeneration (build on demand)
-   ✨ Open build matrix (use any rendering strategy with anything else, mostly)
-   ✨ CLI harness that lets you build apps with ease and confidence
-   ✨ Full i18n support out-of-the-box with [Fluent](https://projectfluent.org)

## How to use

Check out the docs [here](https://arctic-hen7.github.io/perseus) for how to use Perseus.

## Aim

Support every major rendering strategy and provide developers the ability to efficiently create super-fast apps with Rust and a fantastic developer experience!

## Motivation

There is a sore lack of Rust frameworks for frontend development that support more than just SPAs and client-side rendering, and so Perseus was born. We need something like NextJS for WASM.

## Roadmap

### Pre-stable

These tasks still need to be done before Perseus can be pushed to v1.0.0.

-   [x] Create a custom CLI as a harness for apps without ridiculous amounts of configuration needed

*   [ ] Support custom template hierarchies
*   [ ] Support i18n out of the box
*   [ ] (Maybe) Implement custom router
*   [ ] Pre-built integrations for Actix Web (done) and AWS Lambda (todo)

### Beyond

These tasks will be done after Perseus is stable.

-   [ ] Integrations for other platforms

## Contributing

We appreciate all kinds of contributions, check out our [contributing guidelines](./CONTRIBUTING.md) for more information! Also, please be sure to follow our [code of conduct](./CODE_OF_CONDUCT.md).

You can also chat about Perseus at our [Gitter link](https://gitter.im/perseus-framework/community), or (for Sycamore-related stuff) on [our channel on Sycamore's Discord server](https://discord.com/channels/820400041332179004/883168134331256892).

## License

See [`LICENSE`](./LICENSE).

[book]: https://arctic-hen7.github.io/perseus
[crate]: https://crates.io/crates/perseus
[docs]: https://docs.rs/perseus
[contrib]: ./CONTRIBUTING.md
