# Using Plugins

The plugins system is designed to be as easy as possible to use, and you can import plugins into your app like so (taken from [here](https://github.com/framesurge/perseus/blob/main/examples/core/plugins/src/lib.rs)):

```rust
{{#include ../../../../examples/core/plugins/src/lib.rs}}
```

In addition to the usual `PerseusApp` setup, this also uses the `.plugins()` function, passing to it an instance of `perseus::plugins::Plugins`, which manages all the intricacies of the plugins system. If this parameter isn't provided, it'll default to `Plugins::new()`, which creates the configuration for plugins without registering any.

To register a plugin, we use the `.plugin()`/`.plugin_with_client_privilege()` function on `Plugins`, which takes two parameters: the plugin's definition (a `perseus::plugins::Plugin`) and any data that should be provided to the plugin. The former should be exported from the plugin's crate, and the latter you'll need to provide based on the plugin's documentation. Note that plugins can accept almost anything as data (specifically, anything that can be expressed as `dyn Any`).

Plugins in Perseus are fantastic, but they're also a great way to increase your Wasm bundle size, which will make your website slower to load when users first visit it. To mitigate this, Perseus lets plugin authors define where their plugins should run: in the browser (`PluginEnv::Client`), on the server-side (`PluginEnv::Server`), or on both (`PluginEnv::Both`). Plugins that only run on the server-side should be registered with `.plugin()`, and they will not be included in your final Wasm binary, which keeps your website nimble. If a plugin does need to run on the client though, it can be registered with `.plugin_with_client_privilege()` instead, which is named separately for conditional compilation reasons as well as to create a clear separation. But don't worry, if you accidentally register a plugin with the wrong function, your app won'y build, and Perseus will tell you that you've used the wrong function.
