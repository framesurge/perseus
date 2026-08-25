# Heads and Headers

Every web page needs metadata in its `<head>` element (title, meta tags, stylesheets) and sometimes custom HTTP headers (caching, cookies). Perseus provides template-level control over both.

## Setting the Head

Define a head function for each template to set page-specific metadata:

### Basic Head (No State)

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "About Us" }
        meta(name = "description", content = "Learn about our company")
        link(rel = "canonical", href = "https://example.com/about")
    }
}

fn about_page() -> View {
    view! {
        h1 { "About Us" }
        p { "Our story..." }
    }
}

pub fn get_template() -> Template {
    Template::build("about")
        .view(about_page)
        .head(head)
        .build()
}
```

### Head with State

Access your page state for dynamic metadata:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PostStateRx")]
struct PostState {
    title: String,
    description: String,
}

#[engine_only_fn]
fn head(state: PostState) -> View {
    view! {
        title { (format!("{} | My Blog", state.title)) }
        meta(name = "description", content = (state.description))
        meta(property = "og:title", content = (state.title))
    }
}

pub fn get_template() -> Template {
    Template::build("post")
        .build_state_fn(get_build_state)
        .view_with_state(post_page)
        .head_with_state(head)
        .build()
}
```

### Important Notes

1. **Engine-only** - Head functions are marked `#[engine_only_fn]` because they're prerendered server-side
2. **Synchronous** - Head functions cannot be async (don't read files here, do it in state generation)
3. **Return type** - Just returns `View`, Perseus handles the SSR context internally
4. **Errors** - Head functions can return `Result` if needed, but errors cause the page to fail

## Setting HTTP Headers

Set custom HTTP headers for caching, security, or other purposes:

### Basic Headers (No State)

```rust
use perseus::prelude::*;

#[engine_only_fn]
fn set_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CACHE_CONTROL,
        "max-age=3600".parse().unwrap()
    );
    headers
}

pub fn get_template() -> Template {
    Template::build("cached-page")
        .view(page_view)
        .set_headers(set_headers)
        .build()
}
```

### Headers with State

```rust
use perseus::prelude::*;

#[engine_only_fn]
fn set_headers(state: MyState) -> HeaderMap {
    let mut headers = HeaderMap::new();

    // Custom header based on state
    headers.insert(
        header::HeaderName::from_static("x-content-version"),
        state.version.parse().unwrap()
    );

    // Cache control
    headers.insert(
        header::CACHE_CONTROL,
        "public, max-age=86400".parse().unwrap()
    );

    headers
}

pub fn get_template() -> Template {
    Template::build("my-page")
        .build_state_fn(get_build_state)
        .view_with_state(page_view)
        .set_headers_with_state(set_headers)
        .build()
}
```

## Common Head Patterns

### SEO Metadata

```rust
#[engine_only_fn]
fn head(state: PageState) -> View {
    view! {
        title { (state.title) }
        meta(name = "description", content = (state.description))
        meta(name = "robots", content = "index, follow")

        // Open Graph
        meta(property = "og:title", content = (state.title))
        meta(property = "og:description", content = (state.description))
        meta(property = "og:type", content = "website")
        meta(property = "og:image", content = (state.image_url))

        // Twitter Card
        meta(name = "twitter:card", content = "summary_large_image")
        meta(name = "twitter:title", content = (state.title))
    }
}
```

### Stylesheets and Scripts

```rust
#[engine_only_fn]
fn head() -> View {
    view! {
        title { "My App" }
        link(rel = "stylesheet", href = ".perseus/static/styles.css")
        link(rel = "preconnect", href = "https://fonts.gstatic.com")
        // Note: scripts in head block rendering - use sparingly
    }
}
```

### Favicon and Icons

```rust
#[engine_only_fn]
fn head() -> View {
    view! {
        title { "My App" }
        link(rel = "icon", href = ".perseus/static/favicon.ico")
        link(rel = "apple-touch-icon", href = ".perseus/static/apple-touch-icon.png")
        link(rel = "manifest", href = ".perseus/static/manifest.json")
    }
}
```

## Common Header Patterns

### Cache Control

```rust
#[engine_only_fn]
fn set_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    // Cache for 1 hour, allow CDN caching
    headers.insert(
        header::CACHE_CONTROL,
        "public, max-age=3600, s-maxage=86400".parse().unwrap()
    );

    headers
}
```

### Security Headers

```rust
#[engine_only_fn]
fn set_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        "nosniff".parse().unwrap()
    );
    headers.insert(
        header::X_FRAME_OPTIONS,
        "DENY".parse().unwrap()
    );

    headers
}
```

## Index View vs Head Function

| Aspect | Index View | Head Function |
|--------|------------|---------------|
| Scope | Entire app | Per template |
| Content | HTML shell, global styles | Page-specific metadata |
| Set with | `.index_view()` on PerseusApp | `.head()` on Template |
| Dynamic | No | Yes (can use state) |

Use the index view for global concerns, head functions for page-specific metadata.

## Related

- [PerseusApp Configuration](/docs/fundamentals/perseus-app)
- [Static Content](/docs/fundamentals/static-content)
- [Build State](/docs/state/build)
