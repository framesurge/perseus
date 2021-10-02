# Error Pages

When developing for the web, you'll need to be familiar with the idea of an *HTTP status code*, which is a numerical indication in HTTP (HyperText Transfer Protocol) of how the server reacted to a client's request. The most well-known of these is the infamous *404 Not Found* error, but there are dozens of these in total. Don't worry, you certainly don't need to know all of them by heart!

## Handling HTTP Status Codes in Perseus

Perseus has an *app shell* that manages fetching pages for the user (it's a little more complicated than the traditional design of that kind of a system, but that's all you need to know for now), and this is where HTTP errors will occur as it communicates with the Perseus server. If the status code is an error, this shell will fail and render an error page instead of the page the user visited. This way, an error page can be displayed at any route, without having to navigate to a special route.

You can define one error page for each HTTP status code in Perseus, and you can see a list of those [here](https://httpstatuses.com). Here's an example of doing so for *404 Not Found* and *400* (a generic error caused by the client) (taken from [here](https://github.com/arctic-hen7/perseus/tree/main/examples/showcase/src/error_pages.rs)):

```rust,no_run,no_playground
{{#include ../../../examples/showcase/src/error_pages.rs}}
```

It's conventional in Perseus to define a file called `src/error_pages.rs` and put your error pages in here for small apps, but for larger apps where your error pages are customized with beautiful logos and animations, you'll almost certainly want this to be a folder, and to have a separate file for each error page.

When defining an instance of `ErrorPages`, you'll need to provide a fallback page, which will be used for all the status codes that you haven't specified unique pages for. In the above example, this fallback would be used for, say, a *500* error, which indicates an internal server error.

The most important thing to note about these error pages is the arguments they each take, which have all been ignored in the above example with `_`s. There are four of these:

- URL that caused the error
- HTTP status code (`u16`)
- Error message
- Translator (inside an `Option<T>`)

## Translations in Error Pages

Error pages are also available for you to use yourself (see the [API docs](https://docs.rs/perseus) on the functions to call for that) if an error occurs in one of your own pages, and in that case, if you're using i18n, you'll have a `Translator` available. However, there are *many* cases in Perseus in which translators are not available to error pages (e.g. the error page might have been rendered because the translator couldn't be initialized for some reason), and in these cases, while it may be tempting to fall back to the default locale, you should optimally make your page as easy to decipher for speakers of other languages as possible. This means emoji, pictures, icons, etc. Bottom line: if the fourth parameter to an error page is `None`, then communicate as universally as possible.

An alternative is just to display an error message in every language that your app supports, which may in some cases be easier and more practical.
