use perseus::{link, t};
use sycamore::prelude::*;

// This is imported by all alternative containers as well
pub static COPYRIGHT_YEARS: &str = "2021";

#[component(NavLinks<G>)]
pub fn NavLinks<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        li(class = "m-3 p-1 title-font") {
            a(href = link!("/docs", cx), class = "px-2") { (t!("navlinks.docs", cx)) }
        }
        li(class = "m-3 p-1 title-font") {
            a(href = link!("/comparisons", cx), class = "px-2") { (t!("navlinks.comparisons", cx)) }
        }
        li(class = "m-3 p-1 title-font") {
            a(href = link!("/plugins", cx), class = "px-2") { (t!("navlinks.plugins", cx)) }
        }
    }
}

#[derive(Prop)]
pub struct ContainerProps<G: Html> {
    pub title: String,
    pub children: View<G>,
}

#[component(Container<G>)]
pub fn Container<G: Html>(cx: Scope, props: ContainerProps<G>) -> View<G> {
    let title = props.title.clone();
    let menu_open = create_signal(cx, false);
    // We need to verbatim copy the value because of how it's used in Sycamore's
    // reactivity system
    let menu_open_2 = create_memo(cx, || *menu_open.get());
    let toggle_menu = |_| menu_open.set(!*menu_open.get());

    view! { cx,
        header(class = "shadow-md sm:p-2 w-full mb-20 text-white backdrop-blur-md") {
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
        main(class="h-full", id = "scroll-container") {
            (props.children.clone())
        }
        footer(class = "w-full flex justify-center py-5 bg-black text-white backdrop-blur-md") {
            p(class = "mx-5 text-center") {
                span(dangerously_set_inner_html = &t!("footer.copyright", {
                    "years" = COPYRIGHT_YEARS
                }, cx))
            }
        }
    }
}
