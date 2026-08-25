# Capsules

Capsules are one of Perseus' most powerful features for building efficient, cacheable apps.

## What are Capsules?

Remember how **template + state = page**? Similarly, **capsule + state = widget**.

Widgets are like mini-pages that can be embedded within other pages. They:
- Don't have their own `<head>`
- Are embedded inside pages
- Have full access to Perseus' state platform
- Are aggressively cached for performance

## Why Use Capsules?

When Perseus loads a new page, it won't re-request capsules it already has. This means apps using capsules can dramatically reduce network traffic!

### Example: E-commerce Product Carousel

Imagine a product page with a "Similar Products" carousel:

1. Create a `product` capsule that renders product cards
2. The main product is one widget, carousel items are additional widgets
3. If a user clicks a carousel item, the page loads **instantly** (it's already cached!)

## Capsules vs Templates

| Feature | Templates | Capsules |
|---------|-----------|----------|
| Produces | Pages | Widgets |
| Has `<head>` | Yes | No |
| Takes full page | Yes | No |
| Can be cached | Yes | Aggressively |
| Accepts properties | No | Yes |

## Initial Load vs Subsequent Load

**Initial Load** (user comes from external site):
- Page + all capsules sent as one HTML file
- States included for everything

**Subsequent Load** (navigating within your app):
- Only new page state sent
- Widgets loaded separately (in parallel)
- Fallback view shown while loading

## Real-World Use Cases

### Blog Series Sidebar

```
Page: /post/chapter-1
├── Main content (page state)
└── Series widget showing all chapters (capsule)

When user visits /post/chapter-2:
├── Main content (new page state - fetched)
└── Series widget (same capsule - already cached!)
```

### Search Suggestions

Capsules in search suggestions implicitly preload content. If the user searches for "rust tips" and hovers over suggestions, those widgets cache in the background.

### Dynamic Dashboards

Dashboard panels as widgets:
- Rearrangeable by users
- Each panel independently cached
- Failed panel shows fallback, others still work

## Key Concepts

1. **Properties**: Static data passed to widgets by the caller
2. **Fallback Views**: Shown while widget is loading
3. **Referential Definition**: Use `lazy_static` for capsule references
4. **Rescheduling**: When build-time pages use request-time widgets

## Performance Benefits

- Cached widgets load instantly on subsequent navigations
- Parallel widget loading for faster perceived performance
- Reduced data transfer (no duplicate widget data)
- Incremental rendering keeps pages responsive

## Next Steps

- [Using Capsules](/docs/capsules/using) - Create and embed widgets
- [Capsules vs Components](/docs/capsules/capsules-vs-components) - When to use each
