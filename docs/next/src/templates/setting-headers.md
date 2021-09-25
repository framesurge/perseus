# Modifying HTTP Headers

Most of the time, you shouldn't need to touch the HTTP headers of your Perseus templates, but sometimes you will need to. A particular example of this is if you want your users' browsers to only cache a page for a certain amount of time (the default for Perseus if five minutes), then you'd need to set the [`Cache-Control`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) header.

Perseus supports inserting arbitrary HTTP headers for any response from the server that successfully returns a page generated from the template those headers are defined for. You can do this like so (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/basic/src/templates/index.rs)):

```rust,no_run,no_playground
{{#include ../../../../examples/basic/src/templates/index.rs}}
```

Of note here is the `set_headers_fn` function, which returns a `HeaderMap`. This is then used on the template with `.set_headers_fn()`. Note that the function you provide will be given the state as an argument (ignored here), and you must return some headers (you can't return an error).
