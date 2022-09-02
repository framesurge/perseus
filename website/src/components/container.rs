use super::footer::Footer;
use super::header::{Header, HeaderProps};
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ContainerProps<G: Html> {
    pub header: HeaderProps<G>,
    pub children: View<G>,
    pub footer: bool,
}

#[component(Container<G>)]
pub fn Container<G: Html>(cx: Scope, props: ContainerProps<G>) -> View<G> {
    view! { cx,
        Header(props.header)
        main(id = "scroll-container") {
            (props.children.clone())
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
