# Modifying the `<head>`

A big issue with only having one `index.html` file for your whole app is that you don't have the ability to define different `<title>`s and HTML metadata for each page.

However, Perseus overcomes this easily by allowing you to specify `.head()` on a `Template<G>`, which should return a closure that returns a `Template<SsrNode>` (but just write `perseus::template::HeadFn` as the return type, it's an alias for that). The `template!` you define here will be rendered to a `String` and directly interpolated into the `<head>` of any pages this template renders. If you need it to be different based on the properties, you're covered, it takes the same properties as the normal tempalte function!

The only particular thing to note here is that, because this is rendered to a `String`, this **can't be reactive**. Variable interpolation is fine, but after it's been rendered once, the `<head>` **will not change**. If you need to update it later, you should do that with [`web_sys`](https://docs.rs/web-sys), which lets you directly access any DOM element with similar syntax to JavaScript (in fact, it's your one-stop shop for all things interfacing with the browser, as well as it's companion [`js-sys`](https://docs.rs/js-sys)).

Here's an example of modifying a page's metadata (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/basic/src/templates/index.rs)):

```rust,no_run,no_playground
{{#rustdoc_include ../../../../examples/basic/src/templates/index.rs:43:49}}
```
