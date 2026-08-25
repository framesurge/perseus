# Using Capsules

This guide shows how to create and use capsules in your Perseus app.

## Project Structure

Create a `capsules/` directory alongside your `templates/`:

```
src/
├── main.rs
├── templates/
│   └── index.rs
└── capsules/
    ├── mod.rs
    └── greeting.rs
```

## Defining a Capsule

Use the referential definition pattern with `lazy_static`:

```rust
// src/capsules/greeting.rs
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// Define state
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "GreetingStateRx")]
struct GreetingState {
    message: String,
}

// Define properties (passed by caller)
#[derive(Clone)]
pub struct GreetingProps {
    pub size: String,
}

// Capsule view function - note the extra props parameter
#[auto_scope]
fn greeting_widget(
    state: GreetingStateRx,
    props: GreetingProps,
) -> View {
    view! {
        div(class = format!("greeting {}", props.size)) {
            (state.message.get_clone())
        }
    }
}

// Fallback while loading
fn greeting_fallback(_props: GreetingProps) -> View {
    view! {
        div(class = "greeting-skeleton") {
            "Loading..."
        }
    }
}

// Build state
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> GreetingState {
    GreetingState {
        message: "Hello from capsule!".to_string(),
    }
}

// Create the capsule (use lazy_static for referential access)
lazy_static::lazy_static! {
    pub static ref GREETING: Capsule<PerseusNodeType, GreetingProps> = {
        Capsule::build(
            Template::build("greeting")
                .build_state_fn(get_build_state)
        )
        .fallback(greeting_fallback)
        .view_with_state(greeting_widget)
        .build()
    };
}
```

## Key Differences from Templates

| Aspect | Template | Capsule |
|--------|----------|---------|
| View function | Takes state only | Takes state + props |
| Has fallback | No | Yes (required) |
| Created with | `Template::build()` | `Capsule::build(Template::build(...))` |
| Registered with | `.template()` | `.capsule_ref()` |

## Registering Capsules

Add capsules to your `PerseusApp`:

```rust
// src/main.rs
use perseus::prelude::*;

mod capsules;
mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .capsule_ref(&*crate::capsules::greeting::GREETING)
        .error_views(ErrorViews::unlocalized_development_default())
}
```

## Using Widgets in Templates

Embed widgets using `.widget()`:

```rust
// src/templates/index.rs
use perseus::prelude::*;
use sycamore::prelude::*;
use crate::capsules::greeting::{GREETING, GreetingProps};

fn index_page() -> View {
    view! {
        h1 { "My Page" }

        // Embed the greeting widget
        (GREETING.widget(
            "",  // Widget path (empty = root)
            GreetingProps { size: "large".to_string() }
        ))

        p { "More content below the widget" }
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .view(index_page)
        .build()
}
```

## Widget Paths

The path argument specifies which widget to render:

```rust
// Capsule "products" with build_paths: ["apple", "banana", "orange"]

// Render the apple widget
PRODUCTS.widget("apple", ProductProps::default())

// Render the banana widget
PRODUCTS.widget("banana", ProductProps::default())
```

For capsules without build paths (single widget), use empty string `""`.

## Widgets Without State

For capsules without state:

```rust
#[derive(Clone)]
pub struct ButtonProps {
    pub label: String,
}

fn button_widget(props: ButtonProps) -> View {
    view! {
        button { (props.label) }
    }
}

fn button_fallback(_props: ButtonProps) -> View {
    view! {
        button(disabled = true) { "..." }
    }
}

lazy_static::lazy_static! {
    pub static ref BUTTON: Capsule<PerseusNodeType, ButtonProps> = {
        Capsule::build(Template::build("button"))
            .fallback(button_fallback)
            .view(button_widget)
            .build()
    };
}
```

## Widgets Without Properties

Use unit type `()` for properties:

```rust
fn simple_widget() -> View {
    view! { div { "Simple content" } }
}

lazy_static::lazy_static! {
    pub static ref SIMPLE: Capsule<PerseusNodeType, ()> = {
        Capsule::build(Template::build("simple"))
            .empty_fallback()
            .view(simple_widget)
            .build()
    };
}

// Usage
SIMPLE.widget("", ())
```

## Delayed Widgets

For widgets that should load after the main page:

```rust
// Normal widget (included in initial HTML)
(MY_CAPSULE.widget("path", props))

// Delayed widget (loaded after page renders)
(MY_CAPSULE.delayed_widget("path", props))
```

Use delayed widgets for heavy content that would slow down initial page load.

## Rescheduling

If a build-time page uses a request-time widget, Perseus needs permission to reschedule:

```rust
// Template that uses a request-time capsule
Template::build("my-page")
    .build_state_fn(get_build_state)
    .allow_rescheduling()  // Required!
    .view(page_view)
    .build()
```

Only add `.allow_rescheduling()` when you get the error - don't add it preemptively.

## Nested Widgets

Widgets can contain other widgets:

```rust
fn wrapper_widget(state: WrapperStateRx, _props: ()) -> View {
    view! {
        div(class = "wrapper") {
            h2 { (state.title.get_clone()) }
            // Embed another widget
            (INNER_CAPSULE.widget("", InnerProps::default()))
        }
    }
}
```

**Warning**: Deep nesting requires multiple render passes. Limit to 2-3 levels.

## Capsule with Build Paths

```rust
#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        extra: ().into(),
    }
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> LetterState {
    LetterState {
        letter: info.path.clone(),
    }
}

lazy_static::lazy_static! {
    pub static ref LETTER: Capsule<PerseusNodeType, ()> = {
        Capsule::build(
            Template::build("letter")
                .build_paths_fn(get_build_paths)
                .build_state_fn(get_build_state)
        )
        .empty_fallback()
        .view_with_state(letter_widget)
        .build()
    };
}

// Usage
LETTER.widget("a", ())  // Renders letter A
LETTER.widget("b", ())  // Renders letter B
```

## Tips

1. **Use capsules for reusable UI with state** - Headers, footers, sidebars
2. **Consider caching benefits** - Frequently reused content across pages
3. **Keep fallbacks simple** - Skeletons or loading indicators
4. **Watch nesting depth** - Each level adds a render pass
5. **Test with slow networks** - See fallbacks in action

## Getting Help

Capsules are a novel architecture. If you have issues:
- [GitHub Discussions](https://github.com/framesurge/perseus/discussions)
- [Discord](https://discord.com/invite/GNqWYWNTdp)
