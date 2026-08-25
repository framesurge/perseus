# The Reactor

The `Reactor` is Perseus' central control system. Use it to:
- Get the current locale
- Access router state
- Preload pages
- Access global state
- Manage the page state store

## Accessing the Reactor

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn my_component() -> View {
    // Get the reactor
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    // Use it...
    let locale = reactor.get_locale();

    view! {
        p { "Current locale: " (locale) }
    }
}
```

## Common Operations

### Get Current Locale

```rust
let reactor = Reactor::<BrowserNodeType>::from_cx();
let locale = reactor.get_locale();
```

### Access Global State

```rust
use crate::global_state::AppStateRx;

let reactor = Reactor::<BrowserNodeType>::from_cx();
let global_state = reactor.get_global_state::<AppStateRx>();

// Use the reactive global state
let theme = global_state.theme.get_clone();
```

### Preload a Page

```rust
let reactor = Reactor::<BrowserNodeType>::from_cx();

// Preload when user hovers over a link
button(on:mouseenter = move |_| {
    reactor.preload("/about");
}) {
    "Go to About"
}
```

### Access Router State

```rust
let reactor = Reactor::<BrowserNodeType>::from_cx();
let route_info = reactor.router_state.get_load_state();
```

## Node Types

The reactor is generic over the rendering backend:

| Type | When Used |
|------|-----------|
| `BrowserNodeType` | Client-side (browser) |
| `SsrNode` | Engine-side (server) |

In views, use `BrowserNodeType` since views run in the browser after hydration.

## Engine vs Client

The reactor behaves differently on each platform:

**Engine-side:**
- Used during server-side rendering
- Limited functionality (no browser APIs)

**Client-side:**
- Full functionality
- Access to browser APIs
- Manages reactive state

## Example: Theme Toggle with Global State

```rust
use perseus::prelude::*;
use sycamore::prelude::*;
use crate::global_state::ThemeStateRx;

fn theme_toggle() -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();
    let theme = reactor.get_global_state::<ThemeStateRx>();

    view! {
        button(on:click = move |_| {
            let current = theme.mode.get_clone();
            theme.mode.set(if current == "light" {
                "dark".to_string()
            } else {
                "light".to_string()
            });
        }) {
            "Toggle Theme: " (theme.mode.get_clone())
        }
    }
}
```

## Example: Preloading on Hover

```rust
fn nav_link(path: &'static str, label: &'static str) -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    view! {
        Link(
            to = path,
            // Preload when user hovers
            // (Note: actual preload API may vary)
        ) {
            (label)
        }
    }
}
```

## Important Notes

1. **Use correct node type** - Mismatched types cause confusing errors
2. **Don't mix platforms** - Don't try SSR in the browser through Perseus
3. **Capsules require proper context** - Rendering capsules outside Perseus causes panics

## Related

- [Global State](/docs/state/global)
- [Preloading](/docs/fundamentals/preloading)
- [Routing](/docs/fundamentals/routing)
