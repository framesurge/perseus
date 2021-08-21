# Request State

This strategy allows you to define the state for a page at request-time, which gives you access to information like the headers of the user's HTTP request (including any authorization tokens) and real-time factors. This can be useful if you want to render something like a user dashboard on the server, which you wouldn't be able to do with the *build state* strategy, as it only has access to information at build-time.

It should be noted that this strategy is much slower than build-time strategies, as it requires extra computations on the server every time a user requests a page. However, this strategy is superior to client-side rendering (rendering a page at build time with fillers for unique content that you then fill in in the browser) in some ways. This strategy is essentially server-side rendering, and you can read more about its performance [here](https://medium.com/walmartglobaltech/the-benefits-of-server-side-rendering-over-client-side-rendering-5d07ff2cefe8).

## Using with Build State

Perseus supports using both build and request state simultaneously, though it's not advised unless absolutely necessary. This will result in the generation of two competing states, one from build and one from request, which you can then amalgamate by using the `amalgamate_states` strategy. Due to the phenomenally niche nature of this approach, it's not covered in depth in the documentation, but you can check out the `showcase` example if you want to see it in action (specifically the `amalgamate` page).

## Usage

You can define a function for this strategy like so (this will tell the user their own IP address):

```rust
use serde::{Deserialize, Serialize};
use perseus::ErrorCause;

#[derive(Serialize, Deserialize)]
pub struct IpPageProps {
    ip: String,
}
pub async fn get_request_state(_path: String, req: Request) -> Result<String, (String, ErrorCause)> {
    Ok(serde_json::to_string(&IpPageProps {
        // Gets the client's IP address
        ip: format!(
            "{:?}",
            req
                .headers()
                .get("X-Forwarded-For")
                .unwrap_or(&perseus::http::HeaderValue::from_str("hidden from view!").unwrap())
        ),
    })
    .unwrap())
}
```

This function can produce two kinds of errors, broadly: those caused by the server, and those caused by the client. For that reason, you need to return a `(String, ErrorCause)` tuple, the second part of which specifies who's responsible for the error. This allows Perseus to figure out whether it should send a 400 (client error) or 500 (server error) HTTP status code in the event of an error. This function must also be asynchronous.

As with the *build state* strategy, you must return state from this function as a string, and the path provided to this function is the same as the final path at which the page will be rendered.

You can add this strategy to a template like so:

```rust,no_run,no_playground
template
	// ...
    .request_state_fn(Box::new(get_request_state))
```
