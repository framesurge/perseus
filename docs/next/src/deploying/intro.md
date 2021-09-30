# Deploying

> **WARNING:** although Perseus is technically ready for deployment, the system is not yet recommended for production! See [here](../what-is-perseus.md#how-stable-is-it) for more details.

Perseus is a complex system, but we aim to make deploying it as easy as possible. This section will describe a few different types of Perseus deployments, and how they can be managed.

## Release Mode

The Perseus CLI supports the `--release` flag on the `build`, `serve`, and `export` commands. When you're preparing a production release of your app, be sure to use this flag!

## `perseus deploy`

If you haven't [ejected](../cli/ejecting.md), then you can prepare your app for deployment with a single command: `perseus deploy`. If you can use [static exporting](../exporting.md), then you should run `perseus deploy -e`, otherwise you should just use `perseus deploy`.

This will create a new directory `pkg/` for you (you can change that by specifying `--output`) which will contain everything you need to deploy your app. That directory is entirely self-contained, and can be copied to an appropriate hosting provider for production deployment!

Note that this command will run a number of optimizations in the background, including using the `--release` flag, but it won't try to aggressively minimize your Wasm code size. For tips on how to do that, see [here](./size.md).

### Static Exporting

If you use `perseus deploy -e`, the contents of `pkg/` can be served by any file host that can handle the [slight hiccup](../exporting.md#file-extensions) of file extensions. Locally, you can test this out with [`serve`](https://github.com/vercel/serve), a JavaScript package designed for this purpose.

### Fully-Fledged Server

If you just use `perseus deploy`, the `pkg/` directory will contain a binary called `server` for you to run, which will serve your app on its own. However, it's important to note that this binary is structured to support either the development configuration of running inside `.perseus/` or the production configuration of running inside `pkg/`, and you have to provide the `PERSEUS_STANDALONE` environment variable to tell it to do the latter. This binary can then be run on any server with a writable filesystem. For more details on this, see the next subsection.
