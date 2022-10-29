# Common Pitfalls and Known Bugs

This document is a list of common pitfalls and known bugs in Perseus, and will be updated regularly. If you're having an issue with Perseus, check through this list to see if your problem already has a solution.

## `perseus serve` fails with no error message on Arch Linux

If you're running Arch Linux or a derivative (e.g. Manjaro), you're very likely to encounter a bug in which `perseus serve` stops with no error messages whatsoever, and your app doesn't build properly. This is tracked by [issue #78](https://github.com/arctic-hen7/perseus/issues/78), and is due to an issue in OpenSSL that causes a segmentation fault in `wasm-pack` (see [this issue](https://github.com/rustwasm/wasm-pack/issues/1079)). Right now, the only solution to this is to downgrade `wasm-pack` by running `cargo install wasm-pack --version "0.9.1"`, which seems to fix the problem.

## I'm getting JSON error messages...

If an error occurs during `perseus serve`, it's very possible that you'll get error messages in JSON, which are utterly unreadable. This is because of the way the server is run, the Perseus CLI needs a JSON output so that it can figure out where the server binary is. You can access the human-readable logs by [snooping](:reference/snooping) on the output though, which you can do by running `perseus snoop serve` (but make sure you've run `perseus build` first).

## Perseus doesn't work on an M1 Mac

Pending [this PR](https://github.com/rustwasm/wasm-pack/pull/1088), `wasm-pack` doesn't support the M1 Mac, which means Perseus fails. However, you can easily fix this by using the fix explained [here](https://github.com/arctic-hen7/perseus/issues/89), which entails adding the following to `.perseus/Cargo.toml` (you don't need it in your app's, just in `.perseus/`):

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

This will disable optimizations for your Wasm bundle, which prevents this issue from occurring. Note however that you'll end up with very large bundles if you compile on your M1 Mac. Again though, this issue is set to be fixed very soon.

## I want to apply X to my `Cargo.toml`, but it doesn't work

Perseus has a rather unique code structure that will foil most attempts at modifying your own `Cargo.toml`. For example, if you wanted to change the `codegen_units` in the release profile of your app, you couldn't do this in your own `Cargo.toml`, it would have no effect. The reason for this is that the code your write is actually a library that's imported by the CLI's engines, so any custom configuration has to be made directly on the engines. In other words, you'll need to apply your changes on `.perseus/Cargo.toml` instead. You can also apply customizations on the server and the builder, which are separate crates under `.perseus/`. Note that modifying `.perseus/` and retaining your changes requires [ejecting](:reference/ejecting), or you could [write a plugin](:reference/plugins/writing) if it's a change you make a lot.

## Cargo is putting out strange errors...

If you're getting errors along the lines of not being able to find the latest Perseus version, or you have Perseus version mismatches even though you only installed it once, you've probably got some kind of Cargo corruption. Usually, this can be fixed by running `perseus clean && cargo clean`, which will delete `.perseus/` and `target/` and start again from scratch.

However, sometimes you'll need to purge your system's Cargo cache, which can be done safely by running the following commands:

```shell
cd ~/.cargo
mkdir old
mv git old
mv registry old
```

That will archive the `git/` and `registry/` folders in `~/.cargo/`, which should resolve any corruptions. Then, just run `cargo build` in your project (after `perseus clean && cargo clean`) and everything should work! If not and you have no idea what's going on, feel free to ask on our [Discord server](https://discord.com/invite/GNqWYWNTdp)!

## I want to disable a Perseus default feature, but it's not doing anything

If you add `default-features = false` to your `Cargo.toml` and expect Perseus to adapt accordingly, you're in for a bit of a shock unfortunately! The reason for this is that the Perseus CLI isn't (yet) smart enough to know you've done this, so it will completely ignore you and press on with default features in the engine, and those settings will override your own. To disable default features, you'll need to also make these changes in `.perseus/Cargo.toml`, `.perseus/builder/Cargo.toml`, and `.perseus/server/Cargo.toml` (and you'll need to [eject](:reference/ejecting) to save your changes). In future versions, the CLI will be able to detect your preferences for this and update accordingly.

## How do I get the bleeding-edge version of the CLI?

If you've tried to download the bleeding-edge version of the CLI with `cargo install`, using a Git dependency on the `perseus-cli` package, you've probably been hit with a whole host of errors that don't make a whole lot of sense! The reason for this is that the Perseus CLI depends on including a folder that's not checked into Git (the engine, `.perseus/`, but transformed in various ways). That means that, to build the CLI, you need to have that folder available, which `cargo install` isn't smart enough to do yet from a Git dependency.

The way you get around this is unfortunately inconvenient, you'll have to manually clone the whole Perseus repository and then build the CLI yourself. You can do that by running `bonnie setup` in the root of the Perseus repo (after you've cloned it), and then you can build the binary in `packages/perseus-cli/` -- that will give you a copy of the CLI in `target/`! Be warned though that using the bleeding-edge CLI is generally not recommended, as the interdependencies in the engine can be quite fragile, and even the smallest changes that aren't breaking usually can be breaking when you're using the bleeding-edge version of the CLI with a released version of Perseus.

## Hydration doesn't work with X

Perseus v0.3.x uses Sycamore v0.7.x, which still has several hydration bugs, so there are multiple things that won't work with it yet. In fact, as a general rule, if you're getting weird layout bugs that make absolutely no logical sense, try disabling hydration, it will often fix things at the moment.

Sycamore v0.8.0 has been released in beta to solve these problems and many others, though it also radically changes Sycamore's API, and the upgrade of Perseus (a very large and complex system) is still ongoing. Once this is complete, Perseus v0.4.0 will be released in beta, and that should fix all current hydration bugs. In other words, if you have an error solely due to hydration at the moment, you should disable it for now and wait until Perseus v0.4.0, which will hopefully fix it. When that's released, if you're still experiencing problems with hydration, please let us know!

## I'm getting errors about Tokio and Wasm...

Make sure you've set `tokio` to be version `=1.20.1`, since any later versions of Tokio won't work with Perseus v0.3.x, due to internal issues. This is entirely fixed in v0.4.x, which is currently in beta. Make sure also that you're up to date with Perseus v0.3.6, if you're still using v0.3.x (`cargo update`, `perseus clean`, `cargo install perseus-cli --version 0.3.6`).
