# Routing and Navigation

Perseus uses page-based programming where each view is a separate page with its own state. This guide covers how to navigate between pages.

## The Link Component

For internal navigation, use the `Link` component:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn my_page() -> View {
    view! {
        h1 { "Home" }
        Link(to = "/about") { "Go to About" }
        Link(to = "/blog/hello-world") { "Read Blog Post" }
    }
}
```

The `Link` component:
- Handles client-side navigation (no full page reload)
- Manages loading states
- Integrates with Perseus' caching system
- Works with localized routes automatically

## Imperative Navigation

For programmatic navigation (e.g., after form submission):

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn login_form() -> View {
    let handle_login = move |_| {
        // After successful login...
        navigate("/dashboard");
    };

    view! {
        button(on:click = handle_login) { "Login" }
    }
}
```

### Navigation Functions

| Function | Behavior |
|----------|----------|
| `navigate("/path")` | Navigate, add to history |
| `navigate_replace("/path")` | Navigate, replace current history entry |

Use `navigate_replace` when you don't want the user to go back (e.g., after a redirect).

## Route Behavior

Perseus sets a `<base>` tag that makes all routes relative to the site root:

```rust
// From any page, these go to:
Link(to = "/about")           // → /about
Link(to = "/blog/post-1")     // → /blog/post-1
Link(to = "about")            // → /about (same as above)
```

**Note**: Unlike some frameworks, `/my/page` linking to `foo` goes to `/foo`, not `/my/foo`.

## External Links

For external URLs, use regular anchor tags:

```rust
view! {
    // Internal - use Link
    Link(to = "/about") { "About Us" }

    // External - use anchor tag
    a(href = "https://github.com", target = "_blank") {
        "GitHub"
    }
}
```

## Localized Routing

For internationalized apps, use the `link!` macro to prepend the current locale:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn nav() -> View {
    view! {
        // For i18n apps, this ensures correct locale prefix
        Link(to = link!("/about")) { "About" }
    }
}
```

Without `link!`, navigating from `/en-US/home` to `/about` would go to `/about` instead of `/en-US/about`.

See [Internationalization](/docs/fundamentals/i18n) for more details.

## Query Parameters

Access query parameters in your state generation:

```rust
#[engine_only_fn]
async fn get_request_state(
    info: StateGeneratorInfo<()>,
    req: Request,
) -> MyState {
    // Parse query string from the request URI
    let uri = req.uri();
    let query = uri.query().unwrap_or("");

    // Parse as needed...
    MyState { /* ... */ }
}
```

## Preloading

Preload pages before navigation for instant transitions:

```rust
fn nav_item() -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    view! {
        // Preload on hover for faster navigation
        div(on:mouseenter = move |_| {
            // Preloading API (see preloading docs)
        }) {
            Link(to = "/heavy-page") { "Heavy Page" }
        }
    }
}
```

See [Preloading](/docs/fundamentals/preloading) for details.

## Dynamic Routes

Create dynamic routes with build paths:

```rust
// Template: "post"
// Build paths: ["hello", "world", "rust-tips"]
//
// Results in:
// /post/hello
// /post/world
// /post/rust-tips

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec![
            "hello".to_string(),
            "world".to_string(),
            "rust-tips".to_string(),
        ],
        extra: ().into(),
    }
}
```

Access the current path in your state generator:

```rust
#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> PostState {
    let slug = info.path;  // "hello", "world", etc.
    // Fetch post by slug...
    PostState { /* ... */ }
}
```

## 404 Handling

Unknown routes trigger error views with a 404 status:

```rust
// In your error views
ClientError::ServerError { status, .. } => {
    if status.as_u16() == 404 {
        view! {
            h1 { "Page Not Found" }
            Link(to = "/") { "Go Home" }
        }
    } else {
        // Other errors...
    }
}
```

## Summary

| Task | Solution |
|------|----------|
| Internal link | `Link(to = "/path")` |
| External link | `a(href = "https://...")` |
| Programmatic nav | `navigate("/path")` |
| Replace history | `navigate_replace("/path")` |
| Localized link | `link!("/path")` |
| Preload | See preloading docs |

## Related

- [Preloading](/docs/fundamentals/preloading)
- [Internationalization](/docs/fundamentals/i18n)
- [Error Views](/docs/fundamentals/error-views)
