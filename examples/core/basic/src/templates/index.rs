use perseus::{Html, RenderFnResultWithCause, SsrNode, Template, state::{Freeze, MakeRx, MakeUnrx}};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{view, Scope, View, Signal};

// #[perseus::make_rx(IndexPageStateRx)]
#[derive(Serialize, Deserialize)]
pub struct IndexPageState {
    pub greeting: String,
}
// Prepend lifetime to generics of reactive version
#[derive(Clone)]
pub struct IndexPageStateRx<'rx> {
    pub greeting: &'rx Signal<String>
}
impl<'rx> MakeRx<'rx> for IndexPageState {
    type Rx = IndexPageStateRx<'rx>;
    fn make_rx(self, cx: Scope<'rx>) -> Self::Rx {
        IndexPageStateRx {
            greeting: sycamore::prelude::create_signal(cx, self.greeting)
        }
    }
}
impl<'rx> MakeUnrx<'rx> for IndexPageStateRx<'rx> {
    type Unrx = IndexPageState;
    fn make_unrx(self) -> Self::Unrx {
        IndexPageState {
            greeting: (*self.greeting.get_untracked()).clone()
        }
    }
}
impl Freeze for IndexPageStateRx<'_> {
    fn freeze(&self) -> ::std::string::String {
        let unrx = IndexPageState {
            greeting: (*self.greeting.get_untracked()).clone()
        };
        ::serde_json::to_string(&unrx).unwrap()
    }
}

#[perseus::template_rx]
pub fn index_page<'rx, G: Html>(cx: Scope<'rx>, state: IndexPageStateRx<'rx>) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::head]
pub fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
    })
}
