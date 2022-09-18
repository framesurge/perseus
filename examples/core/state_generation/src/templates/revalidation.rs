use perseus::{RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, View};

#[perseus::make_rx(PageStateRx)]
pub struct PageState {
    pub time: String,
}

#[perseus::template_rx]
pub fn revalidation_page<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (format!("The time when this page was last rendered was '{}'.", state.time.get())) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("revalidation")
        .template(revalidation_page)
        // This page will revalidate every five seconds (and so the time displayed will be updated)
        .revalidate_after("5s")
        // This is an alternative method of revalidation that uses logic, which will be executed
        // every time a user tries to load this page. For that reason, this should NOT do
        // long-running work, as requests will be delayed. If both this
        // and `revalidate_after()` are provided, this logic will only run when `revalidate_after()`
        // tells Perseus that it should revalidate.
        .should_revalidate_fn(should_revalidate)
        .build_state_fn(get_build_state)
}

// This will get the system time when the app was built
#[perseus::build_state]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        time: format!("{:?}", std::time::SystemTime::now()),
    })
}

// This will run every time `.revalidate_after()` permits the page to be
// revalidated This acts as a secondary check, and can perform arbitrary logic
// to check if we should actually revalidate a page
#[perseus::should_revalidate]
pub async fn should_revalidate(
    _path: String,
    _locale: String,
    _req: perseus::Request,
) -> RenderFnResultWithCause<bool> {
    // For simplicity's sake, this will always say we should revalidate, but you
    // could make this check any condition
    Ok(true)
}
