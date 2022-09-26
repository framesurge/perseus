# Migrating from v0.3.3

Perseus v0.3.4 added a *huge* number of new features to do with the new [reactive state platform](:reference/state/rx), but maintains backward-compatibility with (almost) all uses of v0.3.3. If you're currently running v0.3.0-0.3.3, here's how to upgrade! Note that any sites you've built with v0.3.3 should still work fine.

1. Update your `Cargo.toml` dependencies for `perseus` to `0.3.6` (to force the upgrade).
2. Upgrade the Perseus CLI with `cargo install perseus-cli --version 0.3.6`.
3. Run `perseus clean` to remove the old `.perseus/` directory.
4. Update your code for the new [reactive state platform](:reference/state/rx) if you want to!

Perseus v0.3.6 also changed several idioms, particularly deprecating `define_app!` in favor of the more versatile `PerseusApp`. This change isn't compulsory, and no warning will appear yet, but it's recommended that you upgrade soon. You can see examples of the new idioms [here](https://github.com/arctic-hen7/perseus/tree/main/examples).

## If You've Ejected

If you were running Perseus v0.3.3 and had ejected, here are the steps you should take in addition to the above.

1. Rename `.perseus/` to `.perseus.old/`.
2. Run `perseus build` to create a new `.perseus/` directory.
3. Apply your changes to the new directory.
4. Delete `.perseus.old/` when you're done.
5. Confirm everything works with `perseus serve`.

This may seem arduous, but v0.3.4 includes extreme restructuring of the innards in `.perseus/`, and it's typically simpler to re-apply your own changes to the new system than it is to apply Perseus' updates to your existing directory (though you could do this if you really wanted).
