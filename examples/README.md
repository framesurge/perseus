# Examples

This folder contains examples for Perseus, which are used to test the project and are excellent learning resources! If any of these don't work, please [open an issue](https://github.com/arctic-hen7/perseus/issues/choose) to let us know!

These examples are all fully self-contained, and do not serve as examples in the traditional Cargo way, they are each indepedent crates to enable the use of build tools such as `wasm-pack`.

## Workspaces??

A Perseus setup is composed of an app and a server, which would normally be in a workspace project. However, examples with their own `Cargo.toml` manifests aren't detected by RLS, and so we need to make them part of the super-workspace at the root. The problem with that is that then we'd have nested workspaces, which are currently impossible. The solution used is to have each example be atomic (i.e. app OR server), but they're still listed together under the same parent directory. If you want to clone one of these to run locally without the rest of the repo, you'll need to get the appropriate directory with both an app and a server, and then add a new `Cargo.toml` at the root of that with the following contents:

```toml
[workspace]
members = [
	"app",
	"server"
]
```

All (non-CLI) examples should have both an `app` and a `server` directory.

-   Showcase -- an app that demonstrates all the different features of Perseus, including SSR, SSG, and ISR (this example is actively used for testing)
-   Basic -- a simple app that uses the Perseus CLI (symlinks to CLI example)
-   CLI -- same as basic, but includes the CLI subcrates (actively used for testing/development)
