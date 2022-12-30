# Examples

This folder contains all the examples for Perseus! These are divided into three categories: core examples, which demonstrate core features of Perseus and are used as end-to-end tests; demonstrative examples, which quickly demonstrate how to do certain things with Perseus; and comprehensive examples, which show off whole systems built with Perseus.

There's also a `.base/` folder that contains a template that new examples should be built from. If you'd like to create a new example, you should copy this directory and modify it to your needs, placing your new example in the appropriate category (this will usually be `demos` or `comprehensive`, you should only put it in `core` if you've confirmed this with a maintainer or if you've built the feature it shows off).

Each of the examples here are fully self-contained Perseus apps, though they use relative path dependencies to the bleeding edge versions of the Perseus packages in this repository. They're also designed to be used with the local, bleeding-edge version of the CLI, which can be invoked by running `bonnie dev example <category> <example> <cli-command>`, where `<cli-command>` is any series of arguments you'd provide to the usual Perseus CLI.

If any of these examples don't work, please [open an issue](https://github.com/arctic-hen7/perseus/issues/choose) and let us know!

The `website/` directory contains the examples you see on the front page of the Perseus website, [here](https://framesurge.sh/perseus/en-US). These should be kept as concise as possible, but it doesn't matter if they're updated on `main` or in a PR for code that hasn't been published yet, since the website gets them from the `stable` branch. That way, those examples will always be for the latest published version of Perseus (even if it's a beta version).

*Note: by default, all examples are assumed to use the `perseus-integration` helper crate, which allows testing them with all integrations. If this is not the case, add a `.integration_locked` file to the root of the example.*

**Warning:** all of the `core` examples use `ErrorViews::unlocalized_default()` for their error views, except for the example that specifically regards error views. This is done solely for convenience and to reduce the burden of maintaining all the examples. In real apps, error views will be provided for you in development for convenience, but you'll have to provide your own in production (unless you explicitly force the development error views to come with you to production, with `::unlocalized_default()`, which is **not** recommended).

Note that the examples all spell out the lifetimes of view functions that take state in full, so, when you see function signatures like `fn my_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a MyStateRx) -> View<G>`, don't despair! This is the full form of these, written out so you can see how everything works, however there is a convenience macro that lets you elide these lifetimes: `#[auto_scope]` (included in `perseus::prelude`). Examples of autoscoped view functions can be found in the `core/basic` example and the `core/capsules` example (on a capsule). You're welcome to use this macro, or not, whichever you prefer.
