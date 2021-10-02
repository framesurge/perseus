# Static Content

It's very routine in a web app to need to access *static content*, like images, and Perseus supports this out-of-the-box. Any and all static content for your website that should be served over the network should be put in a directory called `static/`, which should be at the root of your project (NOT under `src/`!). Any files/folders you put in there will be accessible on your website at `/.perseus/static/[filename-here]` **to anyone**. If you need content to be protected in some way, this is not the mechanism to use (consider a separate API endpoint)!

## Aliasing Static Content

One problem with making all static content available under `/.perseus/static/`  is that there are sometimes occasions where you need it available at other locations. The most common example of this is `/favicon.ico` (the little logo that appears next to your app's title in a browser tab), which must be at that path.

*Static aliases* allow you to handle these conditions with ease, as they let you define static content to be available at any given path, and to map to any given file in your project's directory.

You can define static aliases in the `define_app!` macro's `static_aliases` parameter. Here's an example from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/basic/src/lib.rs):

```rust,no_run,no_playground
{{#include ../../../examples/basic/src/lib.rs}}
```

### Security

Of course, being able to serve any file on your system in a public-facing app is a major security vulnerability, so Perseus will only allow you to create aliases for paths in the current directory. Any absolute paths or paths that go outside the current directory will be disallowed. Note that these paths are defined relative to the root of your project.

**WARNING:** if you accidentally violate this requirement, your app **will not load** at all!
