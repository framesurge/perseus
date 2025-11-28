use perseus::{link, t};
use sycamore::prelude::*;

#[derive(Props)]
pub struct HeaderProps {
    /// The text color used across the whole header.
    pub text_color: String,
    /// The color used for the hamburger menu on mobile. This should use
    /// background colors
    pub menu_color: String,
    /// The title of the page to be used in the header.
    pub title: String,
    /// Additional contents that should be added to the navigation menu on
    /// mobile.
    pub mobile_nav_extension: View,
    /// An optional field that allows the caller to control menu opening
    /// imperatively.
    pub menu_open: Option<Signal<bool>>,
}

/// The header for the entire app.
#[component]
pub fn Header(
    HeaderProps {
        title,
        text_color,
        menu_color,
        mobile_nav_extension,
        menu_open,
    }: HeaderProps,
) -> View {
    // Use the given menu opening `Signal` if it was provided, or create a new one
    // In Sycamore 0.9.2, Signal is Copy, so we can use it directly
    let menu_open = match menu_open {
        Some(signal) => signal,
        None => create_signal(false),
    };
    let toggle_menu = move |_| menu_open.set(!menu_open.get());

    view! {
        header(
            // This doesn't have a background color, we blur the background based on the content underneath
            class = format!(
                "shadow-md sm:p-2 w-full mb-20 bg-neutral-500/30 backdrop-blur-lg {}",
                &text_color
            )
        ) {
            div(class = "flex justify-between items-center") {
                a(class = "justify-self-start self-center m-3 ml-5 text-md sm:text-2xl text-bold title-font", href = link!( "/")) {
                    (title)
                }
                // The button for opening/closing the hamburger menu on mobile
                // This is done by a Tailwind module
                div(
                    class = format!(
                        "md:hidden m-3 mr-5 tham tham-e-spin tham-w-6 {}",
                        if menu_open.get() {
                            "tham-active"
                        } else {
                            ""
                        }
                    ),
                    on:click = toggle_menu
                ) {
                    div(class = "tham-box") {
                        div(
                            class = format!(
                                "tham-inner {}",
                                &menu_color,
                            )
                        ) {}
                    }
                }
                // This displays the navigation links on desktop
                nav(class = "hidden md:flex") {
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
                    "md:hidden w-full text-center justify-center {}",
                    if menu_open.get() {
                        "flex flex-col"
                    } else {
                        "hidden"
                    }
                )
            ) {
                ul(class = "mr-5") {
                    NavLinks()
                }
                (mobile_nav_extension)
            }
        }
    }
}

#[component]
fn NavLinks() -> View {
    view! {
        li(class = "m-3 p-1 title-font") {
            a(href = link!( "/docs"), class = "px-2") { (t!( "navlinks.docs")) }
        }
        li(class = "m-3 p-1 title-font") {
            a(href = link!( "/comparisons"), class = "px-2") { (t!( "navlinks.comparisons")) }
        }
        li(class = "m-3 p-1 title-font") {
            a(href = link!( "/plugins"), class = "px-2") { (t!( "navlinks.plugins")) }
        }
    }
}
