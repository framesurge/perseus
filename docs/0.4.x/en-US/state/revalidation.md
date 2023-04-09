# Revalidation

Sometimes, you'll want to use build-time state generation, but you'll want to update the state you've generated at a later date. For example, let's say you have a website that lists the latest news, and build state is used to do that. If you want to update this news every hour, you could do that with revalidation! (This avoids much of the overhead of request-time state, which must be generated before every single page load, and has no opportunity for caching.)

Generally, if you can use it, revalidation will yield better performance than request-time state.

## Time-based revalidation

The first type of revalidation is the simplest: you set a schedule with `.revalidate_after()` on `Template`, which takes either a `Duration` (from `chrono` or the standard library) or a string of the form `<num><unit>`, like `1h` for one hour. You can read more about that [here](=template/struct.TemplateInner@perseus).

This will cause the Perseus build process to, for each page that this template generates, note down the current time, and write that to a file. Then, on each request, it will check if the current time is later than that recorded time, plus the revalidation interval. If so, then it will re-execute the build state function, and update the state accordingly. Templates using revalidation have their pages stored in the mutable store, since they may update later.

Crucially, this is lazy revalidation: Perseus will not immediately revalidate a page once the revalidation interval is reached. For example, if our news site isn't very popular for its first month, and only gets two visits per day, it won't revalidate 24 times, it will probably revalidate twice: because only two people visited. This also means that revalidation can behave in unexpected ways. Let's say you have a page that revalidates every five seconds, and it's built at second 0. If, no-one requests it until second 6, and then there's a request every second, it will revalidate at second 6, then second 11, then second 16, etc. You may need to re-read that to understand this, and it's usually not a problem, unles syou have very strict requirements.

Note that this is all page-specific, so it's entirely possible for two different pages in the same template to have teh same revalidation interval and revalidate at different times.

## Logic-based revalidation

When you have more stringent needs, you might wish to use logic-based revalidation, which is based on the `.should_revalidate_fn()` method on `Template`. To this, you provide an `async` function of the usual sort with the usual `BlamedError<E>` error handling (see [here](:state/build) for an explanation of that) that takes a [`StateGeneratorInfo`](=prelude/struct.StateGeneratorInfo@perseus) instance and the user's request, and you return a `bool`: if it's true, the page will revalidate, but, if `false`, the old state will stand. This can be used to do more advanced things like having a database of new news, but also having a micro-site set to tell you whether or not there is new news. Thus, you can perform the quicker check to the micro-site (which acts as a [canary](https://en.wikipedia.org/wiki/Sentinel_species)) to avoid unnecessary revalidations, which will improve performance.

Using both logic-based revalidation *and* time-based revalidation is perfectly permissible, as the logic-based revalidation will only be executed on the interval of the time-based. For our news site, therefore, we might want to use the logic-based revalidation to check a canary as to whether or not there is any new news, and then only run that check hourly. This would lead to hourly checks of whether or not we *should* revalidate, rather than just blindly doing so, which can improve performance greatly.

## Example

An example of using both logic-based and time-based revalidation together is below.

```rust
{{#include ../../../examples/core/state_generation/src/templates/revalidation.rs}}
```
