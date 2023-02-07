# Working with JS

When you're working with Perseus, you usually won't need to use any JavaScript whatsoever, since Perseus handles the only JS you need (the stuff that calls your Wasm bundle) automatically. However, sometimes you'll need to work with an external module, or you'll need to work with some other JS code that some other team has built, etc. In these cases, you'll need to work more directly with JS, which can be done through `wasm-bindgen`, which underlies Perseus. For more details on this process, take a look at [their documentation](https://rustwasm.github.io/book/reference/js-ffi.html) on this.

A simple example of working with JS might be this, on the landing page of a hypothetical app:

```rust
{{#include ../../../examples/core/js_interop/src/templates/index.rs}}
```

The comments mostly explain this, but the main thing to understand from the perspective of Perseus as a framework is that `#[wasm_bindgen(module = "..")]` attribute, which can be basically considered magic, because you specify a file, and then that just ends up being hosted by Perseus, and everything *just works*.

Somewhat humorously, Perseus actually does next to nothing to make this happen, because it calls `wasm-bindgen` under the hood (which is also a CLI as well as a library), which automatically copies any JS files you references into a file Perseus puts in `dist/`. That's then served automatically by Perseus' server systems and appropriately relocated by the export systems. These files are called JS 'snippets', and are available at `/.perseus/snippets` (which you don't have to import or anything, it literally just works because of how Perseus hosts other files, namely `bundle.js`). Again, 95% of this is done by `wasm-bindgen`, not Perseus, and it's best to take a look at [their docs](https://rustwasm.github.io/book/reference/js-ffi.html) on the subject.
