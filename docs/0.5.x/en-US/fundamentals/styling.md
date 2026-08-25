# Styling

Perseus works with any CSS approach. This guide covers common styling patterns and recommendations.

## CSS Integration

### Static Stylesheets

Place CSS files in `static/` and link them in your head:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

#[engine_only_fn]
fn head() -> View {
    view! {
        title { "My App" }
        link(rel = "stylesheet", href = ".perseus/static/styles.css")
    }
}
```

Or in your index view for global styles:

```rust
PerseusApp::new()
    .index_view(|| {
        view! {
            html {
                head {
                    link(rel = "stylesheet", href = ".perseus/static/global.css")
                }
                body {
                    PerseusRoot()
                }
            }
        }
    })
```

### Inline Styles

Apply styles directly to elements:

```rust
fn styled_component() -> View {
    view! {
        div(style = "padding: 1rem; background: #f0f0f0;") {
            p(style = "color: #333; font-size: 1.2rem;") {
                "Styled text"
            }
        }
    }
}
```

### Dynamic Styles

Use signals for reactive styling:

```rust
fn dynamic_theme() -> View {
    let is_dark = create_signal(false);

    view! {
        div(
            style = move || if is_dark.get() {
                "background: #1a1a1a; color: white;"
            } else {
                "background: white; color: black;"
            }
        ) {
            button(on:click = move |_| is_dark.set(!is_dark.get())) {
                "Toggle Theme"
            }
        }
    }
}
```

## Tailwind CSS

[Tailwind](https://tailwindcss.com) works excellently with Perseus:

```rust
fn card() -> View {
    view! {
        div(class = "bg-white rounded-lg shadow-md p-6 dark:bg-gray-800") {
            h2(class = "text-xl font-bold text-gray-900 dark:text-white") {
                "Card Title"
            }
            p(class = "mt-2 text-gray-600 dark:text-gray-300") {
                "Card content goes here."
            }
        }
    }
}
```

### Setting Up Tailwind

1. Install Tailwind:
```bash
npm init -y
npm install -D tailwindcss
npx tailwindcss init
```

2. Configure `tailwind.config.js`:
```javascript
module.exports = {
  content: ["./src/**/*.rs"],
  theme: { extend: {} },
  plugins: [],
}
```

3. Create `static/input.css`:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

4. Build CSS:
```bash
npx tailwindcss -i ./static/input.css -o ./static/styles.css --watch
```

5. Link in your app:
```rust
#[engine_only_fn]
fn head() -> View {
    view! {
        link(rel = "stylesheet", href = ".perseus/static/styles.css")
    }
}
```

## Full-Page Layouts

A common pattern for headers, content, and footers:

### CSS

```css
/* static/layout.css */
html, body {
    height: 100%;
    margin: 0;
}

.layout {
    min-height: 100%;
    display: flex;
    flex-direction: column;
}

.header {
    position: sticky;
    top: 0;
    background: white;
    border-bottom: 1px solid #e5e5e5;
    z-index: 100;
}

.content {
    flex: 1;
}

.footer {
    background: #f5f5f5;
    border-top: 1px solid #e5e5e5;
}
```

### Layout Component

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn layout(children: View) -> View {
    view! {
        div(class = "layout") {
            header(class = "header") {
                nav(class = "container mx-auto p-4") {
                    Link(to = "/") { "Home" }
                    Link(to = "/about") { "About" }
                }
            }

            main(class = "content") {
                (children)
            }

            footer(class = "footer") {
                div(class = "container mx-auto p-4") {
                    "© 2024 My App"
                }
            }
        }
    }
}
```

### Using the Layout

```rust
fn home_page() -> View {
    layout(view! {
        div(class = "container mx-auto p-4") {
            h1 { "Welcome Home" }
        }
    })
}
```

## Dark Mode

### CSS Variables Approach

```css
:root {
    --bg-primary: #ffffff;
    --text-primary: #1a1a1a;
}

[data-theme="dark"] {
    --bg-primary: #1a1a1a;
    --text-primary: #ffffff;
}

body {
    background: var(--bg-primary);
    color: var(--text-primary);
}
```

### Toggle Implementation

```rust
use perseus::prelude::*;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;

fn theme_toggle() -> View {
    view! {
        button(on:click = move |_| {
            let document = web_sys::window()
                .unwrap()
                .document()
                .unwrap();
            let html = document.document_element().unwrap();
            let current = html.get_attribute("data-theme")
                .unwrap_or_default();
            html.set_attribute(
                "data-theme",
                if current == "dark" { "light" } else { "dark" }
            ).unwrap();
        }) {
            "Toggle Theme"
        }
    }
}
```

## CSS-in-Rust (Future)

The [Jacaranda](https://github.com/framesurge/jacaranda) project aims to bring fully typed CSS-in-Rust styling to Sycamore/Perseus. Watch this space!

## Best Practices

1. **Organize by component** - Keep styles near their components
2. **Use utility classes** - Tailwind or similar speeds up development
3. **Minimize inline styles** - Use classes for reusable styles
4. **Consider bundle size** - Large CSS files slow initial load
5. **Test dark mode** - If supporting, test thoroughly
6. **Mobile first** - Start with mobile styles, add breakpoints

## Related

- [Static Content](/docs/fundamentals/static-content)
- [Heads and Headers](/docs/fundamentals/head-headers)
