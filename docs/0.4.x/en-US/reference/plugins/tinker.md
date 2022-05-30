# The `tinker` Action

There's one [functional action](:reference/plugins/functional) that's quite special in Perseus: the `tinker` action. This action doesn't run as part of any of the usual processes, and it actually has its own command in the CLI: `perseus tinker`. That's because this action allows plugins to modify the code of the Perseus engine. For example, applying [size optimizations](:reference/deploying/size) is a common requirement in Perseus apps, which means modifying `.perseus/Cargo.toml`. This is the perfect job for a plugin, but if it were done by any other action, you'd be modifying the `Cargo.toml` _after_ the code had been compiled, which means the modifications would have no effect until the next run.

The `tinker` action solves this problem by creating its own process that's specifically designed for engine modification and tweaking. Until [#59](https://github.com/arctic-hen7/perseus/issues/59) is resolved, this is how you'd make major modifications to the `.perseus/` engine efficiently.

## `perseus tinker`

The `tinker` subcommand in the CLI has one simple function: to execute the tinkers of all the plugins an app uses. By default, it will delete and re-create the `.perseus/` directory to remove any corruptions (which are common with plugins that arbitrarily modify Perseus' code, as you can probably imagine). You can disable that behavior with the `--no-clean` flag.

If you've ejected, running this command will lead to an error, because running tinkers after you've ejected may delete some of your modifications. Most plugins expect to start with the default engines, and your modifications may cause all sorts of problems. If you're certain your modifications won't interfere with things, you can add the `--force` flag to push on. Note that if you don't provide `--no-clean` as well, the entire `.perseus/` directory will be deleted irrecoverably!
