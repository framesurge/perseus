use sycamore::prelude::*;

// NOTE: None of the code in this file is Perseus-specific! This could easily be
// applied to any Sycamore app.

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps { title, children }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
        header(style = "background-color: red; color: white; padding: 1rem") {
            p { (title.to_string()) }
        }
        main(style = "padding: 1rem") {
            (children)
        }
        footer(style = "background-color: black; color: white; padding: 1rem") {
            p { "Hey there, I'm a footer!" }
        }
    }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    /// The title of the page, which will be displayed in the header.
    pub title: &'a str,
    /// The content to put inside the layout.
    pub children: Children<'a, G>,
}
