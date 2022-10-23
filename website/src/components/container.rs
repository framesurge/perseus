use super::footer::Footer;
use super::header::{Header, HeaderProps};
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ContainerProps<'a, G: Html> {
    pub header: HeaderProps<G>,
    pub children: Children<'a, G>,
    pub footer: bool,
}

#[component]
pub fn Container<'a, G: Html>(cx: Scope<'a>, props: ContainerProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    view! { cx,
        Header(props.header)
        main(id = "scroll-container") {
            (children)
        }
        (if props.footer {
            view! { cx,
                    Footer {}
            }
        } else {
            View::empty()
        })
    }
}
