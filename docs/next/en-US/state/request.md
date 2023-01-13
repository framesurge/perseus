# Request-time state

Request-time state can be thought of as the next level up from [build-time state](:state/build), because it provides access to everything build state does, plus the actual HTTP request of the user who made the query. This allows you to generate user-specific state: for instance a custom dashboard could be prerendered for them based on their settings.

Here's an example of request state being used to show a user their own IP address (as their browser reports it, which can be spoofed):

```
{{#include ../../../examples/core/state_generation/src/templates/request_state.rs}}
```

Just as with a `get_build_state` function, this `get_request_state` function is `async` and returns a `BlamedError<E>`, where `E` is any compliant error type, but it could return the `PageState` directly if it wanted to. If you need a refresher on these properties, especially on error handling in these kinds of functions, see [this page](:state/build).

Importantly, request-time state gives us access to the usual [`StateGeneratorInfo`](=prelude/struct.StateGeneratorInfo@perseus), which contains the path, locale, and [helper state](:state/helper), plus the [`Request`](:prelude/type.Request@perseus), which is an HTTP request with the body stripped out. This is done because the body is meaningless in Perseus requests, and anything that needs it shoudl use [custom API endpoints].

In this example, we're using `req` to access the headers of the request, which are the main things you'll use (since these contain cookies, etc.). Here, we're just accessing the non-standard `X-Forwarded-For` header, which represents the IP address of the client in many cases.

Critically, the request provided to this function is **not** the 'real' request, meaning altering parts of it will have absolutely no effect whatsoever --- it's just a representation of it provided to your functions so they can access user details. For example, if you wanted to set headers, you should not add them here, but [do this] instead.

A request-time state generating function can be specified using `.request_state_fn()` on `Template`.
