# Using State in Views

This guide explains how to use state in your view functions with Sycamore 0.9.2.

## Reactive State Basics

When you derive `ReactiveState`, Perseus creates a reactive version of your struct:

```rust
// Your definition
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CounterStateRx")]
struct CounterState {
    count: i32,
    name: String,
}

// What Perseus creates (conceptually)
struct CounterStateRx {
    count: Signal<i32>,
    name: Signal<String>,
}
```

## Using State in Views

Use `#[auto_scope]` to simplify view function signatures:

```rust
#[auto_scope]
fn my_view(state: CounterStateRx) -> View {
    view! {
        h1 { "Hello, " (state.name.get_clone()) "!" }
        p { "Count: " (state.count.get()) }

        button(on:click = move |_| {
            state.count.set(*state.count.get() + 1);
        }) {
            "Increment"
        }
    }
}
```

### Reading Values

| Method | Use For | Example |
|--------|---------|---------|
| `.get()` | `Copy` types (`i32`, `bool`, etc.) | `state.count.get()` |
| `.get_clone()` | `Clone` types (`String`, etc.) | `state.name.get_clone()` |

### Setting Values

```rust
// Set to a new value
state.count.set(42);

// Update based on current value
state.count.set(*state.count.get() + 1);

// For strings
state.name.set("New Name".to_string());
```

## Template Registration

Use `.view_with_state()` for views that receive state:

```rust
pub fn get_template() -> Template {
    Template::build("counter")
        .build_state_fn(get_build_state)
        .view_with_state(my_view)  // Not .view()
        .build()
}
```

## Unreactive State

For static content that doesn't need reactivity:

```rust
#[derive(Serialize, Deserialize, UnreactiveState, Clone)]
struct PageInfo {
    title: String,
    description: String,
}

fn page_view(state: PageInfo) -> View {
    view! {
        h1 { (state.title) }  // Direct access, no .get()
        p { (state.description) }
    }
}

pub fn get_template() -> Template {
    Template::build("info")
        .build_state_fn(get_build_state)
        .view_with_unreactive_state(page_view)
        .build()
}
```

### When to Use Unreactive State

- Static content that won't change client-side
- Simpler API (no `.get()` / `.set()`)
- Excluded from Hot State Reload (HSR) by default

## Nested State

For complex state structures, use `#[rx(nested)]`:

```rust
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "UserStateRx")]
struct UserState {
    name: String,
    age: i32,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "PageStateRx")]
struct PageState {
    #[rx(nested)]
    user: UserState,
    page_title: String,
}

#[auto_scope]
fn page_view(state: PageStateRx) -> View {
    view! {
        h1 { (state.page_title.get_clone()) }
        p { "Name: " (state.user.name.get_clone()) }
        p { "Age: " (state.user.age.get()) }
    }
}
```

## Page State Store (PSS)

Perseus caches page states for seamless navigation:

```
User visits /page1 → State stored in PSS
User visits /page2 → State stored in PSS
User returns to /page1 → State restored from PSS (with user's changes!)
```

### Implications

- Form inputs are preserved when returning to a page
- User interactions persist
- No network request needed for cached pages

### Configuring PSS Size

```rust
PerseusApp::new()
    .pss_max_size(100)  // Cache up to 100 pages (default: 25)
```

## Hot State Reload (HSR)

During development, Perseus can preserve state across rebuilds.

To exclude a state type from HSR (so you see fresh content):

```rust
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "BlogPostRx")]
#[rx(hsr_ignore)]  // Add this
struct BlogPost {
    content: String,
}
```

This is useful when editing content you want to preview immediately.

## Complete Example

```rust
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "FormStateRx")]
struct FormState {
    name: String,
    email: String,
    message: String,
    submitted: bool,
}

#[auto_scope]
fn contact_form(state: FormStateRx) -> View {
    let handle_submit = move |_| {
        // In a real app, you'd send this to a server
        web_sys::console::log_1(
            &format!(
                "Submitted: {} - {} - {}",
                state.name.get_clone(),
                state.email.get_clone(),
                state.message.get_clone()
            ).into()
        );
        state.submitted.set(true);
    };

    view! {
        (if *state.submitted.get() {
            view! {
                div(class = "success") {
                    h2 { "Thank you!" }
                    p { "We'll be in touch soon." }
                }
            }
        } else {
            view! {
                form(on:submit = handle_submit) {
                    label {
                        "Name: "
                        input(
                            type = "text",
                            bind:value = state.name
                        )
                    }
                    label {
                        "Email: "
                        input(
                            type = "email",
                            bind:value = state.email
                        )
                    }
                    label {
                        "Message: "
                        textarea(bind:value = state.message)
                    }
                    button(type = "submit") { "Send" }
                }
            }
        })
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> FormState {
    FormState {
        name: String::new(),
        email: String::new(),
        message: String::new(),
        submitted: false,
    }
}

pub fn get_template() -> Template {
    Template::build("contact")
        .build_state_fn(get_build_state)
        .view_with_state(contact_form)
        .build()
}
```

## Summary

| Concept | Usage |
|---------|-------|
| Reactive state | `#[derive(ReactiveState)]` with `#[rx(alias = "...")]` |
| View function | `#[auto_scope] fn view(state: StateRx) -> View` |
| Read copy types | `state.field.get()` |
| Read clone types | `state.field.get_clone()` |
| Update state | `state.field.set(value)` |
| Nested state | `#[rx(nested)]` on fields |
| Unreactive state | `#[derive(UnreactiveState)]` |

## Next Steps

- [Global State](/docs/state/global) - Shared state across all pages
- [Suspended State](/docs/state/suspense) - Client-side state loading
- [Freezing and Thawing](/docs/state/freezing-thawing) - State persistence
