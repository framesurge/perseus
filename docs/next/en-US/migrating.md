# Migrating from v0.3.x

Perseus v0.4.x added a significant number of breaking changes, as almost the entire Perseus core was rewritten from scratch, along with a full migration to Sycamore v0.8.x, which requires some rewriting of your view code, most of which is covered on the [Sycamore website](https://sycamore-rs.netlify.app).

**Warning:** Perseus v0.4.x is now in its final beta period, meaning new features are probably not going to be added until v0.4.0 goes stable. However, due to the full rewrite, we want to make sure there are no outstanding bugs before pushing the full stable release. Please report even the smallest bugs you encounter to us on GitHub, and we can make Perseus v0.4.0 the best it can be.

1. Restructure your `Cargo.toml` to reflect the new dependency-splitting format (which splits engine-side dependencies from those only needed in the browser). See [here](https://github.com/framesurge/perseus/tree/main/examples/core/basic/Cargo.toml) for an example. Note that this will involve adding a server integration for use, like `perseus-warp` (on which you'll probably want to enable the `dflt-server` feature).
2. Upgrade the Perseus CLI with `cargo install perseus-cli --version 0.4.0-beta.19`.
3. Delete the old `.perseus/` directory (this is no longer needed).
4. Rename your `lib.rs` file to `main.rs` and delete `.perseus/` (it's been removed entirely!).
5. Change the `#[perseus::main]` attribute on the function in `main.rs` to be `#[perseus::main(perseus_axum::dflt_server)]` (replace `perseus_axum` with whatever server integration you decide to use).
6. Update your view code for Sycamore's new version (mostly including adding a `cx` parameter as the first argument of every function that returns a `View<G>`).
7. Add `.build()` to the bottom of all `Template::new` calls, and change those to `Template::build`.
8. Remove `#[perseus::template_rx]` and `#[perseus::template]` entirely. Templates that take reactive state should use `#[auto_scope]` now.
9. Remove lifetimes from templates that take state, as these can be handled by `#[auto_scope]` now. See the `#[auto_scope]` docs for more details, including on how to do this manually.
10. Change all `.template()` calls (on `Template`, not `PerseusApp`) to be `.view()` (for templates that take no state), `.view_with_unreactive_state()` for unreactive state, or `.view_with_state()` for reactive state.
11. Change `#[make_rx]` to `#[derive(Serialize, Deserialize, Clone, ReactiveState)]`. You'll need to derive `UnreactiveState` on unreactive state types as well.
12. Replace all macros like `#[build_state]` with `#[engine_only_fn]` (these have all been combined and simplified).
13. Replace `RenderFnResult`/`RenderFnResultWithCause` with your own error types. For blamed types (i.e. everything other than build paths), your error should be wrapped in `BlamedError`. If you don't actually need to return an error, you can return your state directly now, without any `Result`!
14. Replace uses of `blame_err!` with manually returning a `BlamedError`. This API has been dramatically simplified.
15. Make your header setting functions take a scope `cx` as their first argument.
16. Replace your `ErrorPages` with [`ErrorViews`](=prelude/struct.ErrorViews@perseus) (see the documentation of this `struct`, and the `core/error_views` example for further details).
17. Update any manual destructuring of your state to use references (e.g. `let content = state.content;` -> `let content = &state.content;`).
18. Change `MyStateRx<'b>` to `&MyStateRx` (you'll need `&'b MyStateRx` if you're not using `#[auto_scope]`).
19. In your `PerseusApp`, change all the times you've provided functions to actually *call* those functions (e.g. `.template(crate::templates::index::get_template)` -> `.template(crate::templates::index::get_template())`).
20. Change any `#[cfg(target_arch = "wasm32")]` instances to be `#[cfg(client)]`, and any `#[cfg(not(target_arch = "wasm32"))]` ones to say `#[cfg(engine)]`.
21. Change any head functions that take state to use `.head_with_state()` (the same applies for header setting functions, which can now take state too).
22. Update your code for any smaller breaking changes that might affect you, as per the [changelog](https://github.com/framesurge/perseus/blob/main/CHANGELOG.md).
23. Run `cargo update` and then `perseus clean && perseus build` to get everything up to date and ensure that your app works! (This might take a while the first time.)

We realize that this is a mammoth number of breaking changes, and there will be several more if you're a plugin developer. However, Perseus v0.4.0 brings extraordinary levels of performance and ergonomics to Perseus, removing old quirks and streamlining the internals massively. With the introduction of the new capsules system, Perseus is on par with the most powerful frontend frameworks in the world. If you're having any trouble with updating, please do not hesitate to let us know on [Discord](https://discord.com/invite/GNqWYWNTdp), and we'll happily help you out as soon as we can!

Note that, in order to make `rust-analyzer` etc. work with the new version of Perseus, you'll need to tell it to compile your app for the engine-side by default. You can do this by adding a `.cargo/config.toml` file to the root of your project with the following contents:

```toml
[build]
rustflags = [ "--cfg", "engine" ]
```

If you later want to work on a browser-only part of your app, you can just change `engine` to `client` while you work, and Rust will compile your code accordingly!

**IMPORTANT:** One of the more subtle things that changed in beta 12 of v0.4.0 is that the `path` provided to `get_build_state` no longer includes the template path! This means what was once, say, `docs/foo` would now just be `foo`, which can cause a lot of problems for existing code. This was not a decision taken lightly, but the original choice was made due to routing constraints at the time, which no longer exist, and it has been decided that this will be better for future users. Until you've updated all your code to account for this, you may want to add the following compatibility snippet to the top of your `get_build_state` functions:

```rust
let path = format!("<template-name>/{}", path);
let path = path.strip_suffix("/").unwrap_or(&path).to_string();
```

Of course, replace `<template-name>` with the name you put in `Template::build()`, like `docs`, or `posts`. The suffix strip is there to handle index pages to prevent trailing forward slashes that might otherwise confuse your application. Eventually, you should adapt your code to work with the new `path` system, but this provides a good fallback to at least make your app work until you have the time for that larger change.

## If You've Ejected

If you were running Perseus v0.3.x and had ejected, your app's structure is likely to change significantly, as Perseus v0.4.x no longer uses `.perseus/`! For example, you can now directly modify the server Perseus runs, allowing you to add your own API routes trivially! (See the [custom server example](https://github.com/framesurge/perseus/tree/main/examples/core/custom_server) for details.)

The migration process you follow will be highly unique to your app's structure, though most common use-cases should be covered by the custom server example, linked above. If you need any further help, feel free to ask in GitHub discussions or on [Discord](https://discord.com/invite/GNqWYWNTdp), and we're happy to help in any way we can!
