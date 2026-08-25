use perseus::prelude::*;
use sycamore::prelude::*;

fn router_state_page() -> View {
    let load_state_str = create_signal("We're on the server.".to_string());

    #[cfg(client)]
    {
        use perseus::router::RouterLoadState;
        // Clone for the closure (required for 'static lifetime in Sycamore 0.9.2)
        let load_state_str_clone = load_state_str.clone();
        // This uses Sycamore's `create_effect` to create a state that will update
        // whenever the router state changes
        create_effect(move || {
            // Get the load state inside the effect to avoid lifetime issues
            let reactor = Reactor::from_cx();
            let load_state = reactor.router_state.get_load_state();
            // In Sycamore 0.9.2, .get() returns the value directly (not a reference)
            let new_str = match load_state.get_clone() {
                RouterLoadState::Loaded {
                    template_name,
                    path,
                } => {
                    perseus::web_log!("Loaded.");
                    // `path` is a `PathMaybeWithLocale`, a special Perseus type to indicate
                    // a path that will be prefixed with a locale if the app uses i18n, and
                    // not if it doesn't; it derefences to `&String`.
                    format!("Loaded {} (template: {}).", *path, template_name)
                }
                RouterLoadState::Loading {
                    template_name,
                    path,
                } => format!("Loading {} (template: {}).", *path, template_name),
                RouterLoadState::Server => "We're on the server.".to_string(),
            };
            load_state_str_clone.set(new_str);
        });
    }

    view! {
        p { (load_state_str.get_clone()) }

        Link(to = "/about", id = "about-link") { "About!" }
    }
}

pub fn get_template() -> Template {
    Template::build("index").view(router_state_page).build()
}
