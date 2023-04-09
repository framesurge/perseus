# Understanding state

As alluded to several times before, the fundamental core of Perseus as a framework is its approach to state management and generation, which this part of the documentation is devoted to.

To understand state, let's first go back to that early idea that **template + state = page**. When we drill into this, several questions come up.

## What actually is 'state'?

State can be best defined in terms of templates. If you imagine a template as a kind of stencil, it literally provides the scaffold of a view, but there are some pieces missing. In the canonical example of a blog post template, the missing parts might be the title, the tags, and the content. When you take all these missing pieces and make a `struct` out of them, that's your state!

When we talk about state in Perseus, this is what we mean, but it's important to understand that state begins on the engine-side, when it's generated, but then ends up on the client-side, where it's used reactively.

## How is state created?

State is created in Perseus through methods of *state generation*, which usually occur at the template-level (though there is also [global state](:stat/global)) through functions you provide. These functions are all `async`, allowing you to do complex work without blocking other state generations, and can be fallible, allowing you to return errors which Persues will gracefully handle and convert into [error views](:fundamentals/error-views). When we talk about state generation, we're talking about the engine-side creation of state through a number of methods (these used to be called *rendering strategies*).

## When is state generated?

State generation can occur in three places: at build-time, at request-time, or on the client-side. The first two are the main ones, and are built into the state generation platform, whereas the third is called [suspended state](:state/suspense), and it involves overriding state generated at either build or request-time with something else. Since, most of the time, you'll just be overriding something like `None`, we classify this as a kind of state generation.

When you generate state at build-time, you're generating it without knowledge of who will be using it. For instance, you can't generate personalized dashboard pages at build-time, because you don't know yet who your users are. You *can* read Markdown files and convert them into HTML to create blog posts, however, or do anything else that doesn't require knowledge of specific users.

Request-time state generation is more powerful, and involves several strategies. There's the most obvious: request-time state itself, which is the creation of state based on the user's actual HTTP request (e.g. you might extract things like cookies here to confirm authentication, etc.), and then there's revalidation, where you update build-time state under certain conditions (e.g. to refresh an index of all the articles on a news site), and also *incremental generation*, where you call build-time logic only as it's needed.

Finally, suspended state is the use of client-side functions to fetch state. Usually, there's nothing suspended state can do that request-time state can't, but suspended state allows rendering the rest of the page before generating some parts of the state, which can lead to a snappier experience, especially if there are some very heavy components of the page.

## How does state relate to templates?

As mentioned before, inserting state into a template produces a page. Importantly, a single template can have many pages, and therefore many states. This is controlled through the *build paths* system, which allows you to generate a simple list of all the pages that should be created within a template. If build paths are not manually specified, Perseus will default to having the template produce just one page at it's own path (e.g. the `about` template produces a page at `/about`).

Note that incremental generation allows this to go further by allowing you to generate arbitrarily more pages at request-time and cache them for future use.

## How is state used on the client-side?

First, the state for the current page is requested and used to hydrate the page (on initial loads, the state comes bundled with the rest of the HTML, speeding things up), and then Perseus makes the state *reactive*. This allows you to update it on the client-side and have your views respond to it. This encourages what is known as the *model-view-controller* pattern in a somewhat novel way: you define all your template's state, and then user interaction modifies it, and this changes the views they see. For example, if one of the parts of a page's state is the contents of a form input, that state coudl be made to update every time the user changes that input. Further, Perseus will store this state internally, allowing it to be easily restored. This all leads on to *state freezing*, a much more advanced and novel concept unique to Perseus that allows state to be restored from a string.

## What if I don't want state?

If you don't want to use *parts* of the state platform, that's absolutely fine! For example, if you don't need any request-time stuff, you can export your app instead of serving it, or you can still serve it, and just not use those features, no problem. If you don't need reactive state on the client-side, you can use unreactive state instead, which is simpler, and this is often a better fit for static websites (like this one).

In short, even if you're building a completely static app, build-time state will probably still be useful to you, and the template-page system can be extremely helpful in building both simple and complex apps. Nonetheless, if you're not using other features of Perseus, like i18n, error views, preloading, hydration, etc., and you're not even using build-time state, you may want to consider another framework, or even just using [Sycamore](https://github.com/sycamore-rs/sycamore) on its own.
