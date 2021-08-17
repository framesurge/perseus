<h1 style="text-align: center;">Perseus</h1>

[Book][book] • [Crate Page][crate] • [API Documentation][docs] • [Contributing][contrib]

> 🚧 Perseus is **nearly** ready for v0.1.0, but is still under construction! 🚧

Perseus is a blazingly fast frontend web development framework built in Rust with support for major rendering strategies, reactivity without a virtual DOM, and extreme customizability. It wraps the lower-level capabilities of [Sycamore](https://github.com/sycamore-rs/sycamore) and provides a NextJS-like API!

- ✨ Supports static generation (serving only static resources)
- ✨ Supports server-side rendering (serving dynamic resources)
- ✨ Supports revalidation after time and/or with custom logic (updating rendered pages)
- ✨ Supports incremental regeneration (build on demand)
- ✨ Open build matrix (use any rendering strategy with anything else, mostly)

## How to use

Check out the docs [here](https://arctic-hen7.github.io/perseus) (WORK IN PROGRESS!) for how to use Perseus.

## Aim

Support every major rendering strategy and provide developers the ability to efficiently create super-fast apps with Rust and a fantastic developer experience!

## Motivation

There is a sore lack of Rust frameworks for frontend development that support more than just SPAs and client-side rendering, and so Perseus was born. We need something like NextJS for WASM.

## Roadmap

### Pre-beta

These tasks still need to be done before Perseus can be pushed to v0.1.0.

- [ ] Support providing request data to SSR renderers

### Pre-stable

These tasks still need to be done before Perseus can be pushed to v1.0.0.

- [ ] Support custom template hierarchies
- [ ] Create a custom CLI as a harness for apps without riediculous amounts of configuration needed
- [ ] Support i18n out of the box
- [ ] (Maybe) Implement custom router
- [ ] Pre-built integrations for Actix Web and AWS Lambda

### Beyond

These tasks will be done after Perseus is stable.

- [ ] Integrations for other platforms

## Contributing

We appreciate all kinds of contributions, check out our [contributing guidelines](./CONTRIBUTING.md) for more information! Also, please be sure to follow our [code of conduct](./CODE_OF_CONDUCT.md).

## License

See [`LICENSE`](./LICENSE).

[book]: https://arctic-hen7.github.io/perseus
[crate]: https://crates.io/crates/perseus
[docs]: https://docs.rs/perseus
[contrib]: ./CONTRIBUTING.md
