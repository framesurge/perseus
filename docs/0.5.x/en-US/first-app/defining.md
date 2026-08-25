# Defining a Perseus App

Once you've got Perseus installed, it's time to create your app's entry point. This guide explains how Perseus apps are structured.

## The Two Sides of Perseus

Perseus has two parts:
- **Engine-side (server)**: Runs on your server, handles rendering and state generation
- **Client-side (browser)**: Runs as WebAssembly in the user's browser

While you *could* write separate code for each side, Perseus provides a convenient `#[perseus::main]` macro that handles this automatically.

## Basic App Structure

Here's the simplest Perseus app:

```rust
use perseus::prelude::*;

mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
}
```

Let's break this down:

### The `#[perseus::main]` Macro

```rust
#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    // ...
}
```

This macro:
1. Creates the engine-side `main()` function that handles server operations
2. Creates the client-side entry point for WebAssembly
3. Uses the specified server (here `perseus_axum`) to serve your app

**Available servers:**
- `perseus_axum` - Uses [Axum](https://github.com/tokio-rs/axum) (recommended)
- `perseus_warp` - Uses [Warp](https://github.com/seanmonstar/warp)
- `perseus_actix_web` - Uses [Actix Web](https://github.com/actix/actix-web)

### The `PerseusApp` Builder

```rust
PerseusApp::new()
    .template(crate::templates::index::get_template())
    .error_views(ErrorViews::unlocalized_development_default())
```

`PerseusApp` is your app's configuration. Common methods include:

| Method | Description |
|--------|-------------|
| `.template(t)` | Adds a template that generates pages |
| `.error_views(e)` | Sets error handling views |
| `.index_view(f)` | Customizes the HTML shell |
| `.global_state_creator(g)` | Sets up global state |
| `.locales_and_translations_manager(...)` | Enables internationalization |

## Project Structure

A typical Perseus project looks like this:

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs              # App entry point
│   ├── error_views.rs       # Error handling (optional)
│   └── templates/
│       ├── mod.rs           # Template exports
│       ├── index.rs         # Landing page template
│       └── about.rs         # About page template
```

### The Templates Module

Create `src/templates/mod.rs`:

```rust
pub mod about;
pub mod index;
```

Each template file exports a `get_template()` function that you register in `main.rs`.

## A Complete Example

Here's a full `src/main.rs` with multiple templates:

```rust
use perseus::prelude::*;

mod error_views;
mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        // Register templates
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        // Set up error handling
        .error_views(crate::error_views::get_error_views())
}
```

## Custom Index View

By default, Perseus uses a minimal HTML shell. You can customize it:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
        .index_view(|| {
            view! {
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        link(rel = "stylesheet", href = "/.perseus/static/styles.css")
                    }
                    body {
                        PerseusRoot()  // Your app renders here
                    }
                }
            }
        })
}
```

The `PerseusRoot()` component is where your pages will be rendered.

## Development vs Production

During development, you can use:

```rust
.error_views(ErrorViews::unlocalized_development_default())
```

This provides sensible error pages. However, for production, you should create custom error views (see [Error Handling](/docs/first-app/error-handling)).

## Next Steps

Now that your app is set up, let's [generate some pages](/docs/first-app/generating-pages)!
