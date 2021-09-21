# Subsequent Loads

if the user follows a link inside a Perseus app to another page within that same app, the Sycamore router will catch it and prevent the browser from requesting the new file from the server. The following will then occur (for an `/about` page rendered simply):

1. Sycamore router calls Perseus inferred router logic.
2. Perseus inferred router determines from new URL that template `about` should be used, returns to Sycamore router.
3. Sycamore router passes that to closure in `perseus-cli-builder` shell, which executes core app shell.
4. App shell checks if an initial load declaration global variable is present and finds none, hence it will proceed with the subsequent load system.
5. App shell fetches page data from `/.perseus/page/<locale>/about?template_name=about` (if the app isn't using i18n, `<locale>` will verbatim be `xx-XX`).
6. Server checks to ensure that locale is supported.
7. Server renders page using internal systems (in this case that will just return the static HTML file from `.perseus/dist/static/`).
8. Server returns JSON of HTML snippet (not complete file) and stringified properties.
9. App shell deserializes page data into state and HTML snippet.
10. App shell interpolates HTML snippet directly into `__perseus_content_rx` (which Sycamore router controls), user can now see new page.
11. App shell initializes translator if the app is using i18n.
12. App shell renders new `<head>` and updates it (needs the translator to do this).
13. App shell hydrates content at `__perseus_content_rx`, page is now interactive.

The two files integral to this process are [`page_data.rs`](https://github.com/arctic-hen7/perseus/blob/main/packages/perseus-actix-web/src/page_data.rs) and [`shell.rs`](https://github.com/arctic-hen7/perseus/blob/main/packages/perseus/src/shell.rs).

Note that this process still has one improvement to be made before v0.2.0: rendering the document head on the server and interpolating that simultaneously before the translator is fetched on the client side. This further eliminates the need for rendering the `<head>` on the client at all. The tracking issue is [here](https://github.com/arctic-hen7/perseus/issues/15).
