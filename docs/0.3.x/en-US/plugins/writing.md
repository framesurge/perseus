# Writing Plugins

Writing Perseus plugins is a relatively seamless process once you get the hang of the structure, and this section will guide you through the process. If you just want to use plugins, you can skip this section.

## Structure

A plugin will usually occupy its own crate, but it may also be part of a larger app that just uses plugins for convenience and to avoid [ejection](:ejecting). The only thing you'll need in a plugin is the `perseus` crate, though you'll probably want to bring other libraries in (like `sycamore` if you're adding templates or error pages).

## Defining a Plugin

To define a plugin, you'll call `perseus::plugins::Plugin::new()`, which takes four parameters:

-   The name of the plugin as a `&str`, which should be the name of the crate the plugin is in (or the name of a larger app with some extension) (**all plugins MUST have unique names**)
-   A [functional actions](:plugins/functional) registrar function, which is given some functional actions and then extends them
-   A [control actions](:plugins/control) registrar, which is given some control actions and then extends them
-   Whether or not the plugin should only run at `tinker`-time (see below)

Here's an example of a very simple plugin that adds a static alias for the project's `Cargo.toml`, creates an about page, and prints the working directory at [tinker](:plugins/tinker)-time (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/plugins/src/plugin.rs)):

```rust
{{#include ../../../../examples/plugins/src/plugin.rs}}
```

One particularly important thing to note here is the absence of any control actions in this plugin. Because you still have to provide a registrar, this function is using the `empty_control_actions_registrar` convenience function, which does exactly what its name implies.

Another notable thing is the presence of `GenericNode` as a type parameter, because some plugin actions take this, so you'll need to pass it through. We also tell Perseus what type of data out plugin will take in the second type parameter, which enables type checking in the `.plugin()` call when the user imports the plugin.

The rest of the code is the functional actions registrar, which just registers the plugin on the `functional_actions.settings_actions.add_static_aliases`, `functional_actions.settings_actions.add_templates`, and `functional_actions.tinker` actions. The functions provided to the `.register_plugin()` function are _runners_, which will be executed at the appropriate time by the Perseus engine. Runners take two parameters, _action data_, and _plugin data_. Action data are data provided to every runner for the given action (e.g. an action that runs after a failed build will be told what the error was). You should refer to [the API docs](https://docs.rs/perseus) to learn more about these for different actions. The second parameter is plugin data, covered below.

## Plugin Data

Quite often, plugins should accept user configuration, and this is supported through the second runner parameter, which will be given any data that the user defined for your plugin. You can define the type of this with the second type parameter to `Plugin`.

However, because Perseus is juggling all the data for all the plugins the user has installed, across all their different runners, it can't store the type of the data that the user gives (but don't worry, whatever they provide will be type-checked). This means that your runner ends up being given what Rust considers to be _something_. Basically, **we know that it's your plugin data, but Rust doesn't**. Specifically, you'll be given `&dyn Any`, which means you'll need to _downcast_ this to a concrete type (the type of your plugin data). As in the above example, we can do this with `plugin_data.downcast_ref::<YourPluginDataTypeHere>()`, which will return an `Option<T>`. **This will always be `Some`**, which is why it's perfectly safe to label the `None` branch as `unreachable!()`. If this ever does result in `None`, then either you've tried to downcast to something that's not your plugin's data type, or there's a critical bug in Perseus' plugins system, which you should [report to us](https://github.com/arctic-hen7/perseus/issues/new/choose).

## Caveats

Right now, there are few things that you can't do with Perseus plugins, which can be quite weird.

-   You can't extend the engine's server (due to a limitation of Actix Web types), you'll need to manually run a `tinker` on it (add your code into the file by writing it in using [the `tinker` action](:plugins/tinker))
-   You can't set the [mutable store](:stores) from a plugin due to a traits issue, so you'll need to provide something for the user to provide to the `mutable_store` parameter of the `define_app!` macro
-   Similarly, you can't set the translations manager from a plugin

## Tinker-Only Plugins

There are some cases of plugin development in which a plugin only uses [the `tinker` action](:plugins/tinker), and therefore it should only be included when the user is running `perseus tinker`. The main reason you'd want to do this is to prevent your plugin from becoming part of the client-side Wasm bundle, which will be served to browsers. For example, a size optimizations plugin only needs to run at tinker-time, and, if it were allowed to leak into the client-side bundle, it would actually increase the bundle size because it draws in all its dependencies!

You can make your plugin tinker-only by setting the fourth argument to `Plugin::new()` to `true`.

<details>
<summary>I want my plugin to run on the server, but not the client.</summary>

You should make it a tinker-only plugin. As a technicality, tinker-only plugins will actually run on the server and in the build process in addition to the `tinker` process. They just won't run on the client. Be warned though: a future release may well change this.

</details>
