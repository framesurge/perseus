use perseus::{RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, View};

#[perseus::make_rx(PageStateRx)]
pub struct PageState {
    pub greeting: String,
}

#[perseus::template_rx]
pub fn build_state_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("build_state")
        .build_state_fn(get_build_state)
        .template(build_state_page)
}

// We're told the path we're generating for (useless unless we're using build paths as well) and the locale (which will be `xx-XX` if we're not using i18n)
// Note that this function is asynchronous, so we can do work like fetching from a server or the like here (see the `demo/fetching` example)
#[perseus::build_state]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        greeting: "Hello World!".to_string(),
    })
}
