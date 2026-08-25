# Plugins

Perseus is extensible through plugins—library crates that hook into the build process and runtime. For most customizations, consider a [custom server](/docs/fundamentals/serving-exporting) first, as it's simpler.

## Plugin Types

### Functional Plugins

Receive data, process it, and return results. Multiple can act on the same opportunity.

**Example use cases**:
- Adding static aliases
- Injecting HTML into pages
- Modifying build configuration

### Control Plugins

Take exclusive control of a feature. Only one can act per opportunity.

**Example use cases**:
- Replacing the index view
- Custom routing logic
- Alternative state stores

## Using Plugins

Add a plugin to your `PerseusApp`:

```rust
use perseus::prelude::*;
use some_plugin::SomePlugin;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .plugins(Plugins::new().plugin(
            SomePlugin::new(),
            SomePluginData { /* config */ }
        ))
        .error_views(ErrorViews::unlocalized_development_default())
}
```

## Tinker Plugins

Special plugins that run during `perseus tinker`:

```bash
perseus tinker
```

Use cases:
- Custom build processes
- Code generation
- Asset processing
- Modifying user code

Example registration:

```rust
Plugins::new()
    .plugin(
        MyTinkerPlugin::new(),
        MyTinkerData::default()
    )
```

Tinker plugins have largely been superseded by standard Cargo build scripts, but remain available for Perseus-specific transformations.

## Writing Plugins

Plugins implement specific traits for their opportunities. The basics:

```rust
use perseus::plugins::{Plugin, PluginAction, PluginEnv};

pub struct MyPlugin;

impl MyPlugin {
    pub fn new() -> Self {
        Self
    }
}

// Implement specific plugin traits based on what opportunities you need
```

For detailed plugin development, see:
- [Plugin API Documentation](https://docs.rs/perseus/latest/perseus/plugins/)
- [Plugin Example](https://github.com/framesurge/perseus/tree/main/examples/core/plugins)

## Plugin Opportunities

Plugins can hook into various points:

| Opportunity | Type | Purpose |
|-------------|------|---------|
| Static aliases | Functional | Add static file mappings |
| HTML injection | Functional | Inject HTML into pages |
| Build process | Functional | Modify build behavior |
| Index view | Control | Replace the HTML shell |
| Tinker | Functional | Run custom commands |

The number of opportunities will grow in future releases.

## Plugin Registry

Community plugins are listed at [framesurge.sh/perseus/plugins](https://framesurge.sh/perseus/plugins).

- ✓ Endorsed plugins have undergone code review
- Endorsement doesn't guarantee security
- Always audit dependencies you install

## Security Considerations

Plugins execute arbitrary code during build and at runtime:

1. **Only install trusted plugins** - Audit code or trust the author
2. **Check for updates** - Security issues may be patched
3. **Limit permissions** - Don't give plugins unnecessary access
4. **Report issues** - Contact the [Perseus maintainer](mailto:arctic.hen@pm.me) for rogue plugins

Perseus cannot be held responsible for third-party plugin behavior.

## When to Use Plugins

| Need | Solution |
|------|----------|
| API routes | Custom server |
| Middleware | Custom server |
| Build-time code generation | Plugin or build.rs |
| Modified Perseus behavior | Plugin |
| Shared functionality across apps | Plugin |

## Related

- [Serving and Exporting](/docs/fundamentals/serving-exporting)
- [PerseusApp Configuration](/docs/fundamentals/perseus-app)
