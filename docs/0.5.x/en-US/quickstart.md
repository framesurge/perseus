# Quickstart

Get your first Perseus app running in just a few minutes!

## Prerequisites

1. **Install Rust**: Make sure you have Rust installed. We recommend using [`rustup`](https://rustup.rs):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install the WebAssembly target**:
   ```sh
   rustup target add wasm32-unknown-unknown
   ```

## Create Your App

1. **Install the Perseus CLI**:
   ```sh
   cargo install perseus-cli
   ```

2. **Create a new project**:
   ```sh
   perseus new my-app
   cd my-app
   ```

3. **Start the development server**:
   ```sh
   perseus serve -w
   ```

   Visit <http://localhost:8080> and you should see a welcome page!

## Understanding the Project Structure

Your new Perseus app has the following structure:

```
my-app/
├── Cargo.toml          # Rust dependencies
├── src/
│   ├── main.rs         # App entry point
│   └── templates/      # Your page templates
│       ├── mod.rs
│       └── index.rs    # Landing page
```

## Your First Page

Let's look at what's in `src/templates/index.rs`:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page() -> View {
    view! {
        h1 { "Welcome to Perseus!" }
        p { "This is your landing page." }
        Link(to = "/about") { "Go to About" }
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .view(index_page)
        .build()
}
```

**Key things to notice:**
- `View` is the return type for all view functions
- The `view!` macro creates HTML-like syntax in Rust
- `Link` is used for client-side navigation (no full page reload)
- `Template::build("index")` creates a template at the root path `/`

## Adding a New Page

Let's create an About page:

1. **Create** `src/templates/about.rs`:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        h1 { "About Us" }
        p { "This is the about page." }
        Link(to = "/") { "Back to Home" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "About | My App" }
    }
}

pub fn get_template() -> Template {
    Template::build("about")
        .view(about_page)
        .head(head)
        .build()
}
```

2. **Register the module** in `src/templates/mod.rs`:
```rust
pub mod about;
pub mod index;
```

3. **Add the template** in `src/main.rs`:
```rust
use perseus::prelude::*;

mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
}
```

Visit <http://localhost:8080/about> to see your new page!

## Adding Dynamic State

Let's add some state to our index page:

```rust
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// Define your state structure
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexStateRx")]
struct IndexState {
    greeting: String,
    count: i32,
}

// Your view receives the reactive state
#[auto_scope]
fn index_page(state: IndexStateRx) -> View {
    view! {
        h1 { (state.greeting.get_clone()) }
        p { "Count: " (state.count.get()) }
        button(on:click = move |_| state.count.set(*state.count.get() + 1)) {
            "Increment"
        }
        Link(to = "/about") { "Go to About" }
    }
}

// Generate state at build time
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexState {
    IndexState {
        greeting: "Hello, Perseus!".to_string(),
        count: 0,
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .build()
}
```

**Key concepts:**
- `#[derive(ReactiveState)]` makes your state reactive
- `#[rx(alias = "...")]` creates a type alias for the reactive version
- `#[auto_scope]` handles lifetime management automatically
- `.get()` reads a reactive value, `.set()` updates it
- `.get_clone()` clones the value (useful for `String`)

## What's Next?

- [Understanding Templates and Pages](/docs/first-app/generating-pages)
- [Error Handling](/docs/first-app/error-handling)
- [Deploying Your App](/docs/first-app/deploying)
- [State Management](/docs/state/intro)

## Build Stages

When you run `perseus serve`, several things happen:

1. **Generate your app**: Compiles the server-side code and builds all pages
2. **Build to Wasm**: Compiles the browser code to WebAssembly
3. **Start server**: Launches the development server

The `-w` flag enables watch mode - your app automatically rebuilds when you change code.

**Tip**: For faster builds during development, see [Improving Compilation Times](/docs/fundamentals/compilation-times).
