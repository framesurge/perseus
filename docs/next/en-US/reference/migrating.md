# Migrating from v0.3.x

Perseus v0.4.x added several breaking changes, along with a full migration to Sycamore v0.8.x, which requires some rewriting of your view code, most of which is covered on the [Sycamore website](https://sycamore-rs.netlify.app).

**Warning:** Perseus v0.4.x is still in beta, so there may still be several bugs! Additionally, both the Sycamore and Perseus APIs are potentially subject to significant changes during the beta period, so you may be making major changes to your app quite often.

1. Update your `Cargo.toml` dependencies for `perseus` to `0.4.0-beta.8`.
2. Upgrade the Perseus CLI with `cargo install perseus-cli --version 0.4.0-beta.8`.
3. Run `perseus clean` to remove the old `.perseus/` directory.
4. Update your view code for Sycamore's new version (mostly including adding a `cx` parameter as the first argument of every function that returns a `View<G>`).

## If You've Ejected

If you were running Perseus v0.3.x and had ejected, here are the steps you should take in addition to the above.

1. Rename `.perseus/` to `.perseus.old/`.
2. Run `perseus build` to create a new `.perseus/` directory.
3. Apply your changes to the new directory.
4. Delete `.perseus.old/` when you're done.
5. Confirm everything works with `perseus serve`.

This may seem arduous, but v0.4.x includes extreme restructuring of the innards in `.perseus/`, and it's typically simpler to re-apply your own changes to the new system than it is to apply Perseus' updates to your existing directory (though you could do this if you really wanted).
