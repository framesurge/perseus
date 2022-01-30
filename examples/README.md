# Examples

This folder contains all the examples for Perseus! These are divided into three categories: core examples, which demonstrate core features of Perseus and are used as end-to-end tests; demonstrative examples, which quickly demonstrate how to do certain things with Perseus; and comprehensive examples, which show off whole systems built with Perseus.

There's also a `.base/` folder that contains a template that new examples should be built from. If you'd like to create a new exmaple, you should copy this directory and modify it to your needs, placing your new example in the appropriate category (this will usually be `demos` or `comprehensive`, you should only put it in `core` if you've confirmed this with a maintainer or if you've built the feature it shows off).

Each of the examples here are fully self-contained Perseus apps, though they use relative path dependencies to the bleeding edge versions of the Perseus packages in this repository. They're also designed to be used with the local, bleeding-edge version of the CLI, which can be invoked by running `bonnie dev example <category> <example> <cli-command>`, where `<cli-command>` is any series of arguments you'd provide to the usual Perseus CLI.

If any of these examples don't work, please [open an issue](https://github.com/arctic-hen7/perseus/issues/choose) and let us know!
