# Templates

At the core of Perseus is its template system, which is how you'll define every page you'll ever build! However, it's important to understand a few of the nuances of this system so that you can build the best apps possible.

## Templates vs Pages

In Perseus, the idea of a _template_ is very different to the idea of a _page_.

A _page_ corresponds to a URL in your app, like the about page, the landing page, or an individual blog post.

A _template_ can generate _many_ pages or only one by using _rendering strategies_.

The best way to illustrate this is with the example of a simple blog, with each page stored in something like a CMS (content management system). This app would only need two templates (in addition to a landing page, an about page, etc.): `blog` and `post`. For simplicity, we'll put the list of all blog posts in `blog`, and then each post will have its own URL under `post`.

The `blog` template will be rendered to `/blog`, and will only use the _build state_ strategy, fetching a list of all our posts from the CMS every time the blog is rebuilt (or you could use revalidation and incremental generation to mean you never have to rebuild at all, but that's beyond the scope of this section). This template only generates one page, providing it the properties of the list of blog posts. So, in this case, the `blog` template has generated the `/blog` page.

The `post` template is more complex, and it will generate _many_ pages, one for each blog post. This would probably use the _build paths_ strategy, which lets you fetch a list of blog posts from the CMS at build-time and invoke _build state_ for each of them, which would then get their content, metadata, etc. Thus, the `post` template generates many pages.

Hopefully that explains the difference between a template and a post. This is a somewhat unintuitive part of Perseus, but it should be clear in the documentation what the difference is. Note however that old versions of the examples in the repository used these terms interchangeably, when they used to be the same. If you see any remaining ambiguity in the docs, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose)!

## Defining a Template

You can define a template like so (taken from [the basic example](https://github.com/arctic-hen7/perseus/blob/main/examples/basic/src/templates/about.rs)'s about page):

```rust,no_run,no_playground
{{#include ../../../../examples/basic/src/templates/about.rs}}
```

It's seen as convention in Perseus to define each template in its own file, which should expose a `get_template()` file. Note that this is just convention, and as long as you get an instance of `Template<G>` to the `define_app!` macro, it really doesn't matter. That said, using community conventions makes your code easier to understand and debug for others.

There's a list of all the methods available on a template [here](https://docs.rs/perseus/0.2/perseus/template/struct.Template.html#implementations), along with explanations of what they all do. Technically, you could just define a template without calling any of these, but that would just render a blank page, which would probably be useless.

## Routing

Perseus' routing system is basically invisible, there's no router that you need to work with, nor any place for you to define explicit routes. Instead, Perseus automatically infers the routes for all your templates and the pages they generate from their names!

The general rule is this: a template called `X` will be rendered at `/X`. Simple. What's more difficult to understand is what we call _template path domains_, which is the system that makes route inference possible. **A template can only ever generate pages within the scope of its root path.** Its root path is its name. In the example of a template called `X`, it can render `/X/Y`, `/X/Y/Z`, etc., but it can _never_ render `/A`.

To generate paths within a template's domain, you can use the _build paths_ and _incremental generation_ strategies (more on those later). Both of these support dynamic parameters (which might be denoted in other languages as `/post/<title>/info` or the like).

### Dynamic Parameters Above the Domain

One niche case is defining a route like this: `/<locale>/about`. In this case, the `about` template is rendered underneath a dynamic parameter. This is currently impossible in Perseus, but the most common reason to need it, internationalization (making your app work in many language), is support out-of-the-box with Perseus.

### Different Templates in the Same Domain

It's perfectly possible in Perseus to define one template for `/post` (and its children) and a different one for `/post/new`. In fact, this is exactly what [the showcase example](https://github.com/arctic-hen7/perseus/tree/main/examples/showcase) does, and you can check it out for inspiration. This is based on a simple idea: **more specific templates win** the routing contest.

There is one use-case though that requires a bit more fiddling: having a different template for the root path. A very common use-case for this would be having one template for `/posts`'s children (one URl for each blog post) and a different template for `/posts` itself that lists all available posts. Currently, the only way to do this is to define a property on the `posts` template that will be `true` if you're rendering for that root, and then to conditionally render the list of posts. Otherwise, you would render the given post content. This does require a lot of `Option<T>`s, but they could be safely unwrapped (data passing in Perseus is logical and safe).

## Checking Render Context

It's often necessary to make sure you're only running some logic on the client-side, particularly anything to do with `web_sys`, which will `panic!` if used on the server. Because Perseus renders your templates in both environments, you'll need to explicitly check if you want to do something only on the client (like get an authentication token from a cookie). This can be done trivially with the `is_server!` macro, which does exactly what it says on the tin. Here's an example from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/i18n/src/templates/about.rs):

```rust,no_run,no_playground
{{#include ../../../../examples/i18n/src/templates/about.rs}}
```

This is a very contrived example, but what you should note if you try this is the flash from `server` to `client` (when you go to the page from the URL bar, not when you go in from the link on the index page), because the page is pre-rendered on the server and then hydrated on the client. This is an important principle of Perseus, and you should be aware of this potential flashing (easily solved by a less contrived example) when your users [initially load](../advanced/initial-loads.md) a page.

One important thing to note with this macro is that it will only work in a _reactive scope_ because it uses Sycamore's [context system](https://sycamore-rs.netlify.app/docs/advanced/contexts). In other words, you can only use it inside a `template!`, `create_effect`, or the like.
