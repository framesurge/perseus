use perseus::{link, t};
use sycamore::prelude::*;

// This is imported by all alternative containers as well
pub static COPYRIGHT_YEARS: &str = "2021";

#[component(NavLinks<G>)]
pub fn nav_links() -> View<G> {
    view! {
        li(class = "m-3 p-1") {
            a(href = link!("/docs"), class = "px-2") { (t!("navlinks.docs")) }
        }
        li(class = "m-3 p-1") {
            a(href = link!("/comparisons"), class = "px-2") { (t!("navlinks.comparisons")) }
        }
        li(class = "m-3 p-1") {
            a(href = link!("/plugins"), class = "px-2") { (t!("navlinks.plugins")) }
        }
    }
}

pub struct ContainerProps<G: Html> {
    pub title: String,
    pub children: View<G>,
}

#[component(Container<G>)]
pub fn container(props: ContainerProps<G>) -> View<G> {
    let title = props.title.clone();
    let menu_open = Signal::new(false);
    // We need to verbatim copy the value because of how it's used in Sycamore's reactivity system
    let menu_open_2 = create_memo(cloned!((menu_open) => move || *menu_open.get()));
    let toggle_menu = cloned!((menu_open) => move |_| menu_open.set(!*menu_open.get()));

    view! {
        // TODO click-away events
        header(class = "shadow-md sm:p-2 w-full bg-white dark:text-white dark:bg-navy mb-20") {
            div(class = "flex justify-between") {
                a(class = "justify-self-start self-center m-3 ml-5 text-md sm:text-2xl", href = link!("/")) {
                    (title)
                }
                // The button for opening/closing the hamburger menu on mobile
                // This is done by a Tailwind module
                div(
                    class = format!(
                        "xs:hidden m-3 mr-5 tham tham-e-spin tham-w-6 {}",
                        if *menu_open.get() {
                            "tham-active"
                        } else {
                            ""
                        }
                    ),
                    on:click = toggle_menu
                ) {
                    div(class = "tham-box") {
                        div(class = "dark:bg-white tham-inner") {}
                    }
                }
                // This displays the navigation links on desktop
                nav(class = "hidden xs:flex") {
                    ul(class = "mr-5 flex") {
                        NavLinks()
                    }
                }
            }
            // This displays the navigation links when the menu is opened on mobile
            // TODO click-away event
            nav(
                id = "mobile_nav_menu",
                class = format!(
                    "xs:hidden w-full text-center justify-center {}",
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
        div(class = "overflow-y-scroll") {
            main(class="h-full") {
                (props.children.clone())
            }
        }
        footer(class = "w-full flex justify-center py-5 bg-gray-100 dark:bg-navy-deep") {
            p(class = "dark:text-white mx-5 text-center") {
                span(dangerously_set_inner_html = &t!("footer.copyright", {
                    "years": COPYRIGHT_YEARS
                }))
            }
        }
    }
}
