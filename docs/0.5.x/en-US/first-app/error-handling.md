# Error Handling

Perseus gives you full control over how errors are displayed. Instead of generic error pages, you define custom views for different types of errors.

## Why Custom Error Views?

Perseus doesn't want to show bright red error messages if your website uses a completely different style. You provide `View`s that match your app's design.

## Basic Error Views

Here's a simple error handling setup:

```rust
// src/error_views.rs
use perseus::prelude::*;
use sycamore::prelude::*;

pub fn get_error_views() -> ErrorViews {
    ErrorViews::new(|error, _error_context, _error_position| {
        // Return (head, body)
        (
            view! {
                title { "Error" }
            },
            match &error {
                ClientError::ServerError { status, .. } => match status.as_u16() {
                    404 => view! {
                        h1 { "Page Not Found" }
                        p { "The page you're looking for doesn't exist." }
                        Link(to = "/") { "Go Home" }
                    },
                    _ => view! {
                        h1 { "Server Error" }
                        p { (format!("Error {}", status)) }
                    },
                },
                ClientError::Panic(_) => view! {
                    h1 { "Critical Error" }
                    p { "The app has crashed. Please refresh the page." }
                },
                ClientError::FetchError(_) => view! {
                    h1 { "Connection Error" }
                    p { "Please check your internet connection and try again." }
                },
                _ => view! {
                    h1 { "Something Went Wrong" }
                    p { "An unexpected error occurred." }
                },
            }
        )
    })
}
```

## Understanding ClientError

The `ClientError` enum has several variants:

| Variant | When It Occurs |
|---------|---------------|
| `ServerError` | Server-side errors (404, 500, etc.) |
| `Panic` | App crashed due to a panic |
| `FetchError` | Network failure or server unreachable |
| `InvariantError` | Internal Perseus errors |

## Registering Error Views

Add your error views to `PerseusApp`:

```rust
// src/main.rs
use perseus::prelude::*;

mod error_views;
mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(crate::error_views::get_error_views())
}
```

## Error Position

Sometimes errors appear as popups instead of full pages. This happens when:

- The page content rendered fine but the client couldn't initialize
- The user can still see content, it's just not interactive

For popup errors, the head is ignored and your body renders in a popup styled with `#__perseus_popup_error`.

```rust
ErrorViews::new(|error, _ctx, position| {
    match position {
        ErrorPosition::Page => {
            // Full page error
            (
                view! { title { "Error" } },
                view! { h1 { "Error" } p { "Something went wrong." } }
            )
        },
        ErrorPosition::Popup => {
            // Popup error (head is ignored)
            (
                view! {},
                view! { p { "An error occurred. The page may not be interactive." } }
            )
        },
    }
})
```

## Development vs Production

During development, you can use the built-in default:

```rust
.error_views(ErrorViews::unlocalized_development_default())
```

For production, **always create custom error views**. Perseus won't let you deploy with the development defaults.

## HTTP Status Codes

When handling `ServerError`, check the status code:

| Code | Meaning |
|------|---------|
| 404 | Page not found |
| 403 | Forbidden |
| 500 | Internal server error |
| 502/503 | Server unavailable |

```rust
ClientError::ServerError { status, .. } => {
    let code = status.as_u16();
    if code == 404 {
        // Not found
    } else if code >= 500 {
        // Server error
    } else {
        // Other client error
    }
}
```

## Styling Error Pages

Error pages are just normal views. Style them with CSS:

```css
/* In your stylesheet */
.error-page {
    text-align: center;
    padding: 2rem;
}

.error-page h1 {
    color: #e53e3e;
}

/* Popup error styling */
#__perseus_popup_error {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    background: #fef2f2;
    border: 1px solid #fca5a5;
    padding: 1rem;
    border-radius: 0.5rem;
}
```

## Next Steps

Now that you've set up error handling, let's [run your app](/docs/first-app/dev-cycle)!
