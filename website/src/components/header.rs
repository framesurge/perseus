use sycamore::prelude::*;
use perseus::t;

#[derive(Prop)]
pub struct HeaderProps {
    /// The text color used across the whole header.
    pub text_color: String,
}

/// The header for the entire app.
#[component]
pub fn Header<G: Html>(cx: Scope, HeaderProps { classes }: HeaderProps) -> View<G> {
    view! { cx,
        header(
            // This doesn't have a background color, we blur the background based on the content underneath
            class = format!(
                "shadow-md sm:p-2 w-full mb-20 backdrop-blur-md {}",
                &props.text_color
            )
        ) {
            div(class = "flex justify-between") {
                a(class = "justify-self-start self-center m-3 ml-5 text-md sm:text-2xl text-bold title-font", href = link!("/", cx)) {
                    (title)
                }
                // The button for opening/closing the hamburger menu on mobile
                // This is done by a Tailwind module
                div(
                    class = format!(
                        "sm:hidden m-3 mr-5 tham tham-e-spin tham-w-6 {}",
                        if *menu_open.get() {
                            "tham-active"
                        } else {
                            ""
                        }
                    ),
                    on:click = toggle_menu
                ) {
                    div(class = "tham-box") {
                        div(class = "bg-white tham-inner") {}
                    }
                }
                // This displays the navigation links on desktop
                nav(class = "hidden sm:flex") {
                    ul(class = "mr-5 flex") {
                        NavLinks()
                    }
                }
            }
            // This displays the navigation links when the menu is opened on mobile
            // TODO Click-away event
            nav(
                id = "mobile_nav_menu",
                class = format!(
                    "sm:hidden w-full text-center justify-center {}",
                    if *menu_open_2.get() {
                        "flex flex-col"
                    } else {
                        "hidden"
                    }
                )
            ) {
                ul(class = "mr-5") {
                    NavLinks()
                }
            }
        }
    }
}
