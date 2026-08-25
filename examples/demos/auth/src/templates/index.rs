use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::*;

fn index_view() -> View {
    let AppStateRx { auth } = Reactor::from_cx().get_global_state::<AppStateRx>();
    let AuthDataRx { state, username } = auth.clone();
    // This isn't part of our data model because it's only used here to pass to the
    // login function
    let entered_username = create_signal(String::new());

    // We have to trigger this from outside the `create_memo`, and we should only be
    // interacting with storage APIs in the browser (otherwise this would be called
    // on the server too) This will only cause a block on the first load,
    // because this function just returns straight away if the state is already
    // known
    #[cfg(client)]
    auth.detect_state();

    view! {
        (
            match state.get_clone() {
                LoginState::Yes => {
                    let username = username.get_clone();
                    // Clone for the closure (required for 'static lifetime in Sycamore 0.9.2)
                    let auth_logout = auth.clone();
                    view! {
                            h1 { (format!("Welcome back, {}!", &username)) }
                            button(on:click = move |_| {
                                #[cfg(client)]
                                auth_logout.logout();
                            }) { "Logout" }
                    }
                }
                // You could also redirect the user to a dedicated login page
                LoginState::No => {
                    // Clone for the closure (required for 'static lifetime in Sycamore 0.9.2)
                    let auth_login = auth.clone();
                    view! {
                        h1 { "Welcome, stranger!" }
                        input(bind:value = entered_username, placeholder = "Username")
                        button(on:click = move |_| {
                            #[cfg(client)]
                            // In Sycamore 0.9.2, use get_clone() for non-Copy types
                            auth_login.login(&entered_username.get_clone())
                        }) { "Login" }
                    }
                },
                // This will appear for a few moments while we figure out if the user is logged in or not
                LoginState::Server => View::new(),
            }
        )
        br()
        a(href = "about") { "About" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(index_view).build()
}
