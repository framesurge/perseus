use sycamore::prelude::*;

// NOTE: None of the code in this file is Perseus-specific! This could easily be
// applied to any Sycamore app.

#[component]
pub fn Layout<G: Html>(cx: Scope, props: LayoutProps<G>) -> View<G> {
    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
        header(style = "background-color: red; color: white; padding: 1rem") {
            p { (props.title) }
        }
        main(style = "padding: 1rem") {
            (props.children)
        }
        footer(style = "background-color: black; color: white; padding: 1rem") {
            p { "Hey there, I'm a footer!" }
        }
    }
}

#[derive(Prop)]
pub struct LayoutProps<G: Html> {
    /// The title of the page, which will be displayed in the header.
    pub title: String,
    /// The content to put inside the layout.
    pub children: View<G>,
}
