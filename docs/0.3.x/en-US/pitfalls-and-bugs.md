# Common Pitfalls and Known Bugs

This document is a list of common pitfalls and known bugs in Perseus, and will be updated regularly. If you're having an issue with Perseus, check through this list to see if your problem already has a solution.

## `perseus serve` fails with no error message on Arch Linux

If you're running Arch Linux or a derivative (e.g. Manjaro), you're very likely to encounter a bug in which `perseus serve` stops with no error messages whatsoever, and your app doesn't build properly. This is tracked by [issue #78](https://github.com/arctic-hen7/perseus/issues/78), and is due to an issue in OpenSSL that causes a segmentation fault in `wasm-pack` (see [this issue](https://github.com/rustwasm/wasm-pack/issues/1079)). Right now, the only solution to this is to downgrade `wasm-pack` by running `cargo install wasm-pack --version "0.9.1"`, which seems to fix the problem.

## I'm getting JSON error messages...

If an error occurs during `perseus serve`, it's very possible that you'll get error messages in JSON, which are utterly unreadable. This is because of the way the server is run, the Perseus CLI needs a JSON output so that it can figure out where the server binary is. You can access the human-readable logs by [snooping](:snooping) on the output though, which you can do by running `perseus snoop serve` (but make sure you've run `perseus build` first).

## Perseus doesn't work on an M1 Mac

Pending [this PR](https://github.com/rustwasm/wasm-pack/pull/1088), `wasm-pack` doesn't support the M1 Mac, which means Perseus fails. However, you can easily fix this by using the fix explained [here](https://github.com/arctic-hen7/perseus/issues/89), which entails adding the following to `.perseus/Cargo.toml` (you don't need it in your app's, just in `.perseus/`):

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

This will disable optimizations for your Wasm bundle, which prevents this issue from occurring. Note however that you'll end up with very large bundles if you compile on your M1 Mac. Again though, this issue is set to be fixed very soon.

## I want to apply X to my `Cargo.toml`, but it doesn't work

Perseus has a rather unique code structure that will foil most attempts at modifying your own `Cargo.toml`. For example, if you wanted to change the `codegen_units` in the release profile of your app, you couldn't do this in your own `Cargo.toml`, it would have no effect. The reason for this is that the code your write is actually a library that's imported by the CLI's engines, so any custom configuration has to be made directly on the engines. In other words, you'll need to apply your changes on `.perseus/Cargo.toml` instead. You can also apply customizations on the server and the builder, which are separate crates under `.perseus/`. Note that modifying `.perseus/` and retaining your changes requires [ejecting](:ejecting), or you could [write a plugin](:plugins/writing) if it's a change you make a lot.
