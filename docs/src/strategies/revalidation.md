# Revalidation

This strategy allows you to rebuild pages built with the *build state* strategy on a later request. A common reason for this might be to update a statically rendered list of blog posts every 24 hours so it's up-to-date relatively regularly. Perseus' revalidation strategy allows you re-render a page on two conditions: time-based and logic-based. The time-based variant lets you provide a string like `1w`, and then your page will be re-rendered every week. The logic-based variant lets you provide a function that returns a boolean as to whether or not to re-render, which will be run on every request to the page. Notably, the variants can be combined so that you run a logic check only after some length of time.

The time-based strategy adds very little server overhead, as it simply performs a time check, though it does involve another read from your data cache, which may be computationally expensive. The logic-based check is as expensive as you make it.

## Time-Based Variant

The time based variant does have some slightly weird behaviour to watch out for though, which is best explained by explaining how it works.

1. Evaluates your time string (e.g. `1w` for 1 week) to a number of seconds after January 1 1970 (how computers represent time). This provides a timestamp in the future, past which your page should be revalidated.
2. On every request, Perseus checks if this timestamp has been passed yet. If it has, it re-renders your page. This means that **your page will only be revalidated after the time has elapsed *and* a user has queried it**.
3. After revalidation, Perseus repeats from step 1. However, this may not be 2 weeks after the original build (in our example of `1w`), but 1 week after the revalidation, whcih may have been later than a week after the original setting.

To put it simply, Perseus will only revalidate when requested, so don't expect different pages to be synchronised in their revalidations, even if they all have the same timestamp.

This logic is a bit weird, so you may need to think about it for a bit. Don't worry though, it shouldn't impact your app negatively in any way, it's just something to take note of!

## Time Syntax

Perseus lets you define revalidation intervals as strings, the syntax for which is as follows: `xXyYzZ...`, where lower-case letters are numbers meaning the number of the interval X/Y/Z (e.g. 1m4d -- one month four days).

The available intervals are:

- s: second,
- m: minute,
- h: hour,
- d: day,
- w: week,
- M: month (30 days used here, 12M ≠ 1y!),
- y: year (365 days always, leap years ignored, if you want them add them as days)

## Usage

You can add this strategy to a template like so:

```rust,no_run,no_playground
template
	// ...
    .revalidate_after("5s".to_string())
    .should_revalidate_fn(Box::new(|| async { Ok(true) }))
```

That example uses both variants of revalidation, but you can use one or both as necessary. Note that the logic-based variant must be asynchronous, and errors must be returned as `String`s.
