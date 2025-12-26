# Serving and Exporting

Perseus apps can be deployed two ways: **served** (with a server) or **exported** (static files only). Each has trade-offs.

## The Build Process

### 1. Preparation

Perseus reads your `PerseusApp` configuration to understand:
- Templates and capsules
- Internationalization settings
- State generation requirements

### 2. Building

Perseus executes build-time logic:
- Calls `get_build_state` and `get_build_paths` for each template
- Prerenders pages to HTML
- Creates the render configuration

### 3. Wasm Compilation

Simultaneously, Perseus compiles your app to WebAssembly:
- Creates `bundle.js` and `bundle.wasm`
- Optimizes for size in release mode

### 4a. Serving

If you run `perseus serve`:

```bash
perseus serve       # Development
perseus serve -r    # Release mode
```

Perseus starts a server that:
- Handles initial page loads with SSR
- Serves the Wasm bundle
- Processes request-time state generation
- Handles revalidation and incremental generation
- Resolves nested capsules just-in-time

### 4b. Exporting

If you run `perseus export`:

```bash
perseus export      # Development
perseus export -s   # With local server
```

Perseus generates static files:
- Pre-renders all pages to HTML files
- Organizes files to match URL structure
- Creates `dist/exported/` ready for deployment

## Serving vs Exporting

| Feature | Served | Exported |
|---------|--------|----------|
| Request-time state | ✅ | ❌ |
| Incremental generation | ✅ | ❌ |
| Revalidation | ✅ | ❌ |
| API routes | ✅ | ❌ |
| Cookies/Auth | ✅ | ❌ |
| CDN deployment | Harder | Easy |
| Hosting cost | Higher | Lower |
| Setup complexity | More | Less |

**Rule of thumb**: If you can export, export. Static files are faster, cheaper, and simpler.

## Server Integrations

Perseus supports multiple server frameworks:

| Integration | Crate |
|-------------|-------|
| Axum | `perseus-axum` |
| Actix Web | `perseus-actix-web` |
| Warp | `perseus-warp` |

### Default Server

Most apps use the default server:

```rust
#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        // ...
}
```

### Custom Server

Add API routes or middleware:

```rust
use axum::{Router, routing::get};
use perseus_axum::ServerOptions;

async fn api_handler() -> &'static str {
    "Hello from API"
}

#[perseus::main(custom_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        // ...
}

async fn custom_server(
    turbine: &'static Turbine<impl TranslationsManager>,
    opts: ServerOptions,
) {
    let app = Router::new()
        .route("/api/hello", get(api_handler))
        .merge(perseus_axum::get_router(turbine, opts).await);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Deployment Commands

### Development

```bash
perseus serve -w    # Watch mode, auto-reload
perseus export -sw  # Export with watch and server
```

### Production

```bash
perseus deploy           # Served app → pkg/
perseus deploy -e        # Exported app → pkg/
```

The `deploy` command:
- Builds in release mode
- Optimizes Wasm aggressively
- Creates a ready-to-deploy `pkg/` directory

## Error Pages for Exported Apps

Static file hosts need pre-exported error pages:

```bash
# Export 404 page
perseus export-error-page --code 404 --output pkg/404.html

# Export 500 page
perseus export-error-page --code 500 --output pkg/500.html
```

Most hosts (GitHub Pages, Netlify, Vercel) automatically serve `404.html` for missing routes.

**Note**: For i18n apps, exported error pages can't be localized since the user's locale isn't known. Prefer serving for i18n apps when possible.

## Performance Tips

### For Served Apps

1. Use a reverse proxy (nginx, Caddy) for TLS
2. Enable HTTP/2
3. Set appropriate cache headers
4. Consider a CDN for static assets

### For Exported Apps

1. Deploy to edge networks (Cloudflare, Vercel)
2. Enable Brotli/gzip compression
3. Set long cache times for hashed assets
4. Pre-compress files if your host supports it

## Related

- [PerseusApp Configuration](/docs/fundamentals/perseus-app)
- [Debugging](/docs/fundamentals/debugging)
- [Deploying Tutorial](/docs/first-app/deploying)
