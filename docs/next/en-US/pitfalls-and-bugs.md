# Common Pitfalls and Known Bugs

This document is a list of common pitfalls and known bugs in Perseus, and will be updated regularly. If you're having an issue with Perseus, check through this list to see if your problem already has a solution.

## `perseus serve` fails with no error message on Arch Linux

If you're running Arch Linux or a derivative (e.g. Manjaro), you're very likely to encounter a bug in which `perseus serve` stops with no error messages whatsoever, and your app doesn't build properly. This is tracked by [issue #78](https://github.com/arctic-hen7/perseus/issues/78), and is due to an issue in OpenSSL that causes a segmentation fault in `wasm-pack` (see [this issue](https://github.com/rustwasm/wasm-pack/issues/1079)). Right now, the only solution to this is to downgrade `wasm-pack` by running `cargo install wasm-pack --version "0.9.1"`, which seems to fix the problem.
