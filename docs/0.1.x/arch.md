# Architecture

Perseus is a complex system, and this page will aim to explain the basics in a beginner-friendly way. If you've already used similar frameworks from the JS world like NextJS, then some of this may be familiar to you. If you're having trouble following along, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose) and ask us to clarify some sections, because this page in particular should be accessible to everyone. If you'd like more specific help, [ask on Gitter](TODO)!

## Templates and Pages

The core of Perseus is the idea of templates. When you create a Perseus app, what you're basically doing is telling Perseus how to compile your code into a series of *pages*. **Each page has a unique URL on your final website.** If you have a blog, and every post is stored as something like `post/title`, then each post would be a unique page.

But this doesn't mean you have to write the code for every page individually! Perseus does this for you, and only asks you to write *templates*. A template can generate one page or many, and a great example of one would be a `post` template. Each template has a *root path*, which is essentially like the space on your website that that template controls. For example, a post template might control `/post`, meaning it can render pages at `/post`, `/post/test`, `/post/longer/path`, etc. In theory, a template could render pages outside its domain, but this would be a bad idea for structure, and makes your code difficult to understand.

### State

What differentiates pages from templates is *state*, which tells a page how to fill out its template to give unique content. For example, our post template would probably have a `content` field in its state, and its pages would use that to render their unique content!

In terms of writing code, a page's state is just a `struct` that can be serialized and deserialized with [Serde](https://serde.rs).

## Rendering Strategies

Each template has a rendering strategy, which it uses to create its pages. There are a number of rendering strategies in Perseus, each of which is documented in detail in its own section. What's important to understand for now is that there are two main ways a template can render pages, at *build time*, or at *request time*. If a template renders at build time, it generates the code for your pages when you build your app, which means you end up serving static pages. This is *really fast*. However, sometimes you need information specific to each request to render a page (e.g. an authentication token), and you can't render at build. Instead, you'd render at request time, which gives you access to information about the HTTP request a user sent for your page.

Here's a list of Perseus' currently supported rendering strategies. These can all be combined, but some combinations make more sense than others.

| Strategy               | Description                                | Type    |
| ---------------------- | ------------------------------------------ | ------- |
| Build paths            | Generates a series of pages for a template | Build   |
| Build state            | Generates page state                       | Build   |
| Request state          | Generates page state                       | Request |
| Revalidation           | Rebuilds pages conditionally               | Hybrid  |
| Incremental generation | Builds pages on-demand                     | Hybrid  |

There are two *hybrid* strategies listed above. They're a little more complicated, and out of the scope of this page, but they operate at both build *and* request-time, allowing you to reap the benefits of both worlds!

## Routing

*This section describes how Perseus works under the hood. Skip it if you want.*

Perseus doesn't just host your pages at their URLs though. In fact, Perseus has a generic handler for *any URL*, which returns what we call the *app shell*. That's a concept from the single-page app (e.g. ReactJS), where your app always has a constant shell around it, and each page is loaded into that shell, making page transitions more seamless. Perseus adopts this as well, but with the added benefits of super-fast static rendering strategies and a more lightweight shell.

The shell includes a router (courtesy of [Sycamore](https://github.com/sycamore-rs/sycamore)), which determines what page the user wants, and then sends a request to a special endpoint behind `/.perseus`. That then renders the page and returns some static HTML and the page's state.
