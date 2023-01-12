# Testing

As with any software, it's a good idea to test your apps to make sure they work as expected, especially when they're too large to test manually. This can be achieved through unit testing, integration testing, and all manner of other test, but the main concern of Perseus is *end-to-end tests*, where you start up an actual browser, and a program takes over the input to that browser. Quite literally, you then code short programs that will interact with a real version of your app and ensure that what happens is correct.

## Writing tests

Actually writing end-to-end (E2E for short) tests is quite simple, and Perseus provides an opinionated macro `#[perseus::test]` to simplify the process further. This macro uses [`fantoccini`], a library thhat lets you control browsers programmatically. For the vast majority of use-cases, this is absolutely fine, and you'll very rarely need to break out of this macro.

Here's an example of some E2E tests, actually taken from the internal Perseus testing!

```rust
{{#include ../../../examples/core/basic/tests/main.rs}}
```

*We'll get to what 'checkpoints' are in a moment.*

Note that E2E tests are typically placed in a separate directory `tests/` at the root of your project. These will be picked up by `cargo test`, but they will appear to immediately pass when run like this, because the `#[perseus::test]` macro checks for a few environment variables (since running E2E tests requires a whole heap of extra infrastructure). If you're not using this macro, be sure to immediately pass your test if the `PERSEUS_RUN_WASM_TESTS` environment variable is not set.

For more information on what kinds of things you can with E2E tests, check out [`fantoccini`'s API documentation].

## `perseus test`

When it comes to actually running your tests, there are three things you need to do: first, you have to actually be running a server for your app in a special 'testing mode', then you need to be also running a *WebDriver* (an interface to programmatically control a browser), and then you need to run `cargo test` with some environment variables set to actually run your tests.

Of course, managing all this is quite tedious, especially on CI, so Perseus automates most of it with the `perseus test` command, which builds your app for testing and runs your tests against it. Note that this will also run all your unit tests etc., as well as your E2E tests.

*Note: unfortunately, there is currently no option but to run E2E tests in a single-threaded manner, reducing speed substantially, because some WebDriver implementations do not yet support multi-threaded access. To our knowledge, there is no way around this yet.*

As with other Perseus commands, `perseus test` will give some nice green ticks if everything goes to plan, sparing you all the gritty details, and it will print everything if something goes wrong. When you're debugging failing tests, it can sometimes be useful to see exactly what the browser sees, which is what the `--show-browser` option is for. This disables *headless* testing (i.e. running the browser without opening it) and shows you everything your code is doing to control the browser. This will usually happen very quickly, so you may want to introduce `std::thread::sleep()` calls or similar so you can see the state of each page to figure out what's going wrong.

## WebDrivers

The only other thing you need to do while running `perseus test` is run a compliant WebDriver in the background, which Perseus will communicate with to interface with the browser. By default, the `#[perseus::test]` macro assumes the WebDriver is running on port 4444, but this can be changed by specifying a custom URL to that same macro.

*Note: in future, Perseus may suppport automatically installing and executing WebDrivers, but this is currently not implemented.*

For example, if you want to test your app in Firefox, then you would first install `geckodriver` from [here](https://github.com/mozilla/geckodriver/releases) (or it may come bundled with a `firefox` package, depending on your OS), and then run `geckodriver` in one terminal while you run `perseus test` in another. The WebDriver will remain running after `perseus test` finishes, allowing you to run further tests against it if you like. On CI, you'll usually have a step that runs `geckodriver &` to run the process and move it to the background.

Importantly, if you terminate `perseus test` manually, with Ctrl+C or the like, or if one of your tests panics, this may lead to the WebDriver being interrupted and ending up in a broken state, in which case you'll have to restart it. 

## Checkpoints

One thing you may notice about the above example is the use of the `wait_for_checkpoint!()` macro, which, as it says, waits for the given checkpoint. In essence, a checkpoint can be thought of as an event that Perseus fires at certain times, but only when it's in testing mode. The number after this is the index of the event, starting at zero. For example, if you're waiting for the third time a page will become interactive, you should wait for `"page_interactive", 2`. The final argument is just the Fantoccini client. Note that, unlike other event-based systems, if you wait for a Perseus checkpoint that has already happened, it will immediately resolve.

Importantly, checkpoints are preserved across subsequent loads. This means that, if you, say, navigate from the landing page of your app to the about page, the checkpoint counters do not restart. If you want them to, you should refresh the page directly.

Current checkpoints are:

- `begin`: Perseus has initialized
- `page_interactive`: the page is interactive and ready to use
- `error`: the opposite of `page_interactive` , indicating that an error has occurred
- `not_found`: the page was not found (a subset of `error`, but both will be emitted in this case)

This list may grow in future, but any checkpoint removals or changes of meaning will be considered breaking changes.

### Custom checkpoints

It is entirely permissible, and indeed encouraged, for apps that have more advanced render flows to define and emit their own checkpoints, which should be prefixed with `custom`, and must not contain hyphens (or a panic will occur). This can be done with the [`checkpoint`](=prelude/fn.checkpoint@perseus) function. Note that checkpoints will only be emitted during `perseus test`, and will otherwise be ignored.

Common uses of custom checkpoints are particularly when combined with the [suspended state] system, if some of your pages fetch state on the client-side, and you want to test for that all working correctly.

If a checkpoint is not emitted, tests waiting for it will fail with a timeout error.
