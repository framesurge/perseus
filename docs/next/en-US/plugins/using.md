# Using Plugins

The plugins system is designed to be as easy as possible to use, and you can import plugins into your app like so (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/plugins/src/lib.rs)):

```rust
{{#include ../../../../examples/plugins/src/lib.rs}}
```

In addition to the usual `define_app!` calls, this also uses the `plugins` parameter, passing to it an instance of `perseus::plugins::Plugins`, which manages all the intricacies of the plugins system. If this parameter isn't provided, it'll default to `Plugins::new()`, which creates the configuration for plugins without registering any.

To register a plugin, we use the `.plugin()` function on `Plugins`, which takes two parameters: the plugin's definition (a `perseus::plugins::Plugin`) and any data that should be provided to the plugin. The former should be exported from the plugin's crate, and the latter you'll need to provide based on the plugin's documentation. Note that plugins can accept almost anything as data (specifically, anything that can be expressed as `dyn Any`).
