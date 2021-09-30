# Migrating from v0.2.x

Perseus v0.3.0 added significant architectural changes to Perseus under the hood, and also made it easier to use for you! Additionally, this update provides inbuilt deployment functionality for moving Perseus to production (though that's not recommended yet)! If you're currently running v0.2.x, here's how to upgrade!

1. Update your `Cargo.toml` dependencies for `perseus` to `0.3`.
2. Upgrade the Perseus CLI with `cargo install perseus-cli`.
3. Run `perseus clean` to remove the old `.perseus/` directory.
4. Remove any custom config managers you may have, they've been replaced by [mutable and immutable stores](./stores.md).
5. Update your code for the remaining breaking changes listed in [the CHANGELOG](https://github.com/arctic-hen7/perseus/blob/main/CHANGELOG.md).

Perseus v0.3.0 also changed a few common idioms, like breaking out the `.template()` call into a separate function `template_fn()`. This is no longer recommended, though it will still work fine. You can check out the [examples directory](https://github.com/arctic-hen7/perseus/tree/main/examples) to see how things are a bit nicer now in terms of formatting.

## If You've Ejected

If you were running Perseus v0.2.x and had ejected, here are the steps you should take in addition to the above.

1. Rename `.perseus/` to `.perseus.old/`.
2. Run `perseus build` to create a new `.perseus/` directory.
3. Apply your changes to the new directory.
4. Delete `.perseus.old/` when you're done.
5. Confirm everything works with `perseus serve`.

This may seem arduous, but v0.3.0 includes notable updates to the innards in `.perseus/`, and it's typically simpler to re-apply your own changes to the new system than it is to apply Perseus' updates to your existing directory (though you could do this if you really wanted).
