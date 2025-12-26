# Migrating from 0.4.x to 0.5.x

This guide helps you upgrade your Perseus app from 0.4.x to 0.5.x. The main changes involve Sycamore 0.9.2's simplified reactive model.

## Quick Summary

| 0.4.x | 0.5.x |
|-------|-------|
| `fn view<G: Html>(cx: Scope) -> View<G>` | `fn view() -> View` |
| `view! { cx, ... }` | `view! { ... }` |
| `a(href = "/about")` | `Link(to = "/about")` |
| `cx.create_signal(...)` | `create_signal(...)` |
| Sycamore 0.8.x | Sycamore 0.9.2 |

## Step-by-Step Migration

### 1. Update Dependencies

```toml
# Cargo.toml
[dependencies]
perseus = { version = "0.5", features = ["hydrate"] }
sycamore = "0.9"  # Was 0.8.x
```

### 2. Remove Scope Parameters

**Before (0.4.x):**
```rust
fn my_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        h1 { "Hello World" }
    }
}
```

**After (0.5.x):**
```rust
fn my_page() -> View {
    view! {
        h1 { "Hello World" }
    }
}
```

### 3. Update View Functions with State

**Before (0.4.x):**
```rust
#[auto_scope]
fn my_page<G: Html>(cx: Scope, state: &MyStateRx) -> View<G> {
    view! { cx,
        h1 { (state.title.get_clone()) }
    }
}
```

**After (0.5.x):**
```rust
#[auto_scope]
fn my_page(state: MyStateRx) -> View {
    view! {
        h1 { (state.title.get_clone()) }
    }
}
```

### 4. Replace Anchor Tags with Link

**Before (0.4.x):**
```rust
view! { cx,
    a(href = "/about") { "Go to About" }
}
```

**After (0.5.x):**
```rust
view! {
    Link(to = "/about") { "Go to About" }
}
```

The `Link` component is now built into Perseus and handles client-side navigation automatically.

### 5. Update Signal Creation

**Before (0.4.x):**
```rust
fn my_component<G: Html>(cx: Scope) -> View<G> {
    let count = cx.create_signal(0);
    view! { cx,
        p { (count.get()) }
        button(on:click = |_| count.set(*count.get() + 1)) {
            "Increment"
        }
    }
}
```

**After (0.5.x):**
```rust
fn my_component() -> View {
    let count = create_signal(0);
    view! {
        p { (count.get()) }
        button(on:click = move |_| count.set(*count.get() + 1)) {
            "Increment"
        }
    }
}
```

Note: `create_signal` is now a free function, not a method on `Scope`.

### 6. Update Head Functions

**Before (0.4.x):**
```rust
#[engine_only_fn]
fn head<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        title { "My Page" }
    }
}
```

**After (0.5.x):**
```rust
#[engine_only_fn]
fn head() -> View {
    view! {
        title { "My Page" }
    }
}
```

### 7. Update Head with State

**Before (0.4.x):**
```rust
#[engine_only_fn]
fn head<G: Html>(cx: Scope, state: MyState) -> View<G> {
    view! { cx,
        title { (state.title) }
    }
}
```

**After (0.5.x):**
```rust
#[engine_only_fn]
fn head(state: MyState) -> View {
    view! {
        title { (state.title) }
    }
}
```

### 8. Update Template Registration

**Before (0.4.x):**
```rust
pub fn get_template<G: Html>() -> Template<G> {
    Template::build("my-page")
        .view(my_page)
        .build()
}
```

**After (0.5.x):**
```rust
pub fn get_template() -> Template {
    Template::build("my-page")
        .view(my_page)
        .build()
}
```

### 9. Update Error Views

**Before (0.4.x):**
```rust
ErrorViews::new(|cx, error, _ctx, _pos| {
    (
        view! { cx, title { "Error" } },
        view! { cx, h1 { "Error occurred" } }
    )
})
```

**After (0.5.x):**
```rust
ErrorViews::new(|error, _ctx, _pos| {
    (
        view! { title { "Error" } },
        view! { h1 { "Error occurred" } }
    )
})
```

### 10. Update Index View

**Before (0.4.x):**
```rust
.index_view(|cx| {
    view! { cx,
        html {
            head {}
            body {
                PerseusRoot()
            }
        }
    }
})
```

**After (0.5.x):**
```rust
.index_view(|| {
    view! {
        html {
            head {}
            body {
                PerseusRoot()
            }
        }
    }
})
```

## Common Patterns

### Conditional Rendering

**Before:**
```rust
view! { cx,
    (if *show.get() {
        view! { cx, p { "Visible" } }
    } else {
        view! { cx, }
    })
}
```

**After:**
```rust
view! {
    (if *show.get() {
        view! { p { "Visible" } }
    } else {
        view! {}
    })
}
```

### Iterating Over Collections

**Before:**
```rust
view! { cx,
    ul {
        Indexed(
            iterable = items,
            view = |cx, item| view! { cx,
                li { (item) }
            }
        )
    }
}
```

**After:**
```rust
view! {
    ul {
        Indexed(
            list = items,
            view = |item| view! {
                li { (item) }
            }
        )
    }
}
```

### Event Handlers

**Before:**
```rust
button(on:click = |_| {
    // handle click
})
```

**After:**
```rust
button(on:click = move |_| {
    // handle click - note the 'move' keyword is often needed
})
```

## Breaking Changes Summary

1. **No more `Scope` (`cx`) parameter** - Functions no longer receive a scope
2. **No more `G: Html` generic** - Views return `View` directly
3. **`view!` macro simplified** - No `cx` as first argument
4. **`Link` component for navigation** - Replaces `a` tags for internal links
5. **Free functions for signals** - `create_signal()` instead of `cx.create_signal()`
6. **Closure captures** - Often need `move` keyword for event handlers

## Troubleshooting

### "expected `View`, found `View<G>`"

Remove the `<G: Html>` generic and change return type to `View`.

### "cannot find value `cx` in this scope"

Remove `cx` from your function signature and `view!` macro calls.

### "use of moved value"

Add `move` keyword to closures in event handlers.

### Navigation not working

Replace `a(href = "...")` with `Link(to = "...")` for internal navigation.

## Full Example

**Before (0.4.x):**
```rust
use perseus::prelude::*;
use sycamore::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexStateRx")]
struct IndexState {
    greeting: String,
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexStateRx) -> View<G> {
    view! { cx,
        h1 { (state.greeting.get_clone()) }
        a(href = "/about") { "About" }
    }
}

#[engine_only_fn]
fn head<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        title { "Home" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head(head)
        .build()
}
```

**After (0.5.x):**
```rust
use perseus::prelude::*;
use sycamore::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexStateRx")]
struct IndexState {
    greeting: String,
}

#[auto_scope]
fn index_page(state: IndexStateRx) -> View {
    view! {
        h1 { (state.greeting.get_clone()) }
        Link(to = "/about") { "About" }
    }
}

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "Home" }
    }
}

pub fn get_template() -> Template {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head(head)
        .build()
}
```

## Getting Help

If you run into issues:

- Check the [examples](https://github.com/framesurge/perseus/tree/main/examples)
- Open a [GitHub discussion](https://github.com/framesurge/perseus/discussions)
- Join the [Discord](https://discord.com/invite/GNqWYWNTdp)
