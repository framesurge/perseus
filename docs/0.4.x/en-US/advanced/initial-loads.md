# Initial Loads

Perseus handles _initial loads_ very differently from _subsequent loads_. The former refers to what's done when a user visits a page on a Perseus app from an external source (e.g. visiting from a search engine, redirected from another site), and this requires a full HTMl page to be sent that can be interpreted by the browser. By contrast, subsequent loads are loads between pages within the same Perseus app, which can be performed by the app shell (described in the next section).

The process of initial loads is slightly complex, and occurs like so (this example is for a page called `/posts/test`, rendered with incremental generation):

1. Browser requests `/posts/test` from the server.
2. Server matches requested URL to wildcard (`*`) and handles it with the server-side inferred router, determining which `Template<G>` to use.
3. Server calls internal core methods to render the page (using incremental generation strategy, but it doesn't need to know that), producing an HTML snippet and a set of JSON properties.
4. Server calls `template.render_head_str()` and injects the result into the document's `<head>` (avoiding `<title>` flashes and improving SEO) after a delimiter comment that separates it from the metadata on every page (which is hardcoded into `index.html`).
5. Server interpolates JSON state into `index.html` as a global variable in a `<script>`.
6. Server interpolates HTML snippet directly into the user's `index.html` file.
7. Server sends final HTML package to client, including Wasm (injected at build-time).
8. Browser renders HTML package, user sees content immediately.
9. Browser invokes Wasm, hands control to the app shell.
10. App shell checks if initial state declaration global variable is present, finds that it is and unsets it (so that it doesn't interfere with subsequent loads).
11. App shell moves server-rendered content out of `__perseus_content_initial` and into `__perseus_content_rx`, which Sycamore's router had control over (allowing it to catch links and use the subsequent loads system).
12. App shell gets a translator if the app uses i18n.
13. App shell hydrates content at `__perseus_content_rx` with Sycamore and returns, the page is now interactive and has a translator context.

Note: if this app had used i18n, the server would've returned the app shell with no content, and the app shell, when invoked, would've immediately redirected the user to their preferred locale (or the closest equivalent).

The two files integral to this process are [`initial_load.rs`](https://github.com/arctic-hen7/perseus/blob/main/packages/perseus-actix-web/src/initial_load.rs) and [`shell.rs`](https://github.com/arctic-hen7/perseus/blob/main/packages/perseus/src/shell.rs).
