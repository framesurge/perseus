# Installing Perseus

This guide walks you through setting up Perseus from scratch.

## Prerequisites

1. **Install Rust** using [rustup](https://rustup.rs):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Add the WebAssembly target**:
   ```sh
   rustup target add wasm32-unknown-unknown
   ```

## Install the Perseus CLI

The CLI manages building, serving, and deploying your app:

```sh
cargo install perseus-cli
```

## Create a New Project

### Quick Start (Recommended)

```sh
perseus new my-app
cd my-app
```

This creates a ready-to-run project.

### Manual Setup

If you prefer to understand each piece, create manually:

```sh
cargo new my-app
cd my-app
```

#### 1. Configure IDE Support

Create `.cargo/config.toml`:

```toml
[build]
rustflags = [ "--cfg", "engine" ]
rustdocflags = [ "--cfg", "engine" ]
```

This enables proper IDE support. Change `engine` to `client` when working on browser-only code.

#### 2. Set Up Dependencies

Replace your `Cargo.toml` with:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
perseus = { version = "0.5", features = ["hydrate"] }
sycamore = "0.9"
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[target.'cfg(engine)'.dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
perseus-axum = "0.5"
```

## Understanding the Dependencies

| Dependency | Purpose |
|------------|---------|
| `perseus` | The framework core |
| `sycamore` | UI library for views |
| `serde`, `serde_json` | Serialization for state transfer |
| `tokio` | Async runtime (engine-only) |
| `perseus-axum` | Server integration (engine-only) |

## Engine vs Client

Perseus has two build targets:

- **Engine**: Runs on your server (prerendering, serving)
- **Client**: Runs in the browser (WebAssembly, interactivity)

Use `#[cfg(engine)]` and `#[cfg(client)]` to target specific code:

```rust
#[cfg(engine)]
fn server_only_function() {
    // Only compiled for the server
}

#[cfg(client)]
fn browser_only_function() {
    // Only compiled for WebAssembly
}
```

**Why separate them?**
- Smaller browser bundles (no server code in Wasm)
- Faster compilation (only compile what's needed)
- Access platform-specific APIs

## Server Integrations

Perseus supports multiple server frameworks:

| Integration | Crate |
|-------------|-------|
| Axum (recommended) | `perseus-axum` |
| Warp | `perseus-warp` |
| Actix Web | `perseus-actix-web` |

## Cargo Workspaces

If using Perseus in a Cargo workspace, add this to your root `Cargo.toml`:

```toml
[workspace]
resolver = "2"
```

This is **required** - Perseus won't compile without it.

## Verify Installation

Create a minimal app and run it:

```sh
# If you used `perseus new`
perseus serve

# Should open at http://localhost:8080
```

## Troubleshooting

### "target not found" errors

Make sure you added the Wasm target:
```sh
rustup target add wasm32-unknown-unknown
```

### IDE shows errors everywhere

Check that `.cargo/config.toml` exists with the `rustflags` set.

### Workspace compilation fails

Ensure `resolver = "2"` is set in your workspace root.

## Next Steps

Now let's [define your app](/docs/first-app/defining)!
