# Snooping on the CLI

Most of the time, it's fine to run `perseus serve` and enjoy the results, but sometimes you'll need to delve a little deeper and see some more detailed logs. Then you need `perseus snoop`. This command will run one of the lower-level steps the Perseus CLI runs, but in such a way that you can see everything it does. The time you'll use this the most is when you have a `dbg!()` call in the static generation process (e.g. in the *build state* strategy) and you want to actually see its output, which neither `perseus build` nor `perseus serve` will let you do.

## `perseus snoop build`

This snoops on the static generation process, which is half of what `perseus build` does. You can use this to see the outputs of `dbg!()` calls in your build-time code.

## `perseus snoop wasm-build`

This snoops on the `wasm-pack` call that compiles your app to Wasm. There aren't really any use cases for this outside debugging strange errors, because Perseus calls out to this process without augmenting it in any way, so your code shouldn't impact this really at all (unless you're using some package that can't be compiled to Wasm).

## `perseus snoop serve`

This snoops on the server, which is useful if you're hacking on it, or if you're getting errors from it (e.g. panics in the server will only appear if you run this). Crucially though, this expects to be working with a correct build state, which means **you must run `perseus build` before running this command**, otherwise all sorts of things could happen. If such things do happen, you should run `perseus clean --dist`, and that should solve things.
