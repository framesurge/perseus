# Heads and headers

 So far, there are two critical parts of a webpage that we've largely ignored: the document `<head>`, and the HTTP headers. The former is used to store static page metadata, such as the page title, while the latter can be used for all sorts of things, such as to control resource caching and manage cookies.
 
 Generally, in Perseus, you'll use the head much more than you'll use headers, simply because Perseus generally encourages an environment that is fairly siloed from the features of browsers themselves: for example, rather than setting a cookie through the `Set-Cookie` HTTP header, it's far more common in Perseus to just provide the value of the cookie through the state that's sent to the client, and then to set it there. What you choose to do is a matter of personal preference, but Perseus is generally built around state, not things like headers.
 
 *Note: for reading headers from the client to inform your state generation logic, see [request-time state generation](:state/request).*
 
## Setting the head

Heads work very much like views in Perseus: they're set on a template-by-template basis, and can take the state of the page, allowing them to specialize as necessary for pages within the templates on which they're set. 

Here's an example of setting the head without using any state:

```
{{#include ../../../../examples/core/basic/src/templates/about.rs}}
```

Note the use of `#[engine_only_fn]`, since Perseus will prerender the head of each page on the engine-side, as early as it can, and that will be transmitted as a static string to the client for further rendering. While this does mean you could theoretically do something like read from a file in your head function, this is not recommended, since the function is not `async`, and will block the rest of the build process or server, so you should prefer to do that sort of thing when generating state.

Note also the use of `SsrNode` in the return type, which reflects that this will *always* be prerendered to a string on the engine-side.

To access the state in the head, use `.head_with_state()` on `Template` instead of `.head()`, and have your function accept a second argument for your (unreactive) state type.

If you want to return an error from your head function for some reason, you can, and that will lead to the entire page failing. Generally, this is not desired. This behaves similarly to the state generation functions, which you can read more about [here](:state/build).

For information about setting a general *index view*, see [here](:fundamentals/perseus-app).

## Setting headers

When you need to set headers, you can do so with a function of the same form as the one you use to set page heads: it should be synchronous and take two arguments if you're accessing your state, or one if you're not.

Here's a more fully-fledged example that sets the custom `X-Greeting` header with the contents of some generated state:

```
{{#include ../../../examples/core/set_headers/src/templates/index.rs}}
```

Note the use of `.set_headers_with_state()` on `Template`, but this could also be `.set_headers()` if you didn't need access to your state type.

What is by far most important about this function is its return type, which comes from the [`http`](https://docs.rs/http/latest/http) crate, conveniently re-exported from Perseus on the engine-side. You'll need a return a [`HeaderMap`](=http/header/struct.HeaderMap@perseus), specifically, into which you can insert individiual headers, similarly to a `HashMap`.

Just like the head function, this can also return an error if you'd like it to, or it can be infallible, as it is here.
