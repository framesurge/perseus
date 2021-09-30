# Testing

When building a web app, testing is extremely important, and also extremely helpful. If you're familiar with Rust, you're probably used to having two types of tests (unit tests and integration tests), but Perseus follows the JavaScript model of testing slightly more, which is better suited to a user-facing web app, and has three types of tests:

- Unit tests -- same as in Rust, they test a small amount of logic in isolation
- Integration tests -- same as in Rust, they test the system itself, but sometimes mocking things like a database
- End-to-end tests -- not mocking anything at all, and fully testing the entire system as if a real user were operating it

It's that last type that Perseus is particularly concerned with, because that's the way that you can create highly resilient web apps that are tested for real user interaction. In fact, most of Perseus itself is tested this way! Also, E2E tests are more effective at automating otherwise manual testing of going through a browser and checking that things work, and they're far less brittle than any other type of test (all that matters is the final user experience).

In terms of unit tests, these can be done for normal logic (that doesn't render something) with Rust's own testing system. Any integration tests, as well as unit tests that do render things, should be done with [`wasm-bindgen-test`](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html). This module provides a custom *test harness* macro (alternative to `#[test]`) that spins up a *headless browser* (browser without a GUI) that can be used to render your code. Note that this should be done for testing Sycamore components, and not for testing integrated Perseus systems.

When you want to test logic flows in your app, like the possibilities of how a user will interact with a login form, the best way is to use end-to-end testing, which Perseus supports with a custom test harness macro that can be used like so (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/basic/tests/main.rs)):

```rust,no_run,no_playground
{{#include ../../../../examples/basic/tests/main.rs}}
```

The first thing to note is the module that this test imports. It's called [Fantoccini](https://github.com/jonhoo/fantoccini), and it basically lets you control a web browser with code. We'll get to exactly how this works soon. This test goes to <http://localhost:8080> (where a Perseus app is hosted) and then clicks a link on it and makes sure that it's been taken to the correct new URL.

The other important thing to note is the signature of this test function. First, it's annotated with `#[perseus::test]`, which means this will expand into a larger function that makes your function work. It takes a Fantoccini client as a parameter (which we've called `c` for convenience, you'll use it a lot), and returns a result. **In Perseus E2E tests, you shouldn't panic, but return an error gracefully instead**, which gives the harness time to disconnect from the headless browser. If you don't do this, you'll leave the browser in limbo, and other connections will fail, and everything will blow up in your face. Note that `assert!`, `assert_eq!`, and `assert_ne!` do `panic!` if they fail, which will cause the browser to be left in limbo.

## Writing a Test

You can write your own tests by creating files of whatever names you'd like under `test/` in your project's root directory (as you would with traditional Rust integration tests), and then you can write tests like the above example. Don't worry if you stuff up the arguments or the return type slightly, Perseus will let you know. Also note that **test functions must be asynchronous**.

You'll also need to add the following to your `Cargo.toml` (`tokio` is needed for running your tests asynchronously):

```toml
{{#include ../../../../examples/basic/Cargo.toml:14:16}}
```

## Running Tests

Perseus tests can be run with `cargo test` as usual, but you'll need to provide the `PERSEUS_RUN_WASM_TESTS` environment variable as true. This makes sure that you don't accidentally run tests that have external dependencies (like a headless browser). Note that, by default, your tests will run in a full browser, so you'll get GUI windows opening on  your screen that are controlled by your tests. These can be extremely useful for debugging, but they're hardly helpful on CI, so you can remove them and run *headlessly* (without a GUI window) by providing the `PERSEUS_RUN_WASM_TESTS_HEADLESS` environment variable.

Before running E2E tests, you need to have two things running in the background:

- Something that allows you to interact with a headless browser using the *WebDriver* protocol (see below)
- Your app, invoked with `perseus test` (different to `perseus serve`)

<details>
<summary>How would I automate all that?</summary>

It may be most convenient to create a shell script to do these for you, or to use a tool like [Bonnie](https://github.com/arctic-hen7/bonnie) to automate the process. You can see an example of how this is done for a large number of tests across multiple different example apps in the [Perseus repository](https://github.com/arctic-hen7/perseus).

</details>

*Note: Cargo runs your tests in parallel by default, which won't work with some WebDrivers, like Firefox's `geckodriver`. To run your tests sequentially instead (slower), use `cargo test -- --test-threads 1` (this won't keep your tests in the same order though, but that's generally unnecessary).*

## WebDrivers?

So far, we've mostly gone through this without explaining the details of a headless browser, which will be necessary to have some basic understanding of. Your web browser is composed a huge number of complex moving parts, and these are perfect for running end-to-end tests. They have rendering engines, Wasm execution environments, etc. Modern browsers support certain protocols that allow them to be controlled by code, and this can be done through a server like [Selenium](https://selenium.dev). In the case of Perseus though, we don't need something quite so fancy, and a simple system like [`geckodriver`](https://github.com/mozilla/geckodriver) for Firefox or [`chromedriver`](https://chromedriver.chromium.org/) for Chromium/Chrome will do fine.

If you're completely new to headless browsers, here's a quick how-to guide with Firefox so we're all on the same page (there are similar steps for Google Chrome as well):

1. Install [Firefox](https://firefox.com).
2. Install [`geckodriver`](https://github.com/mozilla/geckodriver). On Ubuntu, this can be done with `sudo apt install firefox-geckodriver`.
3. Run `geckodriver` in a terminal window on its own and run your Perseus tests elsewhere.
4. Press Ctrl+C in the `geckodriver` terminal when you're done.

*Note: if your WebDriver instance is running somewhere other than <http://localhost:4444>, you can specify that with `#[perseus::test(webdriver_url = "custom-url-here")]`.*
