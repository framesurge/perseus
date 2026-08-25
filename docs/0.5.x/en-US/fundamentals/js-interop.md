# Working with JavaScript

While Perseus apps are written entirely in Rust, you may occasionally need to interact with JavaScript libraries, browser APIs, or existing JS code. This is done through `wasm-bindgen`.

## Basic JS Interop

Use `wasm-bindgen` to call JavaScript functions from Rust:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

// Import a JS function from a file
#[wasm_bindgen(module = "/src/js/helpers.js")]
extern "C" {
    fn showNotification(message: &str);
}

fn notification_button() -> View {
    view! {
        button(on:click = move |_| {
            showNotification("Hello from Rust!");
        }) {
            "Show Notification"
        }
    }
}
```

The JavaScript file (`src/js/helpers.js`):

```javascript
export function showNotification(message) {
    if (Notification.permission === "granted") {
        new Notification(message);
    } else {
        alert(message);
    }
}
```

## How It Works

When you use `#[wasm_bindgen(module = "..")]`:

1. `wasm-bindgen` copies the JS file to `dist/`
2. Perseus serves it at `/.perseus/snippets/`
3. The import is automatically resolved at runtime

You don't need to manually configure anything—it just works.

## Browser APIs via web-sys

For standard browser APIs, use the `web-sys` crate:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;

fn current_url() -> View {
    let url = web_sys::window()
        .unwrap()
        .location()
        .href()
        .unwrap_or_default();

    view! {
        p { "Current URL: " (url) }
    }
}
```

Add `web-sys` to your `Cargo.toml` with the features you need:

```toml
[dependencies]
web-sys = { version = "0.3", features = ["Window", "Location", "Document"] }
```

## Common Patterns

### Accessing LocalStorage

```rust
use wasm_bindgen::JsCast;

fn save_to_storage(key: &str, value: &str) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item(key, value);
    }
}

fn load_from_storage(key: &str) -> Option<String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|s| s.get_item(key).ok())
        .flatten()
}
```

### Calling External Libraries

```rust
#[wasm_bindgen]
extern "C" {
    // Global function
    #[wasm_bindgen(js_name = "console.log")]
    fn log(s: &str);

    // From a CDN-loaded library
    #[wasm_bindgen(js_namespace = ["hljs"])]
    fn highlightAll();
}

fn code_block() -> View {
    // Call after render
    #[cfg(target_arch = "wasm32")]
    highlightAll();

    view! {
        pre {
            code(class = "language-rust") {
                "fn main() { }"
            }
        }
    }
}
```

### Dynamic Imports

For libraries loaded via CDN:

```rust
#[wasm_bindgen]
extern "C" {
    type Chart;

    #[wasm_bindgen(constructor, js_namespace = ["Chart"])]
    fn new(ctx: &JsValue, config: &JsValue) -> Chart;
}
```

## Platform-Specific Code

Guard browser-only code with cfg attributes:

```rust
fn platform_aware() -> View {
    #[cfg(target_arch = "wasm32")]
    {
        // Browser-only code
        web_sys::console::log_1(&"Running in browser".into());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Server-only code
        println!("Running on server");
    }

    view! {
        p { "Hello from either platform!" }
    }
}
```

## Calling Rust from JavaScript

Export Rust functions to JS:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Then in JS: import { greet } from './my_app';
// greet("World") // Returns "Hello, World!"
```

## Tips

1. **Minimize JS** - Use Rust/Wasm for logic, JS only for browser APIs
2. **Use web-sys** - Standard APIs are well-typed
3. **Guard platform code** - Use `#[cfg(target_arch = "wasm32")]`
4. **Error handling** - JS calls can panic; use try/catch patterns
5. **Keep snippets small** - Large JS files increase bundle size

## Further Reading

- [wasm-bindgen Documentation](https://rustwasm.github.io/docs/wasm-bindgen/)
- [web-sys API Reference](https://docs.rs/web-sys)
- [JS FFI Guide](https://rustwasm.github.io/book/reference/js-ffi.html)

## Related

- [Debugging](/docs/fundamentals/debugging)
- [Static Content](/docs/fundamentals/static-content)
