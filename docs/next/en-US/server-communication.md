# Communicating with a Server

So far, we've described how to use Perseus to build powerful and performant frontend apps, but we've mostly left out the backend. If you want to fetch data from a database, authenticate users, perform server-side calculations or the like, you'll almost certainly want a backend of some kind.

<details>
<summary>Frontend? Backend?</summary>

In web development, we typically refer to a project as having a *frontend*, which is the thing users see (i.e. your web app, with all its styling and the like), and a *backend*, which is a server or serverless function (see below) that performs server-side work. A classic example would be a server that communicates with a database to fetch some data, but it needs to authenticate against the database. If you're new to web dev, you might well be thinking we could just query the database from the web app, but that would mean we'd have to store the access token in our frontend code, which can be easily inspected by the user (albeit less easily with Wasm, but still definitely doable). For that reason, we communicate with a server and ask it to get the data from the database for us.

Of course, a much simpler way of doing the above would be to make the database not need authentication in the first place, but the point stands.

</details>

## The Perseus Server

Perseus has an inbuilt server that serves your app and its data, and this can be extended by your own code. However, this requires [ejecting](/docs/ejecting), which can be brittle, because you'll have to redo everything every time there's a major update.

## Your Own Server

Instead, it's recommended that you create a server separate from Perseus that you control completely. You might do this with [Actix Web](https://actix.rs) or similar software.

### Serverless Functions

In the last few years, a new technology has sprung up that allows you to run individual *functions* (which might be single API routes) whenever they're requested. Infinitely many functions can be spawned simultaneously, and none will be active if none are being requested, which significantly reduces costs and increases scalability. This is provided by services like [AWS Lambda](https://aws.amazon.com/lambda/), and can be used with Rust via [this library](https://docs.rs/netlify_lambda_http). 

## Querying a Server

### At Build-Time

It's fairly trivial to communicate with a server at build-time in Perseus, which allows you to fetch data when you build your app, and then your users don't have to do as much work. You can also use other strategies to fetch data [at request-time](:strategies/request-state) if needed. Right now, it's best to use a blocking API to make requests on the server, which you can do with libraries like [`ureq`](https://docs.rs/ureq).

One of the problems with fetching data at build time (or request-time, etc.) in development is that it often adds a significant delay to building your project, which slows you down. To solve this, Perseus provides two functions `cache_res` and `cache_fallible_res` that can be used to wrap your request code. They'll run it the first time, and then they'll cache the result to `.perseus/cache/`, which will be used in all future requests. These functions take a name (for the cache file), the function to run (which must be async), and a boolean that can be set to `true` if you want to temporarily disable caching. Usefully, these functions don't have to be removed for production, because they'll automatically avoid caching there. You can find an example of using these in [this example](https://github.com/arctic-hen7/perseus/tree/main/examples/fetching).

Incidentally, you can also use those functions to work in an offline environment, even if your app includes calls to external APIs at build time. As long as you've called your app's build process once so that Perseus can cache all the requests, it won't make any more network requests in development unless you tell it to explicitly or delete `.perseus/cache/`.

### In the Browser

In some cases, it's just not possible to fetch the data you need on the server, and the client needs to fetch it themselves. This is often the case in [exported](:exporting) apps. This is typically done with the browser's inbuilt Fetch API, which is conveniently wrapped by [`reqwasm`](https://docs.rs/reqwasm). Note that you'll need to do this in a `Future`, which you can spawn using [`wasm_bindgen_futures::spawn_local`](https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/fn.spawn_local.html), conveniently re-exported from Perseus as `perseus::spawn_local`. You can then modify a `Signal` in there that holds the data you want. It's common practice in web development today to show a page skeleton (those pulsating bars instead of text) while data are still being loaded.

However, if you try to request from a public API in this way, you may run into problems with [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS), which can be very confusing, especially if you're not used to web development! The simple explanation of this is that CORS is a *thing* that browsers use to make sure your code can't send requests to servers that haven't allowed it (as in your code specifically). If you're querying your own server and getting this problem, make sure to set the `Access-Control-Allow-Origin` header to allow your site to make requests (see [here](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) for more details). However, if a public API hasn't set this, you're up the creek! In these cases, it's best to query through your own server or through one of Perseus' rendering strategies (if possible).

## Example

This can be confusing stuff, especially because it's different on the client and the server, so you may want to take a look at [this example](https://github.com/arctic-hen7/perseus/tree/main/examples/fetching) in the Perseus repo, which gets the IP address of the machine that built it, and then shows the user a message hosted with a [static alias](:static-content).
