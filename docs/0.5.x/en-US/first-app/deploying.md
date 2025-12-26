# Deploying Your App

You've built your Perseus app - now let's deploy it!

## Test Before Deploying

First, make sure everything compiles:

```sh
perseus check
```

Then test locally:

```sh
perseus serve
```

Visit <http://localhost:8080> and verify:
- Pages load correctly
- Navigation works (links should be instant)
- Error pages work (try <http://localhost:8080/nonexistent>)

## Production Build

When you're ready to deploy:

```sh
perseus deploy
```

This command:
1. Optimizes your code for production
2. Minimizes the Wasm bundle size
3. Creates a `pkg/` directory with everything needed

The build takes longer than development builds because of these optimizations.

## Running in Production

The `pkg/` folder contains:
- `server` - The server binary
- Static assets and prerendered pages

To run:

```sh
./pkg/server
```

Your app will be available at <http://localhost:8080>.

### Configure Host and Port

Use environment variables:

```sh
PERSEUS_HOST=0.0.0.0 PERSEUS_PORT=80 ./pkg/server
```

| Variable | Default | Description |
|----------|---------|-------------|
| `PERSEUS_HOST` | `127.0.0.1` | Bind address |
| `PERSEUS_PORT` | `8080` | Port number |

### HTTPS

Perseus doesn't handle HTTPS directly. Use a reverse proxy:
- Nginx
- Caddy
- Cloud provider load balancer

## Static Export

If your app doesn't need server-side features (no request-time state), you can export as static files:

```sh
perseus deploy -e
```

This creates static HTML files you can host anywhere:
- GitHub Pages
- Netlify
- Vercel
- Any static file host

### Test Static Export Locally

```sh
perseus export -s
```

The `-s` flag starts a file server for testing.

### Serve with Python (for testing)

```sh
python -m http.server -d pkg/
```

## When to Use Each Deployment Type

| Use Case | Deployment Type |
|----------|----------------|
| Static content (docs, blog) | Export (`-e`) |
| User authentication | Server |
| Request-time data | Server |
| Real-time features | Server |
| Cheapest hosting | Export |

## Performance Tips

### Wasm Bundle Size

The Wasm bundle is the main factor in initial load time. To minimize it:

1. **Use release builds** (automatic with `perseus deploy`)
2. **Minimize dependencies** - Only include what you need in browser code
3. **Use `#[cfg(client)]`** - Keep server-only code out of the bundle

### Alternative Allocators

For smaller bundles, consider alternative allocators (use only for client):

```rust
#[cfg(client)]
#[global_allocator]
static ALLOC: lol_alloc::LeakingAllocator = lol_alloc::LeakingAllocator;
```

**Warning**: Only use these for the client build, not the server!

## Hosting Options

### Self-Hosted

Run the server binary on any Linux server:
- AWS EC2
- DigitalOcean Droplets
- Your own server

### Container Deployment

Create a Dockerfile:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo install perseus-cli
RUN perseus deploy

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/pkg ./pkg
EXPOSE 8080
CMD ["./pkg/server"]
```

### Static Hosting (Export Only)

For exported apps:
- **GitHub Pages**: Free, easy setup
- **Netlify**: Free tier, automatic deploys
- **Vercel**: Free tier, edge functions
- **Cloudflare Pages**: Free, fast CDN

## Deployment Checklist

- [ ] Custom error views (not development defaults)
- [ ] `perseus check` passes
- [ ] Tested locally with `perseus serve`
- [ ] Environment variables configured
- [ ] HTTPS set up (if using server deployment)

## Troubleshooting

### "Development error views not allowed in production"

Create custom error views. See [Error Handling](/docs/first-app/error-handling).

### Server exits immediately

Check logs for errors. Common issues:
- Port already in use
- Missing permissions
- Missing files in `pkg/`

### Export fails

Your app might use features that require a server:
- Request-time state
- Revalidation
- Server-side APIs

Use server deployment instead, or refactor to use build-time state only.

## What's Next?

Congratulations on deploying your first Perseus app!

Explore more:
- [State Management](/docs/state/intro) - Dynamic data handling
- [Internationalization](/docs/fundamentals/i18n) - Multi-language support
- [Capsules](/docs/capsules/intro) - Reusable widgets with state
- [Examples](https://github.com/framesurge/perseus/tree/main/examples) - Real-world code samples

Need help? [Open a discussion](https://github.com/framesurge/perseus/discussions) or join our [Discord](https://discord.com/invite/GNqWYWNTdp)!
