# Routing and navigation

One of things Perseus is big on is *page-based programming*, where each separate view in your app is a completely separate page, since this lets you manage their states all independently. However, one of the thing that needs to happen for you to be able to work like this is *routing*: you need to be able to go from one page to another.

Under the hood, Perseus uses a slightly modified version of [Sycamore's router](https://sycamore-rs.netlify.app/docs/advanced/routing), which means you can use typical Sycamore conventions for both imperative and declarative routing.

## Declarative routing

Declarative routing is when you create an element that will cause routing when it's clicked, and then, when the user clicks it, the routing occurs. In HTML, you would do this by creating a simple anchor tag (`<a>`) with an `href` property equal to where you want to go, and this is...well, exactly what you do in Perseus too! The Sycamore router will automatically detect any links in your app and appropriate them from the browser, so that Perseus can use its special routing behavior to minimize page load times and improve performance (since we know more about the structure of the app than the browser). A link looks like this:

```rust
a(href = "about") { "Click me to go to the about page!" }
```

Remember though, Perseus sets a `<base />` tag that tells the browser to treat all routes as relative to the root of your site. So, if you're at `/my/test/page`, routing to `foo` will go to `/foo`, *not* `/my/test/foo`! This is an important difference between Perseus and a lot of other frameworks. (The reason it's like this is to make it much easier to deploy Perseus under a relative path, like `framesurge.sh/perseus`.)

## Imperative routing

Sometimes, you'll need to write some code that causes a route change, which you can do with the [`navigate`](=prelude/fn.navigate@perseus) function, which is re-exported from the `sycamore-router` package for convenience. You provide this function with a route, and it will take you there! If you want to *replace* the current page in the navigation history, which you can understand by imagining the browser history as a stack of plates that you add things to (`navigate` adds a new plate, replacement navigation replaces the previous plate, meaning the user can't press the back button to go back to it), you can use [`navigate_replace`](=prelude/fn.navigate_replace@perseus). Generally, you won't have a need for this though, as it's really only used in hard redirects and locale redirection (which is handled automatically by Perseus).

## Localized routing

If you're using internationalization, there are a few quirks of routing you should be aware of, which are covered in greater detail on [this page](:fundamentals/i18n). As a summary, put all your links (in `href`s, in `navigate()` calls, etc.) in the `link!` macro, which will prepend the current locale to make sure the user ends up in the right place.
