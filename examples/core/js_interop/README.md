# JavaScript Interoperability Example

Unfortunately, not everything can be done with Wasm all the time. If you need to fall back to JavaScript, this example shows you how to do so, by showcasing Perseus' use of `wasm-bindgen` to support arbitrary JS snippets that can be imported as external code.

Notably, JS snippets are automatically hosted at `/.perseus/snippets` on your site, but you shouldn't ever need to worry about this, since `wasm-bindgen` will automatically import them. You can find more details about JS and Wasm working together [here](https://rustwasm.github.io/docs/wasm-bindgen/reference/js-snippets.html).
