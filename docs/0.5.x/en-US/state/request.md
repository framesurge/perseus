# Request-time State

Request-time state is generated on each HTTP request, giving you access to cookies, headers, and other request data.

## When to Use

- User authentication and personalized content
- Dynamic data that changes per request
- Access to cookies or custom headers
- IP-based customization

## Basic Example

```rust
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "DashboardStateRx")]
struct DashboardState {
    username: String,
    ip_address: String,
}

#[auto_scope]
fn dashboard_view(state: DashboardStateRx) -> View {
    view! {
        h1 { "Welcome, " (state.username.get_clone()) "!" }
        p { "Your IP: " (state.ip_address.get_clone()) }
    }
}

#[engine_only_fn]
async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    req: Request,
) -> Result<DashboardState, BlamedError<std::io::Error>> {
    // Access request headers
    let ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown")
        .to_string();

    // In a real app, you'd validate a session cookie here
    let username = "User".to_string();

    Ok(DashboardState {
        username,
        ip_address: ip,
    })
}

pub fn get_template() -> Template {
    Template::build("dashboard")
        .request_state_fn(get_request_state)
        .view_with_state(dashboard_view)
        .build()
}
```

## The Request Object

Your request state function receives a `Request` with:

```rust
#[engine_only_fn]
async fn get_request_state(
    info: StateGeneratorInfo<()>,
    req: Request,
) -> MyState {
    // Access headers
    let headers = req.headers();

    // Get a specific header
    if let Some(auth) = headers.get("Authorization") {
        // Validate token...
    }

    // Get cookies (from Cookie header)
    if let Some(cookies) = headers.get("Cookie") {
        // Parse and use cookies...
    }

    // info still contains path and locale
    let path = info.path;
    let locale = info.locale;

    // ...
}
```

**Note**: The request body is not available. Use [custom API endpoints](/docs/fundamentals/head-headers) for POST data.

## Error Handling

Request-time functions use `BlamedError` to indicate who caused the error:

```rust
#[engine_only_fn]
async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    req: Request,
) -> Result<MyState, BlamedError<AuthError>> {
    // Check authentication
    let auth_header = req.headers()
        .get("Authorization")
        .ok_or_else(|| BlamedError::client(
            Some(http::StatusCode::UNAUTHORIZED),
            AuthError::MissingToken
        ))?;

    // Validate token
    let user = validate_token(auth_header)
        .map_err(|e| BlamedError::client(
            Some(http::StatusCode::FORBIDDEN),
            e
        ))?;

    Ok(MyState { user })
}
```

| Blame | When to Use | HTTP Status |
|-------|-------------|-------------|
| `BlamedError::client(...)` | Bad request, auth failure | 400, 401, 403 |
| `BlamedError::server(...)` | Database error, internal issue | 500 |

## Combining with Build State

You can have both build-time and request-time state:

```rust
pub fn get_template() -> Template {
    Template::build("page")
        .build_state_fn(get_build_state)      // Runs at build time
        .request_state_fn(get_request_state)  // Runs per request
        .view_with_state(page_view)
        .build()
}
```

When both are used:
1. Build state runs first (at build time)
2. Request state runs per request
3. Use [state amalgamation](/docs/state/amalgamation) to combine them

## Common Use Cases

### Authentication Check

```rust
#[engine_only_fn]
async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    req: Request,
) -> Result<AuthState, BlamedError<AuthError>> {
    let session_cookie = req.headers()
        .get("Cookie")
        .and_then(|c| parse_session_cookie(c.to_str().ok()?))
        .ok_or_else(|| BlamedError::client(
            Some(http::StatusCode::UNAUTHORIZED),
            AuthError::NotLoggedIn
        ))?;

    let user = validate_session(&session_cookie)
        .await
        .map_err(|e| BlamedError::server(None, e))?;

    Ok(AuthState {
        user_id: user.id,
        username: user.name,
        is_admin: user.role == "admin",
    })
}
```

### Locale-based Content

```rust
#[engine_only_fn]
async fn get_request_state(
    info: StateGeneratorInfo<()>,
    req: Request,
) -> ContentState {
    // Use Accept-Language header or info.locale
    let preferred_locale = req.headers()
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or(&info.locale);

    ContentState {
        greeting: get_localized_greeting(preferred_locale),
    }
}
```

## Performance Considerations

Request-time state runs on every request, so:

- Keep database queries minimal
- Cache where possible
- Consider if build-time + revalidation would work instead

For content that changes infrequently, see [Revalidation](/docs/state/revalidation).

## Important Notes

1. **The request is read-only** - Modifying it has no effect
2. **No request body** - Only headers and method are available
3. **Can't be exported** - Apps with request state need a server

## Template Configuration

```rust
Template::build("my-page")
    .request_state_fn(get_request_state)  // Enable request state
    .view_with_state(my_view)
    .build()
```

## Next Steps

- [State Amalgamation](/docs/state/amalgamation) - Combining build and request state
- [Revalidation](/docs/state/revalidation) - Refreshing build-time state
- [Heads and Headers](/docs/fundamentals/head-headers) - Setting response headers
