# Debugging

Perseus apps run on two platforms: the engine (server) and the client (browser). Each requires different debugging approaches.

## Compile-Time Checks

Before debugging runtime issues, catch compile errors:

```bash
perseus check -w    # Watch mode, checks both platforms
perseus check -gw   # Also runs state generation
```

If `perseus check -gw` passes, most Perseus commands will work. Remaining issues are usually runtime bugs in request-time logic.

## Client-Side Debugging

### Console Logging

Standard `println!` and `dbg!` don't work in the browser. Use `web_log!`:

```rust
use perseus::prelude::*;

fn my_component() -> View {
    // Logs to browser console
    web_log!("Component rendered");
    web_log!("Value: {:?}", some_value);

    view! {
        p { "Hello" }
    }
}
```

On the engine-side, `web_log!` falls back to `println!`.

### Browser DevTools

Use your browser's developer tools:

1. **Console** - View `web_log!` output
2. **Network** - Monitor state fetches
3. **Elements** - Inspect rendered HTML
4. **Application** - Check localStorage, cookies

### Wasm-Specific Issues

For Wasm compilation errors:

```bash
perseus snoop wasm-build
```

This shows the raw Wasm build output without Perseus' formatting.

## Engine-Side Debugging

### Build-Time Logging

By default, Perseus hides build output unless errors occur. To see everything:

```bash
perseus snoop build
```

This runs the build process directly, showing all `dbg!` and `println!` output.

### Server Logging

For request-time debugging:

```bash
perseus build            # Build first
perseus snoop serve      # Then run server directly
```

Now you'll see all server-side logging. Note: You must run `perseus build` first.

### State Generation Debugging

```rust
#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> MyState {
    dbg!(&info.path);      // Shows in `perseus snoop build`
    println!("Generating state for: {}", info.path);

    MyState { /* ... */ }
}
```

## Common Issues

### Hydration Mismatches

**Symptom**: Page renders, then content changes or errors appear.

**Causes**:
- Random values during SSR
- Time-dependent content
- Browser-only APIs called during SSR

**Solution**:

```rust
fn my_view() -> View {
    // Don't do this - different values on server vs client
    // let random = rand::random::<u32>();

    // Do this instead - consistent or client-only
    #[cfg(target_arch = "wasm32")]
    let random = rand::random::<u32>();
    #[cfg(not(target_arch = "wasm32"))]
    let random = 0;

    view! { p { (random) } }
}
```

### State Not Updating

**Symptom**: State changes don't reflect in the UI.

**Causes**:
- Not using reactive state correctly
- Wrong signal access method

**Solution**:

```rust
fn counter() -> View {
    let count = create_signal(0);

    view! {
        p { (count.get()) }  // Reactive - updates automatically
        button(on:click = move |_| {
            count.set(count.get() + 1);
        }) {
            "Increment"
        }
    }
}
```

### Template Not Found

**Symptom**: 404 errors for pages that should exist.

**Causes**:
- Template not registered in `PerseusApp`
- Wrong template name
- Missing build paths

**Check**:

```rust
PerseusApp::new()
    .template(crate::templates::index::get_template())  // Is this registered?
    .template(crate::templates::post::get_template())
```

### Capsule Panics

**Symptom**: Panic when rendering a capsule.

**Causes**:
- Capsule rendered outside Perseus context
- Missing capsule registration

**Solution**: Ensure capsules are registered and only rendered within Perseus views:

```rust
PerseusApp::new()
    .capsule_ref(&*crate::capsules::my_capsule::MY_CAPSULE)
```

## Debug vs Release

Some issues only appear in release mode:

```bash
perseus serve -r    # Test release mode locally
```

Release builds:
- Optimize Wasm aggressively
- Remove debug assertions
- May expose timing-sensitive bugs

## Logging Levels

For verbose Perseus output:

```bash
RUST_LOG=debug perseus serve
```

For specific modules:

```bash
RUST_LOG=perseus=debug,my_app=trace perseus serve
```

## Related

- [Testing](/docs/fundamentals/testing)
- [Error Views](/docs/fundamentals/error-views)
- [Hydration](/docs/fundamentals/hydration)
