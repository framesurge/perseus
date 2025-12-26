# Understanding State

State is at the core of Perseus. This section explains how state works and when to use different generation strategies.

## What is State?

Think of a **template** as a stencil with holes. **State** is the data that fills those holes.

For a blog post template, the state might include:
- Title
- Author
- Content
- Tags

When you combine template + state, you get a **page**.

## State Lifecycle

State starts on the engine-side (server) where it's generated, then travels to the client-side (browser) where it becomes reactive.

```
Engine-side:          Client-side:
┌─────────────┐       ┌─────────────┐
│ Generate    │──────>│ Deserialize │
│ State       │       │ State       │
└─────────────┘       └──────┬──────┘
                             │
                      ┌──────▼──────┐
                      │ Make        │
                      │ Reactive    │
                      └──────┬──────┘
                             │
                      ┌──────▼──────┐
                      │ Store in    │
                      │ Page State  │
                      │ Store (PSS) │
                      └─────────────┘
```

## When is State Generated?

State can be generated in three places:

| Method | When | Use Case |
|--------|------|----------|
| **Build-time** | During `perseus build` | Static content, blog posts, docs |
| **Request-time** | On each HTTP request | User-specific data, auth |
| **Client-side** | In the browser | Suspended/lazy loading |

### Build-time State

Generated without knowledge of who's viewing the page. Perfect for:
- Blog posts from Markdown files
- Product catalogs
- Documentation

### Request-time State

Generated for each request, with access to cookies, headers, etc. Use for:
- Personalized dashboards
- Authenticated content
- Real-time data

### Suspended State

Generated client-side. Useful when:
- Part of the page loads slower
- You want to show the rest of the page first
- Data depends on client-side conditions

## Templates and Pages

A single template can produce many pages through **build paths**:

```rust
// Template: "post"
// Build paths: ["hello-world", "rust-tips", "web-dev"]
//
// Results in pages:
// - /post/hello-world
// - /post/rust-tips
// - /post/web-dev
```

Without explicit build paths, a template produces one page at its own path (e.g., `about` template → `/about` page).

## Reactive State

When state reaches the browser, Perseus makes it **reactive**:

```rust
// Your state definition
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CounterStateRx")]
struct CounterState {
    count: i32,
}

// What ReactiveState creates (conceptually)
struct CounterStateRx {
    count: Signal<i32>,
}
```

This means:
- Call `.get()` to read values
- Call `.set()` to update values
- UI automatically updates when state changes

## State Storage

Perseus stores all page states in the **Page State Store (PSS)**:

- When you visit a page, its state is cached
- Return to a page? State is restored (including form inputs!)
- Default capacity: 25 pages

Configure with:
```rust
PerseusApp::new()
    .pss_max_size(50)  // Store up to 50 pages
```

## What if I Don't Need State?

You can use as much or as little of the state platform as you need:

| Scenario | Recommendation |
|----------|---------------|
| Static site | Build-time state only |
| No dynamic data | Unreactive state |
| Purely static pages | Template without state |
| Not using Perseus features | Consider plain Sycamore |

## Quick Reference

| State Type | When to Use |
|------------|-------------|
| Build-time | Content known at build time |
| Request-time | Content depends on the request |
| Revalidation | Build-time content that needs refreshing |
| Incremental | Many possible pages, generate on demand |
| Suspended | Heavy components that can load later |
| Global | Shared across all pages |

## Next Steps

- [Build-time State](/docs/state/build) - The most common approach
- [Request-time State](/docs/state/request) - Per-request generation
- [Using State](/docs/state/browser) - Working with reactive state in views
