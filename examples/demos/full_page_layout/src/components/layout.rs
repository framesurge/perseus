use sycamore::prelude::*;

// NOTE: None of the code in this file is Perseus-specific! This could easily be
// applied to any Sycamore app.

#[component]
pub fn Layout(LayoutProps { title, children }: LayoutProps) -> View {
    let children = children.call();

    view! {
        // These elements are styled with bright colors for demonstration purposes
        header(style = "background-color: red; color: white; padding: 1rem") {
            p { (title) }
        }
        main(style = "padding: 1rem") {
            (children)
        }
        footer(style = "background-color: black; color: white; padding: 1rem") {
            p { "Hey there, I'm a footer!" }
        }
    }
}

#[derive(Props)]
pub struct LayoutProps {
    /// The title of the page, which will be displayed in the header.
    pub title: String,
    /// The content to put inside the layout.
    pub children: Children,
}
