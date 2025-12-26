# Global State

Global state is shared across all pages in your app. Use it for user preferences, authentication status, or any data that should persist across navigation.

## Basic Example

```rust
// src/global_state.rs
use perseus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    pub theme: String,
    pub logged_in: bool,
    pub username: Option<String>,
}

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new()
        .build_state_fn(get_build_state)
}

#[engine_only_fn]
async fn get_build_state(_locale: String) -> AppState {
    AppState {
        theme: "light".to_string(),
        logged_in: false,
        username: None,
    }
}
```

## Registering Global State

Add it to your `PerseusApp`:

```rust
// src/main.rs
use perseus::prelude::*;

mod global_state;
mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .global_state_creator(crate::global_state::get_global_state_creator())
        .error_views(ErrorViews::unlocalized_development_default())
}
```

## Accessing Global State

Use the reactor to access global state from any view:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;
use crate::global_state::AppStateRx;

fn my_view() -> View {
    // Get the reactor
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    // Access global state
    let global_state = reactor.get_global_state::<AppStateRx>();

    view! {
        div(class = format!("theme-{}", global_state.theme.get_clone())) {
            h1 { "My App" }

            (if *global_state.logged_in.get() {
                view! {
                    p { "Welcome, " (global_state.username.get_clone().unwrap_or_default()) }
                    button(on:click = move |_| {
                        global_state.logged_in.set(false);
                        global_state.username.set(None);
                    }) {
                        "Logout"
                    }
                }
            } else {
                view! {
                    button(on:click = move |_| {
                        global_state.logged_in.set(true);
                        global_state.username.set(Some("User".to_string()));
                    }) {
                        "Login"
                    }
                }
            })
        }
    }
}
```

## Global State Methods

| Method | Description |
|--------|-------------|
| `.get_global_state::<T>()` | Get global state (panics if wrong type) |
| `.try_get_global_state::<T>()` | Get global state (returns `Option`) |

## Request-time Global State

You can also generate global state per request:

```rust
pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new()
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
}

#[engine_only_fn]
async fn get_build_state(_locale: String) -> AppState {
    AppState {
        theme: "light".to_string(),
        logged_in: false,
        username: None,
    }
}

#[engine_only_fn]
async fn get_request_state(
    _locale: String,
    req: Request,
) -> Result<AppState, BlamedError<std::io::Error>> {
    // Check for auth cookie
    let logged_in = req.headers()
        .get("Cookie")
        .map(|c| c.to_str().unwrap_or("").contains("session="))
        .unwrap_or(false);

    Ok(AppState {
        theme: "light".to_string(),
        logged_in,
        username: if logged_in {
            Some("User".to_string())
        } else {
            None
        },
    })
}
```

### Combining Build and Request State

Use amalgamation to merge both:

```rust
pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new()
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
        .amalgamate_states_fn(amalgamate_states)
}

#[engine_only_fn]
fn amalgamate_states(
    build_state: AppState,
    request_state: AppState,
) -> AppState {
    AppState {
        // Keep theme from build (or could use request)
        theme: build_state.theme,
        // Use auth info from request
        logged_in: request_state.logged_in,
        username: request_state.username,
    }
}
```

## Common Use Cases

### Theme Preference

```rust
#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "ThemeStateRx")]
pub struct ThemeState {
    pub mode: String,  // "light" or "dark"
}

// In a component:
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
            "Toggle Theme"
        }
    }
}
```

### Shopping Cart

```rust
#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "CartStateRx")]
pub struct CartState {
    #[rx(nested)]
    pub items: Vec<CartItem>,
    pub total: f64,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CartItemRx")]
pub struct CartItem {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}
```

## Important Warnings

### Request-time Global State Pitfalls

If you use **only** request-time global state:

1. **Build-time access will panic** - Any page using `.get_global_state()` during build will crash
2. **Hydration errors** - Pages with build state won't see request data until client-side

**Recommendation**: Always provide build-time defaults, even if you override at request-time.

### Safe Pattern

```rust
pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new()
        .build_state_fn(get_build_state)  // Always provide defaults
        .request_state_fn(get_request_state)  // Override per-request
}

#[engine_only_fn]
async fn get_build_state(_locale: String) -> AppState {
    // Safe defaults
    AppState {
        logged_in: false,
        username: None,
    }
}
```

## Differences from Page State

| Feature | Page State | Global State |
|---------|------------|--------------|
| Scope | Single page | Entire app |
| Cached | In PSS (25 pages) | Always available |
| Updates | Per-page | Shared across pages |
| Clone required | Yes | No |

## Summary

```rust
// 1. Define global state
#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "AppStateRx")]
pub struct AppState { /* ... */ }

// 2. Create the creator
pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new()
        .build_state_fn(get_build_state)
}

// 3. Register in PerseusApp
PerseusApp::new()
    .global_state_creator(get_global_state_creator())

// 4. Use in views
let reactor = Reactor::<BrowserNodeType>::from_cx();
let state = reactor.get_global_state::<AppStateRx>();
```

## Next Steps

- [Freezing and Thawing](/docs/state/freezing-thawing) - Persist state to localStorage
- [The Reactor](/docs/fundamentals/reactor) - More reactor capabilities
