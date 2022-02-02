# Request State

While build-time strategies fulfill many use-cases, there are also scenarios in which you may need access to information only available at request-time, like an authentication key that the client sends over HTTP as a cookie. For these cases, Perseus supports the _request state_ strategy, which is akin to traditional server-side rendering, whereby you render the page when a client requests it.

If you can avoid this strategy, do, because it will bring your app's TTFB (time to first byte) down, remember that anything done in this strategy is done on the server while the client is waiting for a page.

## Usage

Here's an example taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/core/state_generation/src/templates/request_state.rs) of using this strategy to tell the user their own IP address (albeit not hugely reliably as this header can be trivially spoofed, but this is for demonstration purposes):

```rust
{{#include ../../../../examples/core/state_generation/src/templates/request_state.rs}}
```

Note that, just like _build state_, this strategy generates stringified properties that will be passed to the page to render it (serialization is handled by `#[perseus::autoserde(request_state)]`), and it also uses `RenderFnWithCause` (see the section on [build state](:reference/strategies/build-state) for more information). The key difference though is that this strategy receives a second, very powerful parameter: the HTTP request that the user sent (`perseus::Request`).

<details>
<summary>How do you get the user's request information?</summary>

The web frameworks Perseus supports automatically pass this information to handlers like Perseus. The slightly difficult thing is then converting this from their custom format to Perseus' (which is just an alias for the [`http`](https://docs.rs/http) module's). This is done in the appropriate integration crate.

</details>

That parameter is actually just an alias for [this](https://docs.rs/http/0.2/http/request/struct.Request.html), which gives you access to all manner of things in the user's HTTP request. The main one we're concerned with in this example though is `X-Forwarded-For`, which contains the user's IP address (unless it's trivially spoofed). Because we can't assume that any HTTP header exists, we fall back to a message saying the IP address is hidden if we can't access the header.

The other notable thing in the above example is the commented-out line at the beginning of `get_request_state`, which shows you how to return an error if the client didn't provide something that they should've.
