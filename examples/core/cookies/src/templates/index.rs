use perseus::{RenderFnResultWithCause, Request, Template};
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

#[perseus::make_rx(IndexPropsRx)]
pub struct IndexProps {
    cookie_value: String,
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(cx: Scope<'a>, props: IndexPropsRx) -> View<G> {
    view! { cx,
        p { (props.cookie_value) }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}

#[perseus::request_state]
pub async fn get_request_state(
    _path: String,
    _locale: String,
    req: Request,
) -> RenderFnResultWithCause<IndexProps> {
    let cookie_value = req.get_cookie("test");
    req.set_cookie("foo", "bar");
    Ok(IndexProps { cookie_value })
}
