# Common Pitfalls and Known Bugs

This document is a list of common pitfalls and known bugs in Perseus, and will be updated regularly. If you're having an issue with Perseus, check through this list to see if your problem already has a solution.

## I'm getting JSON error messages...

If an error occurs during `perseus serve`, it's very possible that you'll get error messages in JSON, which are utterly unreadable. This is because of the way the server is run, the Perseus CLI needs a JSON output so that it can figure out where the server binary is. You can access the human-readable logs by 'snooping' on the output though, which you can do by running `perseus snoop serve` (but make sure you've run `perseus build` first).

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

## How do I get the bleeding-edge version of the CLI?

If you've tried to download the bleeding-edge version of the CLI with `cargo install`, using a Git dependency on the `perseus-cli` package, you've probably been hit with a whole host of errors that don't make a whole lot of sense! The reason for this is that the Perseus CLI depends on including a folder that's not checked into Git (the engine, `.perseus/`, but transformed in various ways). That means that, to build the CLI, you need to have that folder available, which `cargo install` isn't smart enough to do yet from a Git dependency.

The way you get around this is unfortunately inconvenient, you'll have to manually clone the whole Perseus repository and then build the CLI yourself. You can do that by running `bonnie setup` in the root of the Perseus repo (after you've cloned it), and then you can build the binary in `packages/perseus-cli/` -- that will give you a copy of the CLI in `target/`! Be warned though that using the bleeding-edge CLI is generally not recommended, as the interdependencies in the engine can be quite fragile, and even the smallest changes that aren't breaking usually can be breaking when you're using the bleeding-edge version of the CLI with a released version of Perseus.

## Hydration doesn't work with X

Perseus v0.3.x uses Sycamore v0.7.x, which still has several hydration bugs, so there are multiple things that won't work with it yet. In fact, as a general rule, if you're getting weird layout bugs that make absolutely no logical sense, try disabling hydration, it will often fix things at the moment.

Sycamore v0.8.0 has been released in beta to solve these problems and many others, though it also radically changes Sycamore's API, and the upgrade of Perseus (a very large and complex system) is still ongoing. Once this is complete, Perseus v0.4.0 will be released in beta, and that should fix all current hydration bugs. In other words, if you have an error solely due to hydration at the moment, you should disable it for now and wait until Perseus v0.4.0, which will hopefully fix it. When that's released, if you're still experiencing problems with hydration, please let us know!

## How do I use `[perseus::browser]` and `#[perseus::engine]` in my app?

These macros are simple proxies over the more longwinded `#[cfg(target_arch = "wasm32")]` and the negation of that, respectively. They can be easily applied to functions, `struct`s, and other 'block'-style items in Rust. However, you won't be able to apply them to statements (e.g. `call_my_function();`) , since Rust's [proc macro hygiene](https://github.com/rust-lang/rust/issues/54727) doesn't allow this yet. If you need to use stable Rust, you'll have to go with the longwinded versions in these places, or you could alternatively create a version of the functions you need to call for the desired platform, and then a dummy version for the other that doesn't do anything (effectively moving the target-gating upstream).

The best solution, however, is to switch to nightly Rust (`rustup override set nightly`) and then add `#![feature(proc_macro_hygiene)]` to the top of your `main.rs`, which should fix this.

## I'm getting really weird errors with a page's `<head>`...

Alright, this can mean about a million things. There is one that could be known to be Perseus' fault though: if you go to a page in your app, then reload it, then go to another page, and then navigate *back* to the original page (using a link inside your app, *not* your browser's back button), and there are problems with the `<head>` that weren't there before, then you should disable the `cache-initial-load` feature on Perseus, since Perseus is having problems figuring out how your `<head>` works. Typically, a delimiter `<meta itemprop="__perseus_head_end">` is added to the end of the `<head>`, but if you're using a plugin that's adding anything essential after this, that will be lost on transition to the new page. Any advanced manipulation of the `<head>` at runtime could also cause this. Note that disabling this feature (which is on by default) will prevent caching of the first page the user loads, and it will have to be re-requested if they go back to it, which incurs the penalty of a network request.
