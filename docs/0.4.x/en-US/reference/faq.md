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

## I'm getting a 'mismatched render backends' error

This is a very rare kind of error that Perseus will emit if it knows that running your app in its current state will cause undefined behavior: it's a safeguard against far worse things happening. if you're using the reference pattern of managing your templates and/or capsules, where you define them in `lazy_static!`s, and then bring those into `.template_ref()`/`.capsule_ref()`, this problem is almost certainly caused by your using the incorrect *render backend generic*. In those statics, you have to specify a concrete value for that `G: Html` you see floating around the place. You might have chosen `DomNode`, or `SsrNode`, or maybe even `HydrateNode`, but each of these is only valid sometimes! Perseus internally knows when it uses each one, and it provides a clever little type alias that can handle all this for you: `PerseusNodeType`. If you use that, this error shoudl go away, adn your app should work perfectly!

Alternately, this error can occur if you try to do something very inadvisable, like putting a widget in a `view!` that you try to `render_to_string` on the browser-side. In fact, any attempt to render to a string in the browser that uses widgets is almost certain to trigger this exact error. This is because `PerseusNodeType` automatically resolves to `DomNode`/`HydrateNode` (depending on whether or not you've enabled the `hydrate` feature) on the browser-side, because Perseus doesn't need to do any server-side rendering there (unsurprisingly). That means, when you bring in a widget that's defined as a `lazy_static!` using `PerseusNodeType`, your `View` might be a `View<SsrNode>`, but the `MY_WIDGET.widget()` function will take that `SsrNode`, hold it for a moment, and check the type of itself, which it will find to be `PerseusNodeType`. Since `DomNode != SsrNode` and `HydrateNode != SsrNode`, it will find that you're trying to use a browser-side widget in a server-side rendered view, which is a type mismatch. Normally, this sort of thing could be caught by Rust at compilation-time, but Perseus uses some transmutes internally to make it safe to use `PerseusNodeType`, as long as it lines up with the actual type of the `View` being rendered. if you try to server-side render in the browser though, the types don't line up, and Perseus has the choice of either panicking or causing undefined behavior. To maintain safety, it panics.

Note that this doesn't mean it's actually impossible to server-side render a widget on the browser-side, you can use the functional pattern to do this easily. Rather than using `MY_CAPSULE.widget()`, just use `crate::path::to::my::widget::get_capsule().widget()`, because `get_capsule()` is generic over `G: Html` meaning it will just work with Rust's normal typing system.

If you're still getting this error, and none of these solutions make sense with what you're doing, then you've possibly encountered a rather serious Perseus bug, which we'd like to know about so we can fix it! Please report it [on GitHub](https://github.com/framesurge/perseus/issues/new/choose).

## Problem binding to `http://localhost:3100`

This means another instance of Persues is already running. The reason this talks about <http://localhost:3100> rather than port 8080 is because 3100 is where the live reload server runs by default.
