# Build-time State

Build-time state is generated when you run `perseus build`. It's the most common state generation method.

## Basic Example

```rust
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "PageStateRx")]
struct PageState {
    greeting: String,
}

#[auto_scope]
fn page_view(state: PageStateRx) -> View {
    view! {
        h1 { (state.greeting.get_clone()) }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> PageState {
    PageState {
        greeting: "Hello from build time!".to_string(),
    }
}

pub fn get_template() -> Template {
    Template::build("greeting")
        .build_state_fn(get_build_state)
        .view_with_state(page_view)
        .build()
}
```

## How It Works

1. During `perseus build`, your `get_build_state` function runs
2. The returned state is serialized and saved
3. The page is pre-rendered to HTML with this state
4. On request, the pre-rendered page is served instantly

## StateGeneratorInfo

Your build state function receives `StateGeneratorInfo<T>`:

```rust
#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> PageState {
    // Get the page path (e.g., "hello" for /greeting/hello)
    let path = info.path;

    // Get the locale (e.g., "en-US")
    let locale = info.locale;

    // Access helper state (covered later)
    // let helper = info.extra;

    PageState {
        greeting: format!("Hello from {}", path),
    }
}
```

The generic `<T>` is for helper state. Use `()` if you don't need it.

## Error Handling

You can return errors instead of panicking:

```rust
use std::fs;

#[engine_only_fn]
async fn get_build_state(
    info: StateGeneratorInfo<()>
) -> Result<PageState, BlamedError<std::io::Error>> {
    let content = fs::read_to_string("content.txt")
        .map_err(|e| BlamedError::server(None, e))?;

    Ok(PageState { greeting: content })
}
```

### BlamedError

`BlamedError` annotates errors with who's responsible:

```rust
// Server's fault (most common)
BlamedError::server(None, my_error)

// With HTTP status code
BlamedError::server(Some(StatusCode::INTERNAL_SERVER_ERROR), my_error)

// Client's fault (for request-time)
BlamedError::client(Some(StatusCode::BAD_REQUEST), my_error)
```

**Tip**: Use `?` with `.into()` for automatic conversion:
```rust
let data = fs::read_to_string("file.txt")?;  // Auto-blames server
```

## Build Paths

Create multiple pages from one template using build paths:

```rust
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "PostStateRx")]
struct PostState {
    title: String,
    content: String,
}

#[auto_scope]
fn post_view(state: PostStateRx) -> View {
    view! {
        h1 { (state.title.get_clone()) }
        p { (state.content.get_clone()) }
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec![
            "hello-world".to_string(),
            "rust-tips".to_string(),
            "getting-started".to_string(),
        ],
        extra: ().into(),
    }
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> PostState {
    // info.path is "hello-world", "rust-tips", etc.
    match info.path.as_str() {
        "hello-world" => PostState {
            title: "Hello World".to_string(),
            content: "Welcome to my blog!".to_string(),
        },
        "rust-tips" => PostState {
            title: "Rust Tips".to_string(),
            content: "Some useful Rust tips...".to_string(),
        },
        _ => PostState {
            title: info.path.clone(),
            content: "Content for this post".to_string(),
        },
    }
}

pub fn get_template() -> Template {
    Template::build("post")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .view_with_state(post_view)
        .build()
}
```

This creates:
- `/post/hello-world`
- `/post/rust-tips`
- `/post/getting-started`

### BuildPaths Structure

```rust
BuildPaths {
    // List of page paths under this template
    paths: vec!["path1".to_string(), "path2".to_string()],

    // Helper state (use () if not needed)
    extra: ().into(),
}
```

### Special Paths

| Path | Result |
|------|--------|
| `""` (empty) | Template root (`/post` for `post` template) |
| `"nested/path"` | Nested URL (`/post/nested/path`) |
| `"with spaces"` | Auto URL-encoded |

## Practical Example: Markdown Blog

```rust
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use std::fs;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "PostStateRx")]
struct PostState {
    title: String,
    html_content: String,
}

#[auto_scope]
fn post_view(state: PostStateRx) -> View {
    view! {
        article {
            h1 { (state.title.get_clone()) }
            div(dangerously_set_inner_html = &state.html_content.get_clone())
        }
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    // Read all .md files from the posts directory
    let paths: Vec<String> = fs::read_dir("posts")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()? == "md" {
                path.file_stem()?.to_str().map(String::from)
            } else {
                None
            }
        })
        .collect();

    BuildPaths {
        paths,
        extra: ().into(),
    }
}

#[engine_only_fn]
async fn get_build_state(
    info: StateGeneratorInfo<()>
) -> Result<PostState, BlamedError<std::io::Error>> {
    let markdown = fs::read_to_string(format!("posts/{}.md", info.path))?;

    // Parse markdown (you'd use a proper parser here)
    let title = markdown.lines().next().unwrap_or("Untitled").to_string();
    let html_content = markdown; // In reality, convert to HTML

    Ok(PostState { title, html_content })
}

pub fn get_template() -> Template {
    Template::build("post")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .view_with_state(post_view)
        .build()
}
```

## When to Use Build-time State

**Good for:**
- Blog posts and articles
- Documentation pages
- Product catalogs
- Any content known at build time

**Not suitable for:**
- User-specific data
- Real-time information
- Content that changes per request

For dynamic content, see [Request-time State](/docs/state/request).
