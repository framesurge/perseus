use perseus::prelude::*;
use sycamore::prelude::*;

fn router_state_page<G: Html>(cx: Scope) -> View<G> {
    let load_state_str = create_signal(cx, "We're on the server.".to_string());

    #[cfg(client)]
    {
        use perseus::router::RouterLoadState;
        let load_state = Reactor::<G>::from_cx(cx).router_state.get_load_state(cx);
        // This uses Sycamore's `create_memo` to create a state that will update
        // whenever the router state changes
        create_effect(cx, || {
            let new_str = match (*load_state.get()).clone() {
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
                // Since this code is running in a page, it's a little pointless to handle an error
                // page, which would replace this page (we wouldn't be able to
                // display anything if this happened)
                RouterLoadState::ErrorLoaded { .. } => unreachable!(),
            };
            load_state_str.set(new_str);
        });
    }

    view! { cx,
        a(href = "about", id = "about-link") { "About!" }

        p { (load_state_str.get()) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(router_state_page).build()
}
