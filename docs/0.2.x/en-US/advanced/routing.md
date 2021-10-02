# Routing

Perseus' routing system is quite unique in that it's almost entirely *inferred*, meaning that you don't ever have to define a router or explain to the system which paths go where. Instead, they're inferred from templates in a system that's explained in detail in the [templates section](../templates/intro.md).

## Template Selection Algorithm

Perseus has a very specific algorithm that it uses to determine which template to use for a given route, which is greatly dependent on `.perseus/dist/render_conf.json`. This is executed on the client-side for *subsequent loads* and on the server-side for *initial loads*.

Here's an example render configuration (for the [showcase example](https://github.com/arctic-hen7/perseus/blob/main/examples/showcase)), which maps path to template root path.

```json
{
    "about": "about",
    "index": "index",
    "post/new": "post/new",
    "ip": "ip",
    "post/*": "post",
    "timeisr/test": "timeisr",
    "timeisr/*": "timeisr",
    "time": "time",
    "amalgamation": "amalgamation",
    "post/blah/test/blah": "post",
    "post/test": "post"
}
```

Here are the algorithm's steps (see [`router.rs`](https://github.com/arctic-hen7/perseus/blob/main/packages/perseus/src/router.rs)):

1. If the path is empty, set it to `index` (which is used for the landing page).
2. Try to directly get the template name by trying the path as a key. This would work for anything not using incremental generation (in the above example, anything other than `post/*`).
3. Split the path into sections by `/` and iterate through them, performing the following on each section (iterating forwards from the beginning of the path, becoming more and more specific):
	1. Make a path out of all segments up to the current point, adding `/*` at the end (indicative of incremental generation in the render configuration).
	2. Try that as a key, return if it works.
	3. Even if we have something, continue iterating until we have nothing. This way, we get the most specific path possible (and we can have incremental generation in incremental generation).

## Relationship with Sycamore's Router

Sycamore has its own [routing system](https://sycamore-rs.netlify.app/docs/v0.6/advanced/routing), which Perseus depends on extensively under the hood. This is evident in `.perseus/src/lib.rs`, which invokes the router. However, rather than using the traditional Sycamore approach of having an `enum` with variants for each possible route (which was the approach in Perseus v0.1.x), Perseus provides the router with a `struct` that performs routing logic and returns either `RouteVerdict::Found`, `RouteVerdict::LocaleDetection`, or `RouteVerdict::NotFound`. The render configuration is accessed through a global variable implanted in the user's HTML shell when the server initializes.
