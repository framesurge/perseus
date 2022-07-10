# State Generation

One of the most important features of Perseus is its state platform, as explained [here](:core-principles). However, for state to be of any use, you need a way to generate it. Perseus supports *many* ways of doing this: at build-time, at request-time, or several mixes of the two. This page will outline each of the possible state generation strategies.

## Build State

The *build state* strategy is very simple: provide a function to a [`Template`](=struct.Template@perseus) and it will be run when you build your app. Its output will then be used as state!

To take a simple example, let's say you have a (very contrived) page that should display the number of entries in a database at the time it was built. You would make a templat for this, perhaps called `entries` (so it would be hosted at `/entries` on your site), and you'd provide a simple view that calls on some state, which would only need a single property for the number of entries in the DB. Then, just show that!

To make this work though, we need to execute some logic at build-time, which we can do with *build state*! By calling the `.build_state_fn()` method on [`Template`](struct.Template@perseus) and providing a function annotated with `#[perseus::build_state]`, that function will be called when the template is built, and its output will be used to provide the initial value of the state.

We should clarify at this point that the 'initial value' of the state is what's generated *before* it arrives in the browser. Then, it can be changed in the browser (e.g. a default value for an input generated at build-time that the user can change by typing in it), but any such changes will only impact that single browser. Changing the future value of the state involves *revalidation*, described below.

A *build state* function takes two parameters: the path (see below) and the locale it's being built for. (You might use the locale if you're working with [i18n](:reference/i18n) to fetch some language-specific data.) It will then return a [`RenderFnResultWithCause<State>`](=type.RenderFnResultWithCause@perseus), where `State` is your state type.

## Build Paths

But now, let's say we actually have multiple tables in our database, and we want to know how many entries are in each one, with each count being displayed on a different page. Really, we want several pages under that `/entries` path now. This is easily achievable with *build paths*, which allows a single template to generate many pages.

By providing a function to the `.build_paths_fn()` method of [`Template`](=struct.Template@perseus), that will be called at build-time to generate a `Vec<String>` of paths underneath `/entries`. For instant, if we returned `vec![ "foo".to_string(), "bar".to_string(), "baz".to_string() ]`, Perseus would create pages at `/entries/foo`, `/entires/bar`, and `/entries/baz`. If you wanted to create an `/entries` page, you would provide an empty string as one of the elemtents in that `Vec<String>`.

Note that you can also create nested paths like `foo/bar/baz` just like that.

So, in our example, we would query our database for each of its tables, and then return a vector off their names, and Perseus would then generate a page for each, all from that same template!

However, this is absolutely pointless without *build state* as well, since each of those pages would be the same right now. Usefully, as you may have noticed, the *build state* function takes its first parameter as the path, which is designed for working with *build paths*! So, in this case, of `foo`, `bar`, and `baz`, the provided *build state* function would be run three times, once with `entries/foo`, then with `entries/bar`, and finally with `entries/baz`. Notably, these runs will happen concurrently, speeding everything up! You can then use that given path to know which table's entry count to check. By making your *build state* function 'generic' in this way over the path it's given, which is representative here of the database table to fetch a count from, you can easily display many pages with different information, all from the same template!

A *build paths* function takes no arguments, and returns a [`RenderFnResult<Vec<String>>`](=type.RenderFnResult@perseus).

## Request State

However, what if we only wanted to show the counts to certain people? Let's say authorized users will have a cookie in their browser that we can check somehow, and only they should be allowed to view these counts.

We could use *request state* to run a function provided to the `.request_state_fn()` method of [`Template`](=struct.Template@perseus) when a user's request comes in for that page. Unlike the build-time functions, the logic in here has to be very quick, otherwise we'll slow down the page load and reduce the performance of our site.

Usefully, a *request state* function is given the [`Request`](=struct.Request@perseus) from the user's request, which allows access to cookies, etc. With that, we can check if the user has our authentication cookie and make sure that it's valid, and then return a `None` for the count (which would now have to be an `Option<u32>`, see the next section) and `false` for a new `authorized` property.

## Amalgamate States

However, there's a problem with the above idea in most frameworks that support build state and request state, or similar principles. You can only usually use one, since otherwise the build state and the request state might generate conflicting states! This is exactly what would happen here: the build state would happily get the count, and the request state would always override this as `None`, authorized or not, and it would set `authorized`, which the build state might always assume to be `true`. Whatever shall we do?

The answer to this is dead simple: the *state amalgamation* strategy, whjich allows us to take in both of those states and do some arbitrary stuff to resolve them. In this case, based on the value of `authorized` property from the *request state*, we would either return `authorized: false, state: None`, or `authorized: true, state: Some(state)`, where `state` in the latter comes from the *build state* function. Nifty, eh?

Note though that you won't always need state amalgamation, it's mostly useful for adding this kind of authentication to pages that already have build state, allowing you to get the best optimizations and the best security!

Signature TODO.

## Revalidation

Now, what if we wanted to make that count a little more up to date? Say, we should update it daily. Perseus makes this trivial, you just use the `.revalidate_after` method on [`Template`](=struct.Template@perseus) to define an interval, and, every time a new request comes in, if more than that interval has elapsed, then the *build state* function will be re-run.

Alternatively, you might want to perform some logic first to check if the state should be revalidated or not: use `.should_revalidate_fn()` on [`Template`](=struct.Template@perseus) to provide the function that does this.

Note that you can use both time-based *and* logic-based revalidation on the same template if you want to: the logic-based one will only run if the time-based one tells it to.

*Note: if you use revalidation on a template with many pages, revalidation will be performed piecemeal, page-by-page, as each is requested.*

Signatures TODO

## Incremental Generation

Finally, let's say your database is getting a little out of hand, with new tables popping up every other day. You don't want to constantly have to be rebuilding your whole app for each new table!

In this case, you would want to call `.incremental_generation()` on [`Template`](=struct.Template@perseus), which opens the floodgates, so to speak! Essentially, if we still have our three pages from before (i.e. `/entries/foo`, `/entries/bar`, and `/entries/baz`), and the user goes to `/entries/test`, rather than sending a *404 Not Found* error, the Perseus server will run the *build state* function for this path (`entries/test`), and it will build for that database table at request-time! Even better, if this all works, it'll cache the results for next time, meaning it can serve `/entries/test` instantly next time!

As you can imagine, this is *extremely* useful for templates that render millions, or even billions of pages, since you can build them dynamically and cache them for performance at runtime, rather than spending hours building all of them.

And *this* is why you the *build state* function returns a [`RenderFnResultWithCause`](=type.RenderFnResultWithCause@perseus), because you can *blame* either the client or the server. Without incremental generation, you know you'll only get those paths you defined in the *build paths* function, but, with incremental generation, you could get anything. If you know there's one table, say `admin`, that you should never serve a count for, you can add an if-statement to the top of your *build state* function that checks if the `path` argument is `entries/admin`, and returns a *404 Not Found* error, blaming the client, and they'll be none the wiser!

*Note: in applications using both build paths and incremental generation, those paths defined by the build paths function will be rendered at build-time, while any more that aren't defined there will be rendered dynamically upon request.*

## Examples

Some of this may be a little tricky to visualize, so there's an example [here](https://github.com/artic-hen7/perseus/tree/main/examples/core/state_generation) that goes through each of Perseus' state generation strategies systemtically! Note that it doesn't use the same example of a database entry counter as described here, but rather more basic examples to just show the basic functionality of each strategy. Enjoy!
