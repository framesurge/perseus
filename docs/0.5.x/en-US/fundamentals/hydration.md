# Hydration

Perseus prerenders your pages to HTML on the server, ensuring users see content immediately. But static HTML can't handle button clicks or form submissions. *Hydration* bridges this gap by attaching event handlers to prerendered content.

## Initial vs Subsequent Loads

Perseus handles page loads differently depending on context:

### Initial Load

When a user first visits your site (e.g., from a search engine):

1. Server sends the **app shell**: `bundle.js`, `bundle.wasm`
2. Server also sends **prerendered HTML** of the requested page
3. User sees content *immediately* (no blank page)
4. Wasm bundle loads in the background
5. Sycamore *hydrates* the page, attaching event handlers

### Subsequent Load

After the initial load, navigating to another page:

1. Perseus only fetches the **page state** (small JSON)
2. Rust renders the HTML client-side (faster than fetching HTML)
3. Page transitions feel instant
4. Previously visited pages are cached and restore *immediately*

## How Hydration Works

During server-side rendering, Sycamore inserts *hydration IDs* throughout the HTML. These markers allow the client to match DOM nodes with your Rust code:

```html
<!-- Simplified example of hydrated HTML -->
<button data-hk="0-0">Click me</button>
```

When the Wasm bundle loads, Sycamore:

1. Walks the existing DOM
2. Matches elements using hydration IDs
3. Attaches event handlers from your code
4. Makes the page fully interactive

## Why Hydration is Fast

Unlike JavaScript frameworks where hydration can take seconds, Rust/Wasm hydration is nearly instant:

| Factor | Benefit |
|--------|---------|
| Compiled code | No parsing or JIT compilation needed |
| Streaming Wasm | Browser executes as it downloads |
| Efficient diffing | Sycamore's hydration is optimized |
| Small runtime | No heavy framework overhead |

The limiting factor is download time, not execution. That's why `perseus deploy` aggressively optimizes Wasm for size.

## Hydration Errors

Sometimes hydration fails if the server-rendered HTML doesn't match what the client expects. Common causes:

| Cause | Solution |
|-------|----------|
| Random values | Use seeded random or fetch client-side |
| Timestamps | Use consistent time or fetch client-side |
| Browser-only APIs | Guard with `#[cfg(target_arch = "wasm32")]` |
| Different data | Ensure state is serialized correctly |

When hydration fails, Perseus shows an error popup (preserving the readable content) rather than replacing the page with an error message.

## Optimizing for Hydration

### Keep Initial State Minimal

Large state means larger HTML and longer hydration:

```rust
// Good: Minimal initial state
#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PostStateRx")]
struct PostState {
    title: String,
    excerpt: String,
    // Full content loaded on demand
}

// Avoid: Everything upfront
struct PostState {
    title: String,
    content: String,        // Could be huge
    comments: Vec<Comment>, // Fetch these client-side
}
```

### Use Suspended State for Heavy Data

Fetch non-critical data after hydration:

```rust
#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    title: String,
    #[rx(suspense = "comments_handler")]
    comments: Result<Vec<Comment>, SerdeInfallible>,
}
```

### Delay Non-Essential Widgets

Use `delayed_widget` for heavy capsules:

```rust
// Loads after page is interactive
(HEAVY_CAPSULE.delayed_widget("", ()))
```

## The Popup Error System

If hydration fails but the server rendered content successfully, Perseus shows errors as popups rather than replacing the content. This is better UX because:

1. Users can still *read* the content
2. Static functionality (links, etc.) may still work
3. The error indicates limited interactivity, not total failure

Style the popup with:

```css
#__perseus_popup_error {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    background: #fef2f2;
    border: 1px solid #fca5a5;
    padding: 1rem;
    border-radius: 0.5rem;
    z-index: 9999;
}
```

## Related

- [Error Views](/docs/fundamentals/error-views)
- [Suspended State](/docs/state/browser)
- [Debugging](/docs/fundamentals/debugging)
