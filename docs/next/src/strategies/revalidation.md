# Revalidation

While the *build state* and *build paths* strategies are excellent for generating pages efficiently, they can't be updated for new content. For example, using these strategies alone, you'd need to rebuild a blog every time you added a new post, even if those posts were stored in a database. With *revalidation*, you can avoid this by instructing Perseus to rebuild a template if certain criteria are met when it's requested.

There are two types of revalidation: time-based and logic-based. The former lets you re-render a template every 24 hours or the like, while the latter allows you to re-render a template if an arbitrary function returns `true`.

## Time-Based Revalidation Usage

Here's an example of time-based revalidation from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/src/templates/time.rs) (note that this uses *incremental generation* as well):

```rust,no_run,no_playground
{{#include ../../../../examples/showcase/src/templates/time.rs}}
```

This page displays the time at which it was built (fetched with *build state*), but rebuilds every five seconds. Note that this doesn't translate to the server's actually rebuilding it every five seconds, but rather the server will rebuild it at the next request if more than five seconds have passed since it was last built (meaning templates on the same build schedule will likely go our of sync quickly).

### Time Syntax

Perseus uses a very simple syntax inspired by [this JavaScript project]() to specify time intervals in the form `xXyYzZ` (e.g. `1w`, `5s`, `1w5s`), where the lower-case letters are number and the upper-case letters are intervals, the supported of which are listed below:

- `s`: second,
- `m`: minute,
- `h`: hour,
- `d`: day,
- `w`: week,
- `M`: month (30 days used here, 12M ≠ 1y!),
- `y`: year (365 days always, leap years ignored, if you want them add them as days)

## Logic-Based Revalidation Usage

Here's an example of logic-based revalidation from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase/src/templates/time_root.rs) (actually, this example uses both types of revalidation):

```rust,no_run,no_playground
{{#include ../../../../examples/showcase/src/templates/time_root.rs}}
```

If it were just `.should_revalidate_fn()` being called here, this page would always be rebuilt every time it's requested (the closure always returns `true`, note that errors would be `String`s), however, the additional usage of time-based revalidation regulates this, and the page will only be rebuilt every five seconds. In short, your arbitrary revalidation logic will only be executed at the intervals of your time-based revalidation intervals (if none are set, it will run on every request).

Note that you should avoid lengthy operations in revalidation if at all possible, as, like the *request state* strategy, this logic will be executed while the client is waiting for their page to load.
