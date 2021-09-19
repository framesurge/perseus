# Incremental Generation

This strategy is both the most complex and most simple that Perseus offers. It's simple in that you don't have to write any extra code except telling Perseus to use it (see below), but it's complex in what it does.

Let's say you have an online store, where every item you sell is rendered by the template `item`. If you have 5 items, you might fetch them from a database at build-time and tell Perseus to render each one with the *build paths* strategy. But what if you have 5 million items? If you were to render every single one of these at build time, your builds would take a very long time, especially if you're fetching data for every single item! Enter *incremental generation*, which allows you to return a subset of items with *build paths*, like only the 5 most commonly accessed items, and then the rest are left unspoken.

When a user requests a pre-rendered item, it's served as usual, but if they request something under `/item`, Perseus will detect that that page may well exist, but hasn't been rendered yet, and so it will invoke the *build state* strategy as it would've if this page were being built at build-time, it just does it at request-time! And here's the magic of it, **after the first request, Perseus will cache the page**! So basically, pages are built on demand, and then cached for everyone else! Only the first user to access a page will see the slightest delay.

The one caveat with this strategy is that you need to handle the possibility in the *build state* strategy that the given path may not actually exist, and you'll need to return a 404 (page not found error) in that case. You can do that like so:

```rust,no_run,no_playground
use perseus::ErrorCause

return Err(("custom error message".to_string(), ErrorCause::Client(Some(404))))
```

Note that this tells Perseus that the client caused an error, particularly a 404, which should be handled in your app to return something like 'Page not found'.

## Usage

This strategy can be added to a template like so:

```rust,no_run,no_playground
template
	// ...
	.incremental(true)
```
