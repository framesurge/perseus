# Fantoccini Basics

Now that you know a bit more about how Perseus tests work, it's time to go through how to write them!

Remember, you're controlling an actual browser, so you basically have everything available to you that a user can do (mostly). You can even take screenshots! All this is achieved with [Fantoccini](https://github.com/jonhoo/fantoccini), which you can learn more about [here](https://docs.rs/fantoccini). For now though, here's a quick tutorial on the basics, using [this](https://github.com/arctic-hen7/perseus/blob/main/examples/basic/tests/main.rs) example:

```rust,no_run,no_playground
{{#include ../../../../examples/basic/tests/main.rs}}
```

## Going to a Page

You can trivially go to a page of your app by running `c.goto("...")`. The above example ensures that the URL is valid, but you shouldn't have to do this unless you're testing a page that automatically redirects the user. Also, if you're using [i18n](../i18n/intro.md), don't worry about testing automatic locale redirection, we've already done that for you!

Once you've arrived at a page, you should wait for the `router_entry` (this example uses `begin` because it tests internal parts of Perseus) checkpoint, which will be reached when Perseus has decided what to do with your app. If you're testing particular page logic, you should wait instead for `page_visible`, which will be reached when the user could see content on your page, and then for `page_interactive`, which will be reached when the page is completely ready. Remember though, you only need to wait for the checkpoints that you actually use (e.g. you don't need to wait for `page_visible` and `page_interactive` if you're not doing anything in between).

## Finding an Element

You can find an element easily by using Fantoccini's `Locator` `enum`. This has two options, `Id` or `Css`. The former will find an element by its HTML `id`, and the latter will use a CSS selector ([here](https://www.w3schools.com/cssref/css_selectors.asp)'s a list of them). In the above example, we've used `Locator::Css("p")` to get all paragraph elements, and then we've plugged that into `c.find()` to get the first one. Then, we can get its `innerText` with `.text()` and assert that is what we want it to be.

### Caveats

As you may have noticed above, asserting on the contents of a `<title>` is extremely unintuitive, as it requires using `.html(false)` (meaning include the element tag itself) and asserting against that. For some reason, neither `.html(true)` nor `.text()` return anything. There's a tracking issue for this [here](https://github.com/jonhoo/fantoccini/issues/136).

## Miscellaneous

For full documentation of how Fantoccini works, see its API documentation [here](https://docs.rs/fantoccini).
