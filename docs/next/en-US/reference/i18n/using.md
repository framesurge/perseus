# Using Translations

Perseus tries to make it as easy as possible to use translations in your app by exposing the low-level Fluent primitives necessary to work with very complex translations, as well as a `t!` macro that does the basics. Note that, to use i18n, you'll need to enable a translator, the usual one is for [Fluent](https://projectfluent.org). Change your Perseus import in your `Cargo.toml` to look like this:

```toml
perseus = { version = "<version of Perseus that you're using>", features = [ "translator-fluent" ] }
```

If you don't do this, your app won't build.

All translations in Perseus are done with an instance of `Translator`, which is provided through Sycamore's [context system](https://sycamore-rs.netlify.app/docs/v0.6/advanced/contexts). Here's an example taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/core/i18n/src/templates/index.rs):

```rust
{{#include ../../../../examples/core/i18n/src/templates/index.rs}}
```

In that example, we've imported `perseus::t`, and we use it to translate the `hello` ID, which takes an argument for the username. Notice that we don't provide a locale, Perseus handles all this in the background for us.

## Getting the `Translator`

That said, there are some cases in which you'll want access to the underlying `Translator` so you can do more complex things. You can get it like so:

```rust
perseus::get_render_ctx!().translator;
```

To see all the methods available on `Translator`, see [the API docs](https://docs.rs/perseus).
