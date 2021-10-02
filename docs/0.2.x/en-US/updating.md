# Migrating from v0.1.x

Perseus v0.2.0 added a *huge* number of features, fixed a number of bugs, improved performance and SEO, and made Perseus significantly easier to use. If you're currently running v0.1.x, here's how to upgrade!

*Note: if possible, it's best to start a new project for Perseus v0.2.0 due to the sheer number of changes that have occurred.*

1. Update your `Cargo.toml` dependencies for `perseus` to `0.2`.
2. Remove dependencies on `perseus-actix-web` and `sycamore-router` that you might have had before (fully internal now).
3. Upgrade the Perseus CLI with `cargo install perseus-cli`.
4. Run `perseus clean` to remove the old `.perseus/` directory.
5. Change all `Rc`s to `Arc`s.
6. Change your `lib.rs` to match the [new `define_app!` macro](./define-app.md) and delete routing code (all that is now inferred, with no extra code from you)!.
7. Update your code for the remaining breaking changes listed in [the CHANGELOG]().

*Note: if you're running an older machine (pre-2015), it may be worth setting the `PERSEUS_CLI_SEQUENTIAL` environment variable to `true` to disable the CLI's new multi-threading, which may overly burden older systems. You should try it first to make sure though.*

## Upgrading from a Non-CLI Project

If you were running Perseus v0.1.x and not using the CLI, upgrading your existing app will be almost impossible due to significant infrastructural changes, and you should try to migrate your code over to a v0.2.0 CLI project, which will be faster and far easier to work with.
