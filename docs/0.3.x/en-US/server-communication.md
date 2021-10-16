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

You have a few options if you want to query a server from client-side code. You can use an high-level module, like [reqwest](https://docs.rs/reqwest) (which supports Wasm), or you can use the underlying browser Fetch API directly (which entails turning JavaScript types into Rust types). We recommend the first approach, but an example of the second can be found in the Perseus code [here](https://github.com/arctic-hen7/perseus/blob/61dac01b838df23cc0f33b0d65fcb7bf5f252770/packages/perseus/src/shell.rs#L19-L65).
