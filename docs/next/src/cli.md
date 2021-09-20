# CLI

One of the things that makes Perseus so different from most Rust frameworks is that it has its own CLI for development. The reason for this is to make using Perseus as simple as possible, and also because, if you have a look at what's in `.perseus/`, building without the CLI is really hard!

## Commands

### `build`

Builds your app, performing static generation and preparing a Wasm package in `.perseus/dist/`.

### `serve`

Builds your app in the same way as `build`, and then builds the Perseus server (which has dependencies on your code, and so needs to rebuilt on any changes just like the stuff in `.perseus/dist/`), finally serving your app at <http://localhost:8080>. You can change the default host and port this serves on with the `HOST` and `PORT` environment variables.

You can also provide `--no-build` to this command to make it skip building your app to Wasm and performing static generation. In this case, it will just build the serve rand run it (ideal for restarting the server if you've made no changes).

### `clean`

This command is the solution to just about any problem in your app that doesn't make sense, it deletes the `.perseus/` directory entirely, which should remove any corruptions! If this doesn't work, then the problem is in your code (unless you just updated to a new version and now something doesn't work, then it's probably on us, please [open an issue](https://github.com/arctic-hen7/perseus)!).

Note that this command will force Perseus to rebuild `.perseus/` the next time you run `perseus build` or `perseus serve`, which can be annoying in terms of build times. It's almost always sufficient to run this command with the `--dist` flag, which will only delete some content in `.perseus/dist/` that's likely to be problematic.

### `eject`

See the next section for the details of this command.

## Watching

Right now, the Perseus CLI doesn't support watching files for changes and rebuilding, but it soon will. Until then, you can replicate this behavior with a tool like [`entr`](https://github.com/eradman/entr) or the equivalent. Anything that watches file and reruns commands when they change will work for this.

Here's an example of watching files with `entr`:

```
find . -not -path "./.perseus/*" -not -path "./target/*" | entr -s "perseus serve"
```
