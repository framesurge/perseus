# Authentication

If you're building an app with multiple users that might have different preferences or the like, chances are you'll need some way for those users to log in, you'll need an authentication system. This is fairly easy to achieve in Perseus with the [global state](:reference/state/global) system, though there are a few gotchas -- hence this tutorial!

## Concepts

Authentication as a process involves the user wanting to go to some page that they need to be logged in to access (e.g. a dashboard of bank statements), logging in, accessing that page, and then maybe logging back out a little later. All that really boils down to in terms of code is a system to manage whether or not the user is logged in, a system of authenticating the user's password (or however else they're logging in), and a system of providing access on certain pages only to authenticated users.

The first part can be achieved through an entry in an app's global state that describes whether or not the user is logged in, what their username is, etc. Notably, this could be saved through [state freezing](:reference/state/freezing) to IndexedDB (which would preserve all the properties of the user's login) (not covered in this tutorial), though we still need a way of securely confirming that the user is who they say they are. For that, we would need to store a token, often done in the browser's [local storage](TODO) API, which we'll use in this example. As for how this token works, that requires a good deal of thought to security and how your app will work, and so is elided here (we'll just use a very insecure 'token' that tells us what the user's username is).

The final part of this is controlling access to protected pages, which is the part where Perseus becomes more relevant as a framework. There are two types of protected pages that you might have, user-specific and non-user-specific. If a protected page is user-specific, then it's useless without the user's personal data. For example, a list of bank statements is completely worthless to an attacker without the user's bank statements populating it, rather than a loading skeleton. For these kinds of pages, we can render a skeleton on the server that's then populated with the user's information once the page is loaded in the browser. This means we don't have to go to any extra lengths to prevent access to the skeleton, since we'll assume that the user's data can only be accessed over an APi that needs some unique token that can only be generated with the user's password, or something similar.

If a protected page is non-user-specific, that means it contains content that's the same for all users, but that should only be accessible to users who have logged in. These are more complex because protecting them requires that you don't prerender them on the server at all, and that the code for the protected content not be in your codebase. That may seem weird -- how can you render it at all if it's not in your codebase? Well, you'd have to check if the user is authenticated, and then use some token to fetch the protected content from a server and then display that. If you were to have the protected content anywhere in your code, then it would be accessible to any user willing to reverse-engineer the generated WebAssembly (which isn't too tricky), and hence not really protected at all.

## Secure Authentication

TODO

## Building the App

### Setup

To start with, we'll set up a fairly typical Perseus app by initializing a new Rust project with `cargo new --lib`. Then, put the following in `Cargo.toml` (changing the package name as you want):

```toml
{{#include ../../../examples/demos/auth/Cargo.toml}}
```

The only things of particular note here are the dependency on `web-sys`, from which we use the `Storage` feature (important later), as well as not using Sycamore's [hydration](:reference/hydration) system, as it doesn't handle the kinds of page changes from unauthenticated to authenticated that we'll need in this app. Note that hydration will likely work fine with this in future version of Sycamore (it's currently experimental though).

Now add the following to `src/lib.rs`:

```rust
{{#include ../../../examples/demos/auth/src/lib.rs}}
```

This is a very typical scaffold, but the use of the global state creator is important, and that's what we'll look at next. You can put whatever you want into `src/error_pages.rs` to serve as your app's error pages, but that isn't the subject of this tutorial. You can read more about error pages [here](:reference/error-pages) though.

In `src/global_state.rs`, put the following code:

```rust
{{#include ../../../examples/demos/auth/src/global_state.rs}}
```

This is fairly intense, so let's break it down.

The first thing we do is create the function we call from `src/lib.rs`, `get_global_state_creator`, which initializes a `GlobalStateCreator` with the `get_build_state` function to create the initial global state (generated on the server and passed to the client). What that function does is generates an instance of `AppState`, a `struct` that will store our app's global state (which can include anything you want), which crucially has the `auth` field, an instance of `AuthData`, which will store the data for user authentication. Notably, all these `struct`s are annotated with `.make_rx()` to make them work with Perseus' state platform (note that `AppState` declares nested reactivity for the `auth` field, which you can read more about [here](:reference/state/global)).

`AuthData` has two fields: `state` and `username`. The first is a `LoginState`, which can be `Yes` (the user is logged in), `No` (the user is not logged in), or `Server` (the page has been rendered on the server and we don't have any information about the user's login status yet). The reason for these three possibilities is so we don't assume the user to be logged out before we've even gotten to their browser, as that might result in an ugly flash between pages, or worse an inappropriate redirection to a login page. By forcing ourselves to handle the `Server` case, we make our code more robust and clearer.

You might be wondering why we don't store `username`, which is just a `String`, as a property of `LoginState::Yes`, which would seem to be a much smarter data structure. This is absolutely true, but the problem is that the `make_rx` macro isn't smart enough to handle `enum`s, so we'd have to implement the `MakeRx` trait manually, which is a little tedious. To keep things simple, we'll go with storing `username` separately, but if you have multiple fields of information only relevant to authenticated users, you may want to take the more complex approach for cleanliness.

Next, we implement some functions on `AuthDataRx`, the reactive version of `AuthData`, not bothering to do so on the original because we'll only use these functions in templates, in which we have the reactive version. The first method is `.detect_state()`, which will, if the state is `LoginState::Server`, check if the user is logged in by checking the `username` key in the browser's storage (not IndexedDB, local storage instead, which is more appropriate for this sort of thing). Note that this kind of 'token' management is absolutely atrocious and completely insecure, and serves only as an example of how you might start with authentication. Do NOT use this in a production app!

The only other two functions are very simple, just `.login()` and `.logout()`, which alter the storage key and the global state to register a new login state.

## Templates

Okay, let's get into writing some views based on all this! We'll create an index page and an about page for demonstration, so set up a `src/templates/` directory with a `mod.rs` that declares both files. Then put the following in `src/templates/index.rs`:

```rust
{{#include ../../../examples/demos/auth/src/templates/index.rs}}
```

The only strange stuff in here is in `index_view()`, the rest is pretty much bog-standard Perseus template code. In `index_view()`, we don't take any template sttate, for demonstration purposes (you easily could), but we do take in the global state, which you'll remember contains all the authentication properties. Then we set up some `Signal`s outside the data model for handling a very simple login input (again, demonstrations). The important thing is the call to `auth.detect_state()`, which will refresh the authentication status by checking the user's browser for the login 'token' being stored. Note that, because we coded this to return straight away if we already know the login state, it's perfectly safe to put this at the start of every template you want to be authentication-protected. We also gate this with `#[cfg(target_arch = "wasm32")]` to make sure it only runs on the browser (because we can't check for storage tokens in the server build process, that will throw plenty of errors!).

Skipping past the scary `let view = ...` block for a moment, the end of this function is dead simple: we just display a Sycamore `View<G>` stored in a `Signal` (that's in the `view` variable), and then a link to the about page. Anything other than that `(*view.get())` call will be displayed *whether the user is authenticated or not*.

Now for the fun part. To give us maximum editor support and cleanliness, we define the bulk of the view code outside the `view!` macro and in a variable called `view` instead, a derived `Signal` built with `create_memo` running on `auth.state`. So, if `auth.state` changes, this will also update immediately and automatically! All we do here is handle each of the three possible authentication states with a `match` statement: if we're on the server, we'll display nothing at all; if the user isn't logged in, a login page; and if they are, a welcome message and a logout button. In a real-world app, you'd probably have some code that redirects the user to a login page in the `LoginState::No` case.

You might be wondering why we display nothing before the login state is known, because this would seem to undercut the purpose of preloading the page at all. The answer to this question is that it does, and in an ideal world you'd process the user's login data on the server-side before serving them the appropriate prerendered page, which you *could* do, but that would be unnecessarily complex. Instead, we can display a blank page for a moment before redirecting or loading the appropriate skeleton.

In theory though, on some odler mobile devices, this blank screen might be visible for more than a moment (on 3G networks, it could be 2 seconds or more), which is not good at all. To remedy this, you could make `LoginState::Server` and `LoginState::Yes` render the same skeleton (with some blanks for unfetched user information), so you're essentially assuming the user to be logged in. That means only anonymous users get a flash, from the skeleton to a login page. If your login page is at a central route (e.g. `/login`), you could inject some JavaScript code to run before any of your page is rendered that would check if the user is logged in, and then redirect them to the login page before any of the page loaded if not. This is the best solution, which involves no flashing whatsoever, and the display time of your app is optimized for all users, without needing any server-side code!

*Note: in future, there will likely be a plugin to perform this optimization automatically. If someone wants to create this now, please open a PR!*

Finally, add the following into the about page (just a very basic unprotected page for comparison):

```rust
{{#include ../../../examples/demos/auth/src/templates/about.rs}}
```

## Conclusion

Authentication in Perseus is fairly easy to implement, easier than in many other frameworks, though there are a few hurdles to get over and patterns to understand that will make your code more idiomatic. In future, nearly all this will likely be handled automatically by a plugin or library, which would enable more rapid and efficient development of complex apps. For now though, authentication must be built manually into Perseus apps.
