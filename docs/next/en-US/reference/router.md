# Router

Most of the time, you will never have to worry about how Perseus' router works. However, if you're working on a new server integration, or something else very low-level, you might encounter some thorns to do with how Perseus figures out which page to render. This might seem like a simple problem. The user requests `/about`? Render the `about` template. Now introduce [incremental generation](:reference/state-generation). Now it's a little more complex. For clarity, this page will outline the way Perseus' routing algorithm actually works internally.

If you're just building apps with Perseus, you shouldn't have to read this, it's more for those working with the internals.

Note that this algorithm is executed on the client-side for _subsequent loads_ and on the server-side for _initial loads_ (see [here](:reference/initial_subsequent_loads) for details on those).

Also note that Perseus' routing algorithm is based on a file called `render_conf.json`, which is stored in `dist/`. Importantly, this is stored in memory by the server, and it's interpolated directly into the HTML sent to the user's browser. (Meaning apps with *very\** large numbers of pages should consider incremental generation even if their build times are fine, since it may actually improve load times by a little. Take a look at the `<script>` tags in the `<head>` of this website to see what we mean!)

Here's an example render configuration (for the [state generation example](https://github.com/arctic-hen7/perseus/blob/main/examples/core/state_generation)), which maps URL to template name.

```json
{
    "amalgamation":"amalgamation",
    "revalidation_and_incremental_generation/test":"revalidation_and_incremental_generation",
    "incremental_generation/blah/test/blah":"incremental_generation",
    "incremental_generation/test":"incremental_generation",
    "build_paths/blah/test/blah":"build_paths",
    "build_paths/test":"build_paths",
    "revalidation_and_incremental_generation/*":"revalidation_and_incremental_generation",
    "request_state":"request_state",
    "build_paths":"build_paths",
    "revalidation":"revalidation",
    "build_state":"build_state",
    "incremental_generation/*":"incremental_generation",
    "revalidation_and_incremental_generation/blah/test/blah":"revalidation_and_incremental_generation"
}
```

Here are the algorithm's steps (see [`match_route.rs`](https://github.com/arctic-hen7/perseus/blob/main/packages/perseus/src/router/match_route.rs)):

1. If the path is empty, set it to `index` (which is used for the landing page).
2. Try to directly get the template name by trying the path as a key. This would work for anything not using incremental generation (in the above example, anything other than `incremental_generation/*` and `revalidation_and_incremental_generation/*`).
3. Split the path into sections by `/` and iterate through them, performing the following on each section (iterating forwards from the beginning of the path, becoming more and more specific):
    1. Make a path out of all segments up to the current point, adding `/*` at the end (indicative of incremental generation in the render configuration).
    2. Try that as a key, return if it works.
    3. Even if we have something, continue iterating until we have nothing. This way, we get the most specific path possible (and we can have incremental generation in incremental generation).
