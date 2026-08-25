# `PerseusApp`

`PerseusApp` is the central interface between Perseus internals and your code. You use it to define templates, capsules, error views, and configure your app.

## Basic Usage

```rust
use perseus::prelude::*;

mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
}
```

## Template/Capsule Definition Patterns

### Functional Definition Pattern

Used primarily with templates:

```rust
// In templates/index.rs
pub fn get_template() -> Template {
    Template::build("index")
        .view(index_page)
        .build()
}

// In main.rs
.template(crate::templates::index::get_template())
```

### Referential Definition Pattern

Used primarily with capsules (which need to be accessed from multiple places):

```rust
// In capsules/greeting.rs
lazy_static::lazy_static! {
    pub static ref GREETING: Capsule<PerseusNodeType, ()> = {
        Capsule::build(Template::build("greeting"))
            .empty_fallback()
            .view(greeting_widget)
            .build()
    };
}

// In main.rs
.capsule_ref(&*crate::capsules::greeting::GREETING)
```

## Index Views

Customize the HTML shell that wraps your app:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
        .index_view(|| {
            view! {
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        link(rel = "stylesheet", href = "/.perseus/static/styles.css")
                    }
                    body {
                        PerseusRoot()  // Your app renders here
                    }
                }
            }
        })
}
```

**Note**: Use `PerseusRoot()` component to mark where your app renders. Nothing in the index view can be reactive.

For raw HTML strings, use `.index_view_str()` instead.

## Mutable Stores

Perseus writes data to two directories in `dist/`:

| Directory | Purpose | Mutability |
|-----------|---------|------------|
| `static/` | Prerendered HTML, static pages | Immutable |
| `mutable/` | Pages that can change (revalidation) | Mutable |

By default, `FsMutableStore` writes to the filesystem. For serverless environments (which have immutable filesystems), you'd need a custom `MutableStore` implementation using a database.

## Translations Management

For internationalized apps, `FsTranslationsManager` is the default, expecting translations in `translations/`:

```
my-app/
├── translations/
│   ├── en-US.ftl
│   └── es-ES.ftl
```

## Page State Store (PSS)

Configure how many pages Perseus caches in memory:

```rust
PerseusApp::new()
    .pss_max_size(50)  // Cache up to 50 pages (default: 25)
```

**Guidelines:**
- Higher values = more instant back-navigation, more RAM usage
- Lower values = less RAM, but slower navigation to old pages
- Large state apps (like documentation) should use lower values
- Capsules are cached separately until their parent pages are evicted

## Common Methods

| Method | Purpose |
|--------|---------|
| `.template(t)` | Add a template |
| `.capsule_ref(&c)` | Add a capsule by reference |
| `.error_views(e)` | Set error handling views |
| `.index_view(f)` | Customize HTML shell |
| `.global_state_creator(g)` | Set up global state |
| `.pss_max_size(n)` | Set page cache size |
| `.locales_and_translations_manager(...)` | Enable i18n |
| `.static_alias(path, file)` | Serve static files |

## Full API

See the [PerseusAppBase API docs](https://docs.rs/perseus/latest/perseus/struct.PerseusAppBase.html) for all available methods.
