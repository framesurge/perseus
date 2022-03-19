# CLI

One of the things that makes Perseus so different from most Rust frameworks is that it has its own CLI for development. The reason for this is to make using Perseus as simple as possible, and also because, if you have a look at what's in `.perseus/`, building without the CLI is really hard!

## Commands

### `build`

Builds your app, performing static generation and preparing a Wasm package in `.perseus/dist/`.

### `serve`

Builds your app in the same way as `build`, and then builds the Perseus server (which has dependencies on your code, and so needs to rebuilt on any changes just like the stuff in `.perseus/dist/`), finally serving your app at <http://localhost:8080>. You can change the default host and port this serves on with the `HOST` and `PORT` environment variables.

You can also provide `--no-build` to this command to make it skip building your app to Wasm and performing static generation. In this case, it will just build the serve rand run it (ideal for restarting the server if you've made no changes).

### `test`

Exactly the same as `serve`, but runs your app in testing mode, which you can read more about [here](:testing/intro).

### `export`

Builds and exports your app to a series of purely static files at `.perseus/dist/exported/`. This will only work if your app doesn't use any strategies that can't be run at build time, but if that's the case, then you can easily use Perseus without a server after running this command! You can read more about static exporting [here](:exporting).

### `deploy`

Builds your app for production and places it in `pkg/`. You can then upload that folder to a server of your choosing to deploy your app live! You can (and really should) read more about deployment and the potential problems you may encounter [here](:deploying/intro).

### `clean`

This command is the solution to just about any problem in your app that doesn't make sense, it deletes the `.perseus/` directory entirely, which should remove any corruptions! If this doesn't work, then the problem is in your code (unless you just updated to a new version and now something doesn't work, then it's probably on us, please [open an issue](https://github.com/arctic-hen7/perseus)!).

Note that this command will force Perseus to rebuild `.perseus/` the next time you run `perseus build` or `perseus serve`, which can be annoying in terms of build times. It's almost always sufficient to run this command with the `--dist` flag, which will only delete some content in `.perseus/dist/` that's likely to be problematic.

### `eject`

See the next section for the details of this command.

## Watching

The Perseus CLI supports watching your local directory for changes when running `perseus serve` or `perseus export` through the `-w/--watch` flag. Adding this will make the CLI spawn another version of itself responsible for running the actual builds, and the original process acts as a supervisor. This approach was chosen due to the complexity of the CLI's multithreaded build system, which makes terminating unfinished builds *extremely* difficult.

Notably, the CLI spawns another version of itself as a process group (or `JobObject` on Windows) using the [`command-group`](https://github.com/watchexec/command-group) crate, which allows terminations signals to go to all builder child processes. However, this means that the CLI needs to manually handle termination signals to it to terminate the processes in thr group. This means that, if the CLI terminates improperly (e.g. if you `kill` it), you will very likely end up with build jobs running in the background. Those shouldn't be too problematic, and you probably won't even notice them, but a server process could also be orphaned, which would leave a port occupied. If this happens, use `ps aux | grep perseus` to find the process ID, and then `kill` it by that (e.g. `kill 60850`) on Linux. If possible though, avoiding force-terminating the Perseus CLI.

Right now, the CLI's watching systems will ignore `.perseus/`, `target/`, and `.git/`. If you have any other directories that you'd like to ignore, you should use an alternative watching system, like [`entr`](https://github.com/eradman/entr). However, we're willing to add support for this if it's a widely-requested feature, so please feel free to [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose) if this affects you!

Here's an example of watching files with `entr` on Linux:

```
find . -not -path "./.perseus/*" -not -path "./target/*" | entr -rs "perseus serve"
```
