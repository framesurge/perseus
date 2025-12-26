# Error Views

When errors occur in Perseus (not in your business logic), error views tell Perseus how to display them. These are framework-level error handlers for things like 404s, network failures, and panics.

## When to Use Error Views

Error views handle **framework errors**, not your app's errors:

| Error Views Handle | You Handle Manually |
|-------------------|---------------------|
| 404 Not Found | Invalid form input |
| Network failure | Authentication failure |
| Hydration errors | Business logic errors |
| Panics | API response errors |

## Basic Error Views

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

pub fn get_error_views() -> ErrorViews {
    ErrorViews::new(|error, _error_context, error_position| {
        match error_position {
            ErrorPosition::Page => {
                // Full page error
                (
                    view! { title { "Error" } },
                    match &error {
                        ClientError::ServerError { status, .. } => {
                            if status.as_u16() == 404 {
                                view! {
                                    h1 { "Page Not Found" }
                                    p { "The page you requested doesn't exist." }
                                    Link(to = "/") { "Go Home" }
                                }
                            } else {
                                view! {
                                    h1 { "Server Error" }
                                    p { (format!("Error: {}", status)) }
                                }
                            }
                        },
                        ClientError::Panic(_) => view! {
                            h1 { "Application Crashed" }
                            p { "Please reload the page." }
                        },
                        _ => view! {
                            h1 { "Something Went Wrong" }
                        },
                    }
                )
            },
            ErrorPosition::Popup => {
                // Popup error (head is ignored)
                (
                    view! {},
                    view! {
                        p { "An error occurred. The page may not be fully interactive." }
                    }
                )
            },
            ErrorPosition::Widget => {
                // Widget error (head is ignored)
                (
                    view! {},
                    view! {
                        p { "Widget failed to load." }
                    }
                )
            },
        }
    })
}
```

## ClientError Variants

### ServerError

Errors propagated from the server (404, 500, etc.):

```rust
ClientError::ServerError { status, message } => {
    match status.as_u16() {
        404 => view! { h1 { "Not Found" } },
        403 => view! { h1 { "Forbidden" } },
        500 => view! { h1 { "Server Error" } },
        code if code >= 400 && code < 500 => view! {
            h1 { "Client Error" }
        },
        code if code >= 500 => view! {
            h1 { "Server Error" }
        },
        _ => view! { h1 { "Error" } },
    }
}
```

### FetchError

Network communication failures:

```rust
ClientError::FetchError(_) => view! {
    h1 { "Connection Error" }
    p { "Please check your internet connection." }
}
```

### Panic

Application crash (recovery not possible):

```rust
ClientError::Panic(panic_info) => view! {
    h1 { "Application Crashed" }
    p { "Please reload the page to continue." }
    // Optionally display panic info in development
}
```

### Other Variants

| Variant | Description |
|---------|-------------|
| `PluginError` | Plugin-related errors |
| `ThawError` | State deserialization failures |
| `PlatformError` | Critical platform failures |
| `PreloadError` | Preloading failures (usually mistyped paths) |
| `InvariantError` | Internal Perseus failures |

## Error Position

Perseus chooses where to display errors based on context:

| Position | When Used |
|----------|-----------|
| `Page` | Server errors on initial load |
| `Popup` | Client-side errors (preserves content) |
| `Widget` | Errors in capsule widgets |

### Why Popup Errors?

If hydration fails but the server rendered content fine, the user can still **see** the content. Replacing it with an error message would be worse UX. Popup errors preserve readable content while indicating limited interactivity.

Style popups with:
```css
#__perseus_popup_error {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    background: #fef2f2;
    border: 1px solid #fca5a5;
    padding: 1rem;
    border-radius: 0.5rem;
    z-index: 9999;
}
```

## Error Context

The `ErrorContext` indicates what Perseus features are available:

```rust
ErrorViews::new(|error, error_context, position| {
    match error_context {
        ErrorContext::Full => {
            // Full app available (translator, router, etc.)
        },
        ErrorContext::PluginsOnly => {
            // Only plugins available
        },
        ErrorContext::Static => {
            // Nothing available (static 404 page)
        },
        ErrorContext::None => {
            // Critical failure, minimal rendering
        },
    }
    // ...
})
```

## Development vs Production

Development default (not for production):
```rust
.error_views(ErrorViews::unlocalized_development_default())
```

For production, you **must** create custom error views. Perseus won't let you deploy with development defaults.

## Registering Error Views

```rust
#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(crate::error_views::get_error_views())
}
```

## Custom Subsequent Load Handling

Override how subsequent load errors are positioned:

```rust
ErrorViews::new(/* ... */)
    .subsequent_load_determinant_fn(|error| {
        // Return ErrorPosition::Page or ErrorPosition::Popup
        match error {
            ClientError::ServerError { status, .. } if status.as_u16() == 404 => {
                ErrorPosition::Page  // Show 404 as full page
            },
            _ => ErrorPosition::Popup,
        }
    })
```

## Complete Example

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

pub fn get_error_views() -> ErrorViews {
    ErrorViews::new(|error, ctx, pos| {
        let head = view! {
            title { "Error | My App" }
        };

        let body = match pos {
            ErrorPosition::Page => match &error {
                ClientError::ServerError { status, .. } => match status.as_u16() {
                    404 => view! {
                        div(class = "error-page") {
                            h1 { "404" }
                            p { "This page doesn't exist." }
                            Link(to = "/") { "Return Home" }
                        }
                    },
                    _ => view! {
                        div(class = "error-page") {
                            h1 { "Server Error" }
                            p { "Something went wrong on our end." }
                        }
                    },
                },
                ClientError::Panic(_) => view! {
                    div(class = "error-page") {
                        h1 { "Crash" }
                        p { "The app has crashed. Please reload." }
                    }
                },
                _ => view! {
                    div(class = "error-page") {
                        h1 { "Error" }
                        p { "An unexpected error occurred." }
                    }
                },
            },
            ErrorPosition::Popup | ErrorPosition::Widget => view! {
                p { "Error loading content." }
            },
        };

        (head, body)
    })
}
```

## Related

- [Error Handling Tutorial](/docs/first-app/error-handling)
- [ClientError API](https://docs.rs/perseus/latest/perseus/errors/enum.ClientError.html)
