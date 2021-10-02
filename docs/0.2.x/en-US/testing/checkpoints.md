# Checkpoints

If you start using Perseus' testing system now, you'll likely hit a snag very quickly, involving errors to do with *stale DOM elements*. This is an unfortunate side-effect of the way Perseus currently handles initial loads (we move a number of DOM elements around after they've been sent down from the server), which results in the WebDriver thinking half the page has just disappeared out from under it!

This, and many similar problems, are easily solvable using one of Perseus' most powerful testing tools: *checkpoints*. When you run your app with `perseus test`, a system is enabled in the background that writes a new DOM element to a hidden list of them when any app code calls `checkpoint()`. This can then be detected with Fantoccini! Admittedly, a far nicer solution would be DOM events, but the WebDriver protocol doesn't yet support listening for them (understandable since it's mimicking a user's interaction with the browser).

Note that checkpoints will never be reached if your app is not run with `perseus test`. If you use `--no-run` and then execute the server binary manually, be sure to provide the `PERSEUS_TESTING=true` environment variable.

You can wait for a Perseus checkpoint to be reached like so (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/basic/tests/main.rs)):

```rust,no_run,no_playground
{{#include ../../../../examples/basic/tests/main.rs}}
```

Note in particular the use of the `wait_for_checkpoint!` macro, which accepts three arguments:

- Name of the checkpoint
- Version of the checkpoint
- Fantoccini client

For want of a better term, that second argument refers to how Perseus manages checkpoints. Because a single checkpoint might be emitted multiple times, Perseus attaches a number to the end of each. The final element `id` looks like this: `__perseus_checkpoint-<checkpoint_name>-<number>`, where `<number>` starts from 0 and increments.

*Note: checkpoints are not cleared until the page is fully reloaded, so clicking a link to another page will not clear them!*

## Custom Checkpoints

In addition to Perseus' internal checkpoints (listed below), you can also use your own checkpoints, though they must follow the following criteria:

- Must not include hyphens (used as a delimiter character), use underscores instead
- Must not conflict with an internal Perseus checkpoint name

The best way to uphold the latter of those criteria is to prefix your own checkpoints with something like the name of your app, or even just `custom_`. Of course, if your app has a name like `router`, then that will be a problem (many Perseus checkpoints begin with `router_`), but Perseus will never generate checkpoints internally that begin with `custom_`.

Note that it's not enough to make sure that your checkpoints don't clash with any existing checkpoints, as new checkpoints may be added in any new release of Perseus, so conflicts may arise with the tiniest of updates!

## Internal Checkpoints

Perseus has a number of internal checkpoints that are listed below. Note that this list will increase over time, and potentially in patch releases.

- `begin` -- when the Perseus system has been initialized
- `router_entry` -- when the Perseus router has reached a verdict and is about to either render a new page, detect the user's locale and redirect, or show an error page
- `not_found` -- when the page wasn't found
- `app_shell_entry` -- when the page was found and it's being rendered
- `initial_state_present` -- when the page has been rendered for the first time, and the server has preloaded everything (see [here](../advanced/initial-loads.md) for details)
- `page_visible` -- when the user is able to see page content (but the page isn't interactive yet)
- `page_interactive` -- when the page has been hydrated, and is now interactive
- `initial_state_not_present` -- when the initial state is not present, and the app shell will need to fetch page data from the server
- `initial_state_error` -- when initial state showed an error
