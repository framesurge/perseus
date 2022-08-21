# Communicating with a Server

So far, we've described how to use Perseus to build powerful and performant frontend apps, but we've mostly left out the backend. If you want to fetch data from a database, authenticate users, perform server-side calculations or the like, you'll almost certainly want a backend of some kind.

<details>
<summary>Frontend? Backend?</summary>

In web development, we typically refer to a project as having a _frontend_, which is the thing users see (i.e. your web app, with all its styling and the like), and a _backend_, which is a server or serverless function (see below) that performs server-side work. A classic example would be a server that communicates with a database to fetch some data, but it needs to authenticate against the database. If you're new to web dev, you might well be thinking we could just query the database from the web app, but that would mean we'd have to store the access token in our frontend code, which can be easily inspected by the user (albeit less easily with Wasm, but still definitely doable). For that reason, we communicate with a server and ask it to get the data from the database for us.

Of course, a much simpler way of doing the above would be to make the database not need authentication in the first place, but the point stands.

</details>

Perseus has an inbuilt server that serves your app and its data, and this can be extended by your own code. However, this requires [ejecting](:reference/ejecting), which can be brittle, because you'll have to redo everything every time there's a major update. This is NOT the recommended approach for setting up your backend!

Instead, it's recommended that you create a server separate from Perseus that you control completely. You might do this with [Actix Web](https://actix.rs) or similar software. You could even set up serverless functions on a platform like [AWS Lambda](https://aws.amazon.com/lambda), which can reduce operation costs.

## Querying a Server

Querying a server in Perseus is fairly simple, though there are two different environments in which you'll want to do it, which are quite different from each other: on the server and in the browser. The main reason for this difference is because, in the browser, we're limited to the Web APIs, which are restricted by [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS), meaning the browser will ask APIs you query if they're expecting your app, which they won't be unless they've been configured to. For this reason, it's nearly always best to proxy requests to third-party APIs through your own server, which you can configure CORS on as necessary. In many cases, you can even perform third-party queries entirely at build-time and then pass through the results as state to pages.

Here's an example of both approaches (taken from [here](https://github.com/arctic-hen7/perseus/tree/main/examples/demos/fetching)):

```rust
{{#include ../../../examples/demos/fetching/src/templates/index.rs}}
```

### Build-Time

In the above example, we fetch the server's IP address at build-time from <https://api.ipify.org> using [`ureq`](https://docs.rs/ureq), a simple (and blocking) HTTP client. Note that Perseus gives you access to a full Tokio `1.x` runtime at build time, so you can easily use a non-blocking library like [`reqwest`](https://docs.rs/reqwest), which will be faster if you're making a lot of network requests at build-time. However, for simplicity's sake, this example uses `ureq`.

One problem of fetching data at build-time though, in any framework, is that you have to fetch it again every time you rebuild your app, which slows down the build process and thus slows down your development cycle. To alleviate this, Perseus provides two helper functions, `cache_res` and `cache_fallible_res` (used for functions that return a `Result`) that can be used to wrap any asynchronous code that runs on the server-side (e.g. at build-time, request-time, etc.). The first time they run, these will just run your code, but then they'll cache the result to a file in `.perseus/`, which can be used in all subsequent requests, making your long-running code (typically network request code, but you could even put machine learning stuff in them in theory...) run almost instantaneously. Of course, sometimes you'll need to re-run that asynchronous code if you change something, which yo ucan do trivially by changing the second argument from `false` to `true`, which will override the cache and always re-run the given code.

Incidentally, you can also use those functions to work in an offline environment, even if your app includes calls to external APIs at build time. As long as you've called your app's build process once so that Perseus can cache all the requests, it won't make any more network requests in development unless you tell it to explicitly or delete `.perseus/cache/`.

Note also that those functions don't have to be removed for production, they'll automatically be disabled.

### In the Browser

In the above example's `index_page()` function, we perform some request logic that we want to do in the browser. It's important to remember here that Perseus will run your template's code on the server as well when it prerenders (which happens more often than you may think!), so if we want to only run something in the browser, we have to check with `G::IS_BROWSER` (usefully provided by Sycamore). From there, the comments in the code should mostly explain what we're doing, but the broad idea is to spawn a `Future` in the browser (which we do with a function that Perseus re-exports from another library called [`wasm-bindgen-futures`](https://docs.rs/wasm-bindgen-futures)) that uses a library like [`reqwasm`](https://docs.rs/reqwasm) (a wrapper over the browser's Fetch API) to get some data. In this example, we fetch that data from some static content on the same site, which avoids issues with [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) (something you will very much want to understand, because it can generate some very confusing errors, especially for those new to web development).

As for what we do with the data we fetch, we just modify a Sycamore `Signal` to hold it, and our `view! {...}` will update accordingly!
