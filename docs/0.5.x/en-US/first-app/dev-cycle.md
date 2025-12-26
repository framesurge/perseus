# Development Cycle

This guide covers the commands and workflow for developing Perseus apps.

## Two Modes of Development

When developing a Perseus app, you'll alternate between:

1. **Coding mode** - Writing business logic, checking for errors
2. **Preview mode** - Seeing your app in the browser, styling, testing features

## Quick Type Checking

For fast feedback while coding, use:

```sh
perseus check -w
```

This runs `cargo check` on both engine-side and browser-side code. It's much faster than a full build because it only checks for errors without compiling.

Add `-g` to also check your build logic:

```sh
perseus check -gw
```

## IDE Setup

To get proper syntax highlighting and error detection in your IDE, create `.cargo/config.toml`:

```toml
[build]
rustflags = [ "--cfg", "engine" ]
```

This tells your IDE to check the engine-side code by default.

**Tip**: When working on browser-only logic, temporarily change `engine` to `client` to get proper IDE support for that code.

## Running Your App

When you need to see your app in a browser:

```sh
perseus serve -w
```

This:
1. Builds your app
2. Starts a development server
3. Opens your app at <http://localhost:8080>

The `-w` flag enables watch mode - changes to your code trigger automatic rebuilds.

## Rebuild Speed

| Change Type | Rebuild Speed |
|-------------|--------------|
| Static files (CSS, images) | Near instant |
| Rust code | Slower (Rust compilation) |

This is a tradeoff of Rust web development: slower builds but faster, more reliable apps.

## Debugging with `perseus snoop`

The standard commands hide most output. To see all logs and debug output:

```sh
perseus snoop build     # View build output
perseus snoop wasm-build   # View Wasm compilation output
perseus snoop serve     # View server output
```

Use these when you need to see `dbg!()` output or detailed error messages.

## Custom Watch Paths

Watch additional directories beyond your source code:

```sh
perseus serve -w --custom-watch ../docs
```

Exclude paths with `!`:

```sh
perseus serve -w --custom-watch !./generated
```

## Common Commands Reference

| Command | Purpose |
|---------|---------|
| `perseus check -w` | Fast type checking with watch |
| `perseus serve -w` | Development server with watch |
| `perseus build` | Build without serving |
| `perseus export` | Export as static files |
| `perseus deploy` | Production build |
| `perseus clean` | Clear build artifacts |
| `perseus snoop [cmd]` | Run command with full output |

## Workflow Tips

1. **Use `check` while coding** - It's much faster than `serve`
2. **Only `serve` when you need to see the UI** - Visual testing, styling
3. **Use `snoop` for debugging** - See all output including `dbg!()` calls
4. **Keep terminal visible** - Watch mode shows compile errors immediately

## Example Workflow

```sh
# Start development session
cd my-perseus-app

# Fast iteration while coding
perseus check -w

# When ready to test in browser
perseus serve -w

# If something isn't working, debug
perseus snoop serve

# Build for production
perseus deploy
```

## Next Steps

Ready to ship? Learn about [deploying your app](/docs/first-app/deploying)!
