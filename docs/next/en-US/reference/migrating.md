# Migrating from v0.3.x

Perseus v0.4.x added several breaking changes, along with a full migration to Sycamore v0.8.x, which requires some rewriting of your view code, most of which is covered on the [Sycamore website](https://sycamore-rs.netlify.app).

**Warning:** Perseus v0.4.x is still in beta, so there may still be several bugs! Additionally, both the Perseus API is potentially subject to significant changes during the beta period, so you may be making major changes to your app quite often.

1. Restructure your `Cargo.toml` to reflect the new dependency-splitting format (which splits engine-side dependencies from those only needed in the browser). See [here](https://github.com/framesurge/perseus/tree/main/examples/core/basic/Cargo.toml) for an example. Note that this will involve adding a server integration for use, like `perseus-warp`.
2. Upgrade the Perseus CLI with `cargo install perseus-cli --version 0.4.0-beta.9`.
3. Delete the old `.perseus/` directory (this is no longer needed).
4. Rename your `lib.rs` file to `main.rs`.
5. Update each error page instantiation function to provide, as another argument, a function that returns a Sycamore `View<G>` for the document metadata of that error page (e.g. title).
6. Change the `#[perseus::main]` attribute on the function in `main.rs` to be `#[perseus::main(perseus_warp::dflt_server)]` (replace `perseus_warp` with whatever server integration you decide to use).
7. Update your view code for Sycamore's new version (mostly including adding a `cx` parameter as the first argument of every function that returns a `View<G>`).
8. Update your code for any smaller breaking changes that might affect you, as per the [changelog](https://github.com/framesurge/perseus/blob/main/CHANGELOG.md).
9. Run `cargo update` and then `perseus build` to get everything up to date and e
ensure that your app works! (This might take a while the first time.)

## If You've Ejected

If you were running Perseus v0.3.x and had ejected, your app's structure is likely to change significantly, as Perseus v0.4.x no longer uses `.perseus/`! For example, you can now directly modify the server Perseus runs, allowing you to add your own API routes trivially! (See the [custom server example](https://github.com/framesurge/tree/main/examples/core/custom_server) for details.)

The migration process you follow will be highly unique to your app's structure, though most common use-cases should be covered y the custom server example, linked above. If you need any further help, feel free to ask in GitHub discussions or on [Discord](https://discord.com/invite/GNqWYWNTdp), and we're happy to help in any way we can!
