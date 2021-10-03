use crate::components::container::NavLinks;
use crate::components::container::COPYRIGHT_YEARS;
use crate::templates::docs::generation::DocsVersionStatus;
use perseus::{link, t};
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;

#[derive(Clone)]
pub struct DocsContainerProps<G: GenericNode> {
    pub children: SycamoreTemplate<G>,
    pub docs_links: String,
    pub status: DocsVersionStatus,
}

#[component(DocsContainer<G>)]
pub fn docs_container(props: DocsContainerProps<G>) -> SycamoreTemplate<G> {
    let docs_links = props.docs_links.clone();
    let docs_links_2 = docs_links.clone();
    let status = props.status.clone();
    // TODO parse the status into a readable message (translation as well)

    let menu_open = Signal::new(false);
    // We need to verbatim copy the value because of how it's used in Sycamore's reactivity system
    let menu_open_2 = create_memo(cloned!((menu_open) => move || *menu_open.get()));
    let menu_open_3 = create_memo(cloned!((menu_open) => move || *menu_open.get()));
    let toggle_menu = cloned!((menu_open) => move |_| menu_open.set(!*menu_open.get()));

    template! {
        // TODO click-away events
        header(class = "shadow-md sm:p-2 w-full bg-white dark:text-white dark:bg-navy mb-20") {
            div(class = "flex justify-between") {
                a(class = "justify-self-start self-center m-3 ml-5 text-md sm:text-2xl", href = link!("/")) {
                    (t!("perseus"))
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
                        div(class = "dark:bg-white tham-inner") {}
                    }
                }
                // This displays the navigation links on desktop
                // But it needs to hide at the same time as the sidebar
                nav(class = "hidden md:flex") {
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
                    "md:hidden w-full text-center justify-center overflow-y-scroll {}",
                    if *menu_open_2.get() {
                        "flex flex-col"
                    } else {
                        "hidden"
                    }
                )
            ) {
                // TODO find a solution that lets you scroll here that doesn't need a fixed height
                div(class = "mr-5 overflow-y-scroll", style = "max-height: 500px") {
                    ul {
                        NavLinks()
                    }
                    hr()
                    div(class = "text-left p-3") {
                        div(class = "docs-links-markdown", dangerously_set_inner_html = &docs_links)
                    }
                }
            }
        }
        div(
            class = format!(
                "mt-14 xs:mt-16 sm:mt-20 lg:mt-25 overflow-y-auto {}",
                if !*menu_open_3.get() {
                    "flex"
                } else {
                    "hidden"
                }
            )
        ) {
            div(class = "flex w-full") {
                // The sidebar that'll display navigation through the docs
                div(class = "h-full hidden md:block max-w-xs w-full border-r") {
                    div(class = "mr-5") {
                        div(class = "text-left text-black dark:text-white p-3") {
                            aside(class = "docs-links-markdown", dangerously_set_inner_html = &docs_links_2)
                        }
                    }
                }
                div(class = "h-full flex w-full") {
                    // These styles were meticulously arrived at through pure trial and error...
                    div(class = "px-3 w-full sm:mr-auto sm:ml-auto sm:max-w-prose lg:max-w-3xl xl:max-w-4xl 2xl:max-w-5xl") {
                        (status.render())
                        main(class = "text-black dark:text-white") {
                            (props.children.clone())
                        }
                    }
                }
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
