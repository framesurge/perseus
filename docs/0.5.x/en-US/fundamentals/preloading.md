# Preloading

Perseus caches visited pages for instant back-navigation. Preloading extends this: if you know where a user will likely go next, load it in advance for an instant transition.

## Why Preload?

When navigating to a new page, Perseus needs:
- The page's state (JSON)
- The page's head metadata

Without preloading, there's a brief loading state. With preloading, the page appears *instantly*.

## Basic Preloading

Use the Reactor's `.preload()` method:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn nav() -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    view! {
        // Preload on hover
        div(on:mouseenter = {
            let reactor = reactor.clone();
            move |_| {
                reactor.preload("/about");
            }
        }) {
            Link(to = "/about") { "About Us" }
        }
    }
}
```

The `.preload()` method:
- Runs asynchronously (doesn't block the main thread)
- Silently fails on server errors
- Panics on programmer errors (misspelled routes)

## Fine-Grained Control

For custom error handling, use `.try_preload()`:

```rust
use perseus::prelude::*;
use sycamore::prelude::*;

fn dynamic_nav(path: String) -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    let preload_result = create_signal(None::<String>);

    view! {
        div(on:mouseenter = {
            let reactor = reactor.clone();
            let path = path.clone();
            move |_| {
                let reactor = reactor.clone();
                let path = path.clone();
                spawn_local(async move {
                    match reactor.try_preload(&path).await {
                        Ok(()) => preload_result.set(Some("Ready!".to_string())),
                        Err(e) => preload_result.set(Some(format!("Failed: {:?}", e))),
                    }
                });
            }
        }) {
            Link(to = path.clone()) { (path) }
        }
    }
}
```

## Common Preloading Patterns

### Preload on Hover

The most common pattern—preload when the user hovers over a link:

```rust
fn preloaded_link(path: &'static str, label: &'static str) -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    view! {
        div(on:mouseenter = {
            let reactor = reactor.clone();
            move |_| {
                reactor.preload(path);
            }
        }) {
            Link(to = path) { (label) }
        }
    }
}
```

### Preload on Page Load

For predicted navigation flows:

```rust
fn checkout_page() -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    // User will likely go to confirmation next
    reactor.preload("/checkout/confirmation");

    view! {
        h1 { "Checkout" }
        // ... checkout form
    }
}
```

### Preload Multiple Pages

```rust
fn dashboard() -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    // Preload common next pages
    reactor.preload("/dashboard/analytics");
    reactor.preload("/dashboard/settings");
    reactor.preload("/dashboard/reports");

    view! {
        h1 { "Dashboard" }
        // ... dashboard content
    }
}
```

## Preloading with i18n

When using internationalization, preloading works within the current locale only:

```rust
fn localized_nav() -> View {
    let reactor = Reactor::<BrowserNodeType>::from_cx();

    view! {
        // This preloads /en-US/about if you're on /en-US/
        div(on:mouseenter = {
            let reactor = reactor.clone();
            move |_| {
                reactor.preload(link!("/about"));
            }
        }) {
            Link(to = link!("/about")) { "About" }
        }
    }
}
```

**Important**: You cannot preload across locales. Each locale has its own translations, and Perseus only keeps one set in memory at a time.

## When to Preload

| Scenario | Recommendation |
|----------|----------------|
| Main navigation links | Preload on hover |
| Predictable user flows | Preload on page load |
| Search results | Don't preload (too many) |
| Paginated lists | Preload next page |
| Forms | Preload success page |

## Performance Considerations

- **Don't over-preload** - Each preload is a network request
- **Prioritize likely paths** - Focus on common user flows
- **Cache hits are free** - Already-visited pages don't re-fetch
- **Network conditions** - Preloading on slow connections may hurt UX

## Preloading vs Delayed Widgets

| Feature | Purpose |
|---------|---------|
| Preloading | Load full pages before navigation |
| Delayed widgets | Load widget content after page renders |

Use both together for optimal perceived performance.

## Related

- [Routing and Navigation](/docs/fundamentals/routing)
- [The Reactor](/docs/fundamentals/reactor)
- [Capsules](/docs/capsules/intro)
