use super::footer::Footer;
use super::header::{Header, HeaderProps};
use sycamore::prelude::*;

#[derive(Props)]
pub struct ContainerProps {
    pub header: HeaderProps,
    pub children: Children,
    pub footer: bool,
}

#[component]
pub fn Container(props: ContainerProps) -> View {
    let children = props.children.call();
    // In Sycamore 0.9.2, Option props need to be unwrapped or defaulted
    let menu_signal = props.header.menu_open.unwrap_or_else(|| create_signal(false));

    view! {
        Header(
            text_color = props.header.text_color,
            menu_color = props.header.menu_color,
            title = props.header.title,
            mobile_nav_extension = props.header.mobile_nav_extension,
            menu_open = menu_signal,
        )
        main(id = "scroll-container") {
            (children)
        }
        (if props.footer {
            view! {
                    Footer {}
            }
        } else {
            View::new()
        })
    }
}
