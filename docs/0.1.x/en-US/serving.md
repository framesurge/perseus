# Serving

_You only need this page if you're not using the Perseus CLI, which performs this process for you!_

Having generated a large number of static files, you'll need a system to host your files for you! Due to the dynamic nature of some rendering strategies, Perseus needs to be involved in this process (for executing request-time logic), and so it provides a simple API interface for serving pages.

Perseus aims to be agnostic as to what framework you use to host your files, and any framework that gives you access to request headers and wildcard paths should work (in other words, any framework worth its salt).

If you're using one of our supported integrations, you don't have to bother with this page, nearly all of it can be done for you!

-   [Actix Web](./integrations/actix-web.md)
-   _More coming soon..._

## Endpoints

Here are the endpoints that a server for Perseus must serve:

-   `/.perseus/page/*` – used to serve the JSON data that the app shell needs to render a page (`*` should be extractable as a filename, e.g. `{filename:.*}` in Actix Web)
-   `/.perseus/bundle.js` – the JavaScript bundle file that calls your Wasm code (see [tutorial on building your first app](./tutorials/first_app/intro.md))
-   `/.perseus/bundle.wasm` – the Wasm bundle file that contains your code (see [tutorial on building your first app](./tutorials/first_app/intro.md))
-   `*` (anything else) – any page that the user actually requests, which will return the app shell to do the heavy lifting (or more accurately an HTML file that includes the bundle)

## Usage

This example shows what would be done to acquire a page for any framework. You'll need to have access to these data to get a page:

-   The page path the user requested, e.g. `/post/test` for a request to `/.perseus/page/post/test`
-   Data about the HTTP request the user sent (see below)
-   A map of templates produced with [`get_templates_map!`]() (API docs WIP)
-   A [config manager](./config_managers.md)

```rust,no_run,no_playground
use perseus::{get_page};

// See below for details on this line
let http_req = convert_req(&req).unwrap();

let page_data = get_page(path, http_req, &render_cfg, &templates, config_manager.get_ref()).await;

match page_data {
    Ok(page_data) => // Return a 200 with the stringified `page_data`
    // We parse the error to return an appropriate status code
    Err(err) => // Return the error dictated by `err_to_status_code(&err)` with the body of the stringified `err`
}
```

## Request Data

Perseus needs access to information about HTTP requests so it can perform tasks related to the _request state_ strategy, which provides access to headers and the like. Internally, Perseus uses [`http::Request`](https://docs.rs/http/0.2.4/http/request/struct.Request.html) for this, with the body type `()` (payloads are irrelevant in requests that ask for a page at a URL).

Unfortunately, different web server frameworks represent request data differently, and so you'll need to convert from your framework's system to `http`'s. When integrations are ready, this will be done for you!

### Writing a Converter

This is a simplified version of an Actix Web converter:

```rust
use perseus::{HttpRequest, Request};

/// Converts an Actix Web request into an `http::request`.
pub fn convert_req(raw: &actix_web::HttpRequest) -> Result<Request, String> {
	let mut builder = HttpRequest::builder();
	// Add headers one by one
	for (name, val) in raw.headers() {
		// Each method call consumes and returns `self`, so we re-self-assign
		builder = builder.header(name, val);
	}
	// The URI to which the request was sent
	builder = builder.uri(raw.uri());
	// The method (e.g. GET, POST, etc.)
	builder = builder.method(raw.method());
	// The HTTP version used
	builder = builder.version(raw.version());

	let req = builder
		// We always use an empty body because, in a Perseus request, only the URI matters
		// Any custom data should therefore be sent in headers (if you're doing that, consider a dedicated API)
		.body(())
		.map_err(|err| format!("converting actix web request to perseus-compliant request failed: '{}'", err))?;

	Ok(req)
}
```

Notably, the data that need to be converted are:

-   Headers
-   URI to which the request was sent
-   HTTP method (subject to change in future Perseus versions, currently `GET`)
-   HTTP version used

Note that mis-converting any of these data will not affect Perseus (which doesn't require any of them to function), only your own code. So if you have no intention of using the _request state_ strategy in your app, you could theoretically just parse an empty request to Perseus like so:

```rust
use perseus::HttpRequest

HttpRequest::new(());
```

## File Storage

Perseus' systems of storing files in production are documented in-depth [here](./config_managers.md).
