use crate::global_state::{AppStateRx, LoginState};
use perseus::{Html, Template};
use sycamore::prelude::*;

#[perseus::template_rx]
fn index_view(_: (), AppStateRx { auth }: AppStateRx) -> View<G> {
    // This isn't part of our data model because it's only used here to pass to the login function
    let entered_username = Signal::new(String::new());
    let eu_2 = entered_username.clone();

    // We have to trigger this from outside the `create_memo`, and we should only be interacting with storage APIs in the browser (otherwise this would be called on the server too)
    // This will only cause a block on the first load, because this function just returns straight away if the state is already known
    #[cfg(target_arch = "wasm32")]
    auth.detect_state();

    // We make the view as a memo outside the root `view!` for better editor support (some editors don't like highlighting code in macros)
    // We need to clone `global_state` because otherwise the `Signal` updates won't be registered
    let view = create_memo(cloned!(auth => move || {
        match *auth.state.get() {
            LoginState::Yes => {
                let username = auth.username.get();
                view! {
                    h1 { (format!("Welcome back, {}!", &username)) }
                    button(on:click = cloned!(auth => move |_| {
                        auth.logout();
                    })) { "Logout" }
                }
            }
            // You could also redirect the user to a dedicated login page
            LoginState::No => view! {
                h1 { "Welcome, stranger!" }
                input(bind:value = entered_username.clone(), placeholder = "Username")
                    button(on:click = cloned!(eu_2, auth => move |_| {
                        auth.login(&eu_2.get())
                    })) { "Login" }
            },
            // This will appear for a few moments while we figure out if the user is logged in or not
            LoginState::Server => View::empty(),
        }
    }));
    view! {
        (*view.get())
        br()
        a(href = "about") { "About" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_view)
}
