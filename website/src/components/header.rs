use perseus::{link, t};
use sycamore::prelude::*;

#[derive(Prop)]
pub struct HeaderProps<G: Html> {
    /// The text color used across the whole header.
    pub text_color: String,
    /// The color used for the hamburger menu on mobile. This should use
    /// background colors
    pub menu_color: String,
    /// The title of the page to be used in the header.
    pub title: String,
    /// Additional contents that should be added to the navigation menu on
    /// mobile.
    pub mobile_nav_extension: View<G>,
    /// An optional field that allows the caller to control menu opening
    /// imperatively.
    pub menu_open: Option<RcSignal<bool>>,
}

/// The header for the entire app.
#[component]
pub fn Header<G: Html>(
    cx: Scope,
    HeaderProps {
        title,
        text_color,
        menu_color,
        mobile_nav_extension,
        menu_open,
    }: HeaderProps<G>,
) -> View<G> {
    // Use the given menu opening `Signal` if it was provided, or create a new one
    let menu_open = match menu_open {
        Some(signal) => create_ref(cx, signal),
        None => create_signal(cx, false),
    };
    let toggle_menu = |_| menu_open.set(!*menu_open.get());

    view! { cx,
        header(
            // This doesn't have a background color, we blur the background based on the content underneath
            class = format!(
                "shadow-md sm:p-2 w-full mb-20 bg-neutral-500/30 backdrop-blur-lg {}",
                &text_color
            )
        ) {
            div(class = "flex justify-between items-center") {
                a(class = "justify-self-start self-center m-3 ml-5 text-md sm:text-2xl text-bold title-font", href = link!(cx, "/")) {
                    (title)
                }
                // The button for opening/closing the hamburger menu on mobile
                // This is done by a Tailwind module
                div(
                    class = format!(
                        "md:hidden m-3 mr-5 tham tham-e-spin tham-w-6 {}",
                        if *menu_open.get() {
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
                    if *menu_open.get() {
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
fn NavLinks<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        li(class = "m-3 p-1 title-font") {
            a(href = link!(cx, "/docs"), class = "px-2") { (t!(cx, "navlinks.docs")) }
        }
        li(class = "m-3 p-1 title-font") {
            a(href = link!(cx, "/comparisons"), class = "px-2") { (t!(cx, "navlinks.comparisons")) }
        }
        li(class = "m-3 p-1 title-font") {
            a(href = link!(cx, "/plugins"), class = "px-2") { (t!(cx, "navlinks.plugins")) }
        }
    }
}
