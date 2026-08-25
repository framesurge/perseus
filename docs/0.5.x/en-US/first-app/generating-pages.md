# Generating Pages

Perseus uses **templates** to generate **pages**. This is a core concept that makes Perseus powerful and flexible.

## Templates and Pages

Think of it this way:
- A **template** is like a stencil with holes
- **State** is the data that fills those holes
- A **page** is what you get when you combine them

**Template + State = Page**

For example, a blog post template might have:
- A hole for the title
- A hole for the content
- A hole for the author

Each blog post fills these holes with different data, creating different pages from the same template.

## Your First Template

Let's create a simple template without state:

```rust
// src/templates/about.rs
use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        h1 { "About Us" }
        p { "Welcome to our website!" }
        Link(to = "/") { "Go Home" }
    }
}

pub fn get_template() -> Template {
    Template::build("about")
        .view(about_page)
        .build()
}
```

**Key points:**
- `fn about_page() -> View` - View functions return `View`
- `view! { ... }` - Creates HTML-like elements
- `Link(to = "/")` - Client-side navigation component
- `Template::build("about")` - Creates a template at `/about`

## Adding a Head

Every page needs metadata like a title. Use the `head` function:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        h1 { "About Us" }
        p { "Welcome to our website!" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "About | My Website" }
        meta(name = "description", content = "Learn about our company")
    }
}

pub fn get_template() -> Template {
    Template::build("about")
        .view(about_page)
        .head(head)
        .build()
}
```

The `#[engine_only_fn]` macro marks this function as server-side only - it won't be included in your browser bundle.

## Adding State

Most pages need dynamic data. Here's how to add state:

### Step 1: Define Your State

```rust
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "GreetingStateRx")]
struct GreetingState {
    name: String,
    message: String,
}
```

**Understanding the derives:**
- `Serialize, Deserialize` - State is sent over the network as JSON
- `ReactiveState` - Makes fields reactive (they update the UI automatically)
- `Clone` - Required by Perseus internals
- `#[rx(alias = "...")]` - Creates a type alias for the reactive version

### Step 2: Create Your View

```rust
#[auto_scope]
fn greeting_page(state: GreetingStateRx) -> View {
    view! {
        h1 { "Hello, " (state.name.get_clone()) "!" }
        p { (state.message.get_clone()) }
    }
}
```

**Key points:**
- `#[auto_scope]` - Handles complex lifetime requirements
- `state: GreetingStateRx` - Receives the *reactive* version of state
- `.get_clone()` - Gets a clone of the value (use for `String`)
- `.get()` - Gets a reference to the value (use for `Copy` types like `i32`)

### Step 3: Generate State at Build Time

```rust
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> GreetingState {
    GreetingState {
        name: "World".to_string(),
        message: "Welcome to Perseus!".to_string(),
    }
}
```

This function runs at build time and can:
- Read files
- Query databases
- Call APIs
- Do anything async

### Step 4: Wire It Together

```rust
pub fn get_template() -> Template {
    Template::build("greeting")
        .build_state_fn(get_build_state)
        .view_with_state(greeting_page)
        .build()
}
```

Note: Use `.view_with_state()` when your view receives state.

## Complete Example

Here's a complete template with state:

```rust
// src/templates/index.rs
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexStateRx")]
struct IndexState {
    greeting: String,
    count: i32,
}

#[auto_scope]
fn index_page(state: IndexStateRx) -> View {
    view! {
        h1 { (state.greeting.get_clone()) }

        // Interactive counter
        p { "Count: " (state.count.get()) }
        button(on:click = move |_| {
            state.count.set(*state.count.get() + 1);
        }) {
            "Increment"
        }

        // Navigation
        nav {
            Link(to = "/about") { "About" }
        }
    }
}

#[engine_only_fn]
fn head(state: IndexState) -> View {
    view! {
        title { (format!("{} | My App", state.greeting)) }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexState {
    IndexState {
        greeting: "Hello, World!".to_string(),
        count: 0,
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}
```

## Reactive State in Action

The `ReactiveState` derive creates reactive signals for each field:

```rust
// Original struct
struct MyState {
    count: i32,
    name: String,
}

// What ReactiveState creates (simplified)
struct MyStateRx {
    count: Signal<i32>,
    name: Signal<String>,
}
```

This means:
- When you call `.set()`, the UI updates automatically
- Changes persist across page navigations (stored in Page State Store)
- No need to manually manage state updates

## Error Handling

State generation functions can return `Result`:

```rust
use perseus::prelude::*;

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>
) -> Result<MyState, BlamedError<std::io::Error>> {
    let data = std::fs::read_to_string("data.json")
        .map_err(|e| BlamedError::server(None, e))?;

    Ok(serde_json::from_str(&data).unwrap())
}
```

See [Build-Time State](/docs/state/build) for more on error handling.

## Template Routing

The string in `Template::build()` determines the URL:

| Template Name | URL Path |
|--------------|----------|
| `"index"` | `/` (special case) |
| `"about"` | `/about` |
| `"blog/post"` | `/blog/post` |

## Next Steps

- [Error Handling](/docs/first-app/error-handling) - Handle errors gracefully
- [Build-Time State](/docs/state/build) - Generate state at build time
- [Request-Time State](/docs/state/request) - Generate state per request
- [Incremental Generation](/docs/state/incremental) - Generate pages on demand
