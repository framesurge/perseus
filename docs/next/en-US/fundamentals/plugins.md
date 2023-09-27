# Plugins

One of the most powerful features of Perseus is its extensibility, which comes in a number of forms. The first is the openness of the API that runs Perseus, primarily through the [`Turbine`](=turbine/struct.Turbine@perseus) type, which can be used to manually generate state, prerender pages, and even build your app. This can be used to create custom engines by ignoring the `#[perseus::main]` macro, and to control your entire app from start to finish. You can even replace Perseus' default instantiation code on the client-side.

However, 99.99% of the time, you won't need to do any of this, because your needs will be met far more effectively by either a [custom server](:fundamentals/serving-exporting), or a plugin.

Plugins in Perseus are library crates, usually published on crates.io, that can be used as dependencies in your app that have access to various *plugin opportunities*, which are basically points in your app where Perseus allows third-party code to do certain things.

Plugins fall into two types: *functional* and *control*. Functional plugins are very simple: they're given some data at a certain time, they do some stuff, and then they return some data. For a single plugin opportunity, there can be as many functional plugins registered as you like. For example, a plugin can define extra static aliases, and, of course, the results of many plugins doing this can all be collated together. Control plugins work differently: for a single control plugin opportunity, only one plugin can act, for example redefining the index view (since you can't necessarily combine two completely different index views from two completely different plugins).

Currently, there aren't a huge number of plugin actions in Perseus, but this will change in future, and the number of plugin opportunities can be expected to grow over the coming releases. Writing plugins is somewhat complex, and is best explained through the [plugin API documentation](=plugins@perseus), and the [plugin example](https://github.com/framesurge/perseus/tree/main/examples/core/plugins) in the Perseus repository. If you need any further help writing your plugin, feel free to [open a GitHub discussion](https://github.com/framesurge/perseus/discussions/new/choose) or [ask on our Discord](https://discord.com/invite/GNqWYWNTdp), and our community will be happy to help!

## Tinker plugins

One type of plugin that is particularly special is the *tinker plugin*. These plugins have free rein to do whatever they want when a special command, `perseus tinker`, is run. An example of registering a tinker plugin can be found [here](https://github.com/framesurge/perseus/tree/main/examples/core/plugins). These may be used to run special build processes, or to even modify user code in arbitrary ways (for example to set a custom allocator), since they run as a separate stage to the build process. These can be considered the closest equivalent to normal Rust build scripts in a Perseus app. (Although you can use normal build scripts if you like, those will work too.) Since the removal of the legacy `.perseus/` directory, tinker plugins have far less utility today than they once did.

## The plugins registry

On this website, a registry of all known plugins is maintained [here](plugins), which currently has a very small number of plugins, because the ecosystem for all this is still very young (plugins were only introduced in v0.3.x). Plugins that are endorsed by the Perseus developers (which implies a code audit, but by no means a guarantee of security, and Perseus takes no responsibility for rogue plugins whatsoever, as they are third-party code) will appear with a tick next to them. You can add your plugin to the registry by following the instructions in [our issue-reporting system](https://framesurge.sh/perseus/tribble/workflow/perseus/), which will guide you through the process.

## Security

Plugins are third-party code, and, because they can do basically anything they want, they can pose a serious risk to the security of your system. For example, a plugin running at build-time could say it's parsing some [Less](https://lesscss.org), but actually be downloading ransomware onto your computer. Any reports of rogue plugins should be reported confidentially [to the Perseus maintainer](mailto:arctic.hen@pm.me), and they will be dealt with expeditiously from there. Of course, we cannot take down rogue third-party plugins, but we can report them to code hosts like GitHub, and have them removed from our own plugin registry.

Once again, the Perseus project cannot be held responsible in any way for rogue plugins, including those we list on our registry, as no code audits take place before listing. Plugins that have a tick next to them have undergone an audit *in the past*, and may have since been taken over. Always make sure you trust the plugins you install! (You should do this for any library you install on your system.)
