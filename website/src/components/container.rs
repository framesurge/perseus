use super::footer::Footer;
use super::header::{Header, HeaderProps};
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ContainerProps {
    pub header: HeaderProps,
    pub children: Children<'a>,
    pub footer: bool,
}

#[component]
pub fn Container(props: ContainerProps<'a>) -> View {
    let children = props.children.call();

    view! {
        Header(props.header)
        main(id = "scroll-container") {
            (children)
        }
        (if props.footer {
            view! {
                    Footer {}
            }
        } else {
            View::empty()
        })
    }
}
