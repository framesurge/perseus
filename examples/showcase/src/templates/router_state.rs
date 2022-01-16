use perseus::{templates::RouterLoadState, Html, Template};
use sycamore::prelude::{cloned, component, create_memo, view, View};

#[perseus::template(RouterStatePage)]
#[component(RouterStatePage<G>)]
pub fn router_state_page() -> View<G> {
    let load_state = sycamore::context::use_context::<perseus::templates::RenderCtx>()
        .router
        .get_load_state();
    let load_state_str = create_memo(
        cloned!(load_state => move || match (*load_state.get()).clone() {
            RouterLoadState::Loaded(name) => format!("Loaded {}.", name),
            RouterLoadState::Loading(new) => format!("Loading {}.", new),
            RouterLoadState::Server => "We're on the server.".to_string()
        }),
    );

    view! {
        a(href = "about", id = "about-link") { "About!" }


        p { (load_state_str.get()) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("router_state").template(router_state_page)
}
