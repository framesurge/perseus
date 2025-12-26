# Improving Compilation Times

Perseus apps can take a while to compile due to the framework's complexity and Rust's compilation model. Here's how to speed things up.

## Quick Wins

### 1. Use Nightly Rust

Switch to nightly for faster compilation:

```bash
rustup override set nightly
```

This alone can nearly halve compile times. Switch back to stable for production builds if desired.

### 2. Export Instead of Serve

If your app doesn't need request-time features, use `#[perseus::main_export]`:

```rust
#[perseus::main_export]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        // ...
}
```

This avoids compiling a server integration.

## Advanced Optimizations

### Cranelift Backend

[Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift) is an alternative compiler backend that prioritizes compile speed over runtime performance.

1. **Install**: Follow the [precompiled builds guide](https://github.com/bjorn3/rustc_codegen_cranelift/#precompiled-builds)

2. **Verify installation**:
```bash
cargo-clif -h
```

3. **Use with Perseus**:
```bash
perseus serve -w --cargo-engine-path cargo-clif
```

**Warning**: Only use Cranelift for development. Production builds should use the standard compiler.

### What Cranelift Affects

Cranelift only applies to the engine (server) binary:
- ✅ State generation
- ✅ Server-side rendering
- ❌ Wasm compilation (no Cranelift support yet)

Fortunately, Wasm builds are already reasonably fast due to Perseus' target-gated compilation.

## Benchmark Results

Testing on the `basic` example with a cold cache:

| Configuration | Time |
|---------------|------|
| Stable, no optimizations | 28s |
| Nightly + Cranelift | 7s |

That's a **75% reduction** in compile time!

## Development vs Production

| Stage | Toolchain | Backend | Command |
|-------|-----------|---------|---------|
| Development | Nightly | Cranelift | `perseus serve -w --cargo-engine-path cargo-clif` |
| Testing | Nightly | Standard | `perseus test` |
| Production | Stable | Standard | `perseus deploy` |

## Additional Tips

### Use Watch Mode

```bash
perseus serve -w    # Incremental rebuilds
```

Watch mode only recompiles changed code.

### Minimize Dependencies

Large dependency trees slow compilation. Audit your `Cargo.toml`:

```bash
cargo tree | wc -l    # Count dependencies
```

### Feature Flags

Only enable features you need:

```toml
[dependencies]
perseus = { version = "0.5", default-features = false, features = ["..."] }
```

### Parallel Compilation

Ensure Cargo uses all cores:

```bash
# In .cargo/config.toml
[build]
jobs = 16  # Adjust to your CPU
```

### Incremental Compilation

Enable (usually on by default):

```bash
# In .cargo/config.toml
[build]
incremental = true
```

### SSD Storage

Compile on SSD, not HDD. Rust's compilation is I/O intensive.

## Profile-Guided Optimization

For the fastest production builds (at the cost of longer compile times):

```bash
# Build with PGO
RUSTFLAGS="-Cprofile-generate=/tmp/pgo" perseus deploy
# Run the app to generate profile data
# Then rebuild with profiles
RUSTFLAGS="-Cprofile-use=/tmp/pgo" perseus deploy
```

This is rarely necessary but can squeeze out extra runtime performance.

## Related

- [Debugging](/docs/fundamentals/debugging)
- [Serving and Exporting](/docs/fundamentals/serving-exporting)
