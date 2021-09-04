# Setting up the App Itself

Okay, you're over the hump! Now it's time to put your template into a functional app and get stuff happening in your browser!

## Defining a Router

Perseus needs to know where to put your final pages, which can be defined with a router, which is defined through [Sycamore](https://github.com/sycamore-rs/sycamore) (which handles that side of things).

In `src/lib.rs`, replace everything other than `mod templates;` with the following:

```rust
use perseus::define_app;

#[derive(perseus::Route)]
pub enum Route {
    #[to("/")]
    Index,
    #[not_found]
    NotFound,
}
```

This imports a macro we'll use in a moment to define your app, and then it sets up an `enum` for each of your pages. Notice that the `NotFound` page is special, and note that Perseus will pretty much handle it for you.

All we've done for this simple app is defined an `Index` variant that will be served at the `/` path, the root of your app. Thus, it will be your landing page! But Perseus still needs you to connect that variant and the template we created in the last section.

## Error Pages

But first, let's define some custom error pages for if your users go to a page that doesn't exist. To keep everything clean, we'll do this in a new file. Create `src/error_pages.rs` and put the following inside (making sure to add `mod error_pages;` to the top of `lib.rs`):

```rust
use perseus::ErrorPages;
use sycamore::template;

pub fn get_error_pages() -> ErrorPages {
    let mut error_pages = ErrorPages::new(Box::new(|_, _, _| {
        template! {
            p { "Another error occurred." }
        }
    }));
    error_pages.add_page(
        404,
        Box::new(|_, _, _| {
            template! {
                p { "Page not found." }
            }
        }),
    );
    error_pages.add_page(
        400,
        Box::new(|_, _, _| {
            template! {
                p { "Client error occurred..." }
            }
        }),
    );

    error_pages
}
```

Here's what this code does:

1. Import the Perseus [`ErrorPages`](https://docs.rs/perseus/0.1.2/perseus/shell/struct.ErrorPages.html) `struct`, and the Sycamore templating macro for writing pseudo-HTML.
2. Define a single function that will get all your error pages (you'll call this in `lib.rs`).
3. Create a new instance of `ErrorPages` with the required fallback page. Error pages in Perseus are based on HTTP status codes (but you can create your own beyond this system if you need), and there are *a lot* of them, so the fallback page is used for all the status codes that you don't explicitly handle.
4. Add two new error pages, one for 404 (page not found) and another for 400 (generic client error). Note that the functions we provide have to be `Box`ed (so Rust can allocate the memory properly), and they'll also be provided three arguments, which you'll want to use in a production app. They are: the URL that caused the problem, the HTTP status code, and the error message that was the payload of the request.

You can read more about error pages [here](https://arctic-hen7.github.io/perseus/error_pages.html).

## Setting up Some HTML

Perseus is just a web framework, and it needs some good old HTML to cling to, so you'll need to create an `index.html` file in the root of your project (*next* to `src/`). Then put the following inside:

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta http-equiv="X-UA-Compatible" content="IE=edge" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Perseus Starter App</title>
        <!-- Importing this runs Perseus -->
        <script src="/.perseus/bundle.js" defer></script>
    </head>
    <body>
        <div id="root"></div>
    </body>
</html>
```

This is all pretty typical, and you shouldn't have to worry about it. If you want to include analytics or some other script in your app, you can do it here, this HTML will be on every page of your app (including errors).

Note the single `<script>` that imports from `/.perseus/bundle.js`. That's the stuff that Rollup generates, and the CLI will serve it automatically. We also `defer` it for best practice.

## Defining Your App

And now for the magic. Perseus does a ton of stuff to initialize your app, all of which can be abstracted through the `define_app!` macro that we imported earlier. Add the following to `lib.rs` (underneath your `Route` definition) to use it:

```rust,no_run,no_playground
define_app! {
    root: "#root",
    route: Route,
    router: {
        Route::Index => [
            "index".to_string(),
            templates::index::template_fn()
        ]
    },
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        crate::templates::index::get_template::<G>()
    ]
}
```

*Note: these properties currently **must** be in order, otherwise they won't work.*

And again, here's an explanation:

1. Define the CSS selector at which Perseus will render. This will be the `<div id="root"></div>` we defined in `index.html` in this case.
2. Tell Perseus about the `Route` `enum` you defined earlier.
3. Handle each of your roots with `match`-like syntax. You'll need to handle every variant of your `Route` `enum` here except the `NotFound` variant, which will use your 404 error page (or the fallback if you didn't define one for that status code). Each route that goes to the path at which it will be rendered (which may seem pointless, but it's *really* useful for more complex pages) and the `template_fn` helper function we defined in the last section.
4. Tell Perseus about your error pages.
5. Declare each of your templates, using that `get_template` helper function we defined in the last section. Notice the ambient `G` parameter here, which that function also took. That lets Perseus control whether it renders your page for the server (as for server-side rendering) or for the client (in the browser), and it needs to do both.

## Ship It.

It's time to run your app for the first time! If you're using an editor like VS Code with a Rust plugin, then you shouldn't have any compile-time errors in your code (if you do, that's a problem...).

We'll use the Perseus CLI to take care of everything from here. Go into your app's root folder (with `src/` and `Cargo.toml`) in a terminal, and run this command:

```
perseus serve
```

This will take quite a while the first time, because Perseus is literally creating another crate in the background (check out the new `.perseus/` folder if you're interested) and building all its dependencies. Also note that your `.gitignore` has been added to to ignore the `.perseus/` folder, which can be rebuilt on any machine with the Perseus CLI installed, so there's no point having it in Git.

When all 5 steps have been completed, your app will be available in the browser! Go to <http://localhost:8080> and you should see `Welcome to the app!` on your screen! If so, then congratulations, you've just created your first ever Perseus app! (And don't worry, running it will be much faster next time.)

You can see all the code put together [here](https://github.com/arctic-hen7/perseus-starter-app).

## Further Reading

Now that you've created your first app with Perseus, you're more than ready to venture into the rest of the documentation, which will explain each aspect of what you've achieved here in further detail, and will guide you in building more complex apps, particularly in the [rendering strategies](https://arctic-hen7.github.io/perseus/strategies/intro.html) section, which explains the real magic of Perseus' rendering.

So go forth and build something amazing!
