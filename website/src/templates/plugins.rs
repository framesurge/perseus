// The structure of the `plugins` directory is to have a subdirectory for each
// locale, and then a list of plugins inside Note that the same plugins must be
// defined for every locale

use crate::components::container::Container;
use crate::components::header::HeaderProps;
use crate::components::trusted_svg::TRUSTED_SVG;
#[cfg(engine)]
use crate::Error;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(engine)]
use std::fs;
use sycamore::prelude::*;
#[cfg(engine)]
use walkdir::WalkDir;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Serialize, Deserialize, Clone, UnreactiveState)]
struct PluginsPageProps {
    /// The list of plugins with minimal details. These will be displayed in
    /// cards on the index page.
    plugins: Vec<PluginDetails>,
}
/// The minimal amount of details for a plugin, which will be displayed in a
/// card on the root page. This is a subset of `PluginDetails` (except for the
/// `slug`). This needs to be `Eq` for Sycamore's keyed list diffing algorithm.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
struct PluginDetails {
    /// The plugins's name.
    name: String,
    /// A short description of the plugin.
    description: String,
    /// The author of the plugin.
    author: String,
    /// Whether or not the plugin is trusted by the Perseus development team.
    /// Note that this is just a superficial measure, and it does not indicate
    /// security, audit status, or anything else of the like. It should NOT be
    /// relied on when deciding whether or not a plugin is secure!
    trusted: bool,
    /// The plugin's home URL, which the plugins registry will redirect the user
    /// to. This avoids developers having to update documentation in many places
    /// or ask for the website to be rebuilt every time their READMEs change.
    url: String,
}

fn plugins_page<G: Html>(cx: Scope, props: PluginsPageProps) -> View<G> {
    let plugins = create_signal(cx, props.plugins);
    // This will store the plugins relevant to the user's search (all of them by
    // This stores the search that the user provides
    let filter = create_signal(cx, String::new());
    // A derived state that will filter the plugins that the user searches for
    let filtered_plugins = create_memo(cx, || {
        plugins
            .get()
            .iter()
            .filter(|plugin| {
                let filter_text = &*filter.get().to_lowercase();
                plugin.name.to_lowercase().contains(filter_text)
                    || plugin.author.to_lowercase().contains(filter_text)
                    || plugin.description.to_lowercase().contains(filter_text)
            })
            .cloned()
            .collect::<Vec<PluginDetails>>()
    });

    view! { cx,
        Container(
            header = HeaderProps {
                title: t!("perseus", cx),
                text_color: "text-black dark:text-white".to_string(),
                menu_color: "bg-black dark:bg-white".to_string(),
                mobile_nav_extension: View::empty(),
                menu_open: None,
            },
            footer = true,
        ) {
                div(class = "mt-14 xs:mt-16 sm:mt-20 lg:mt-25 dark:text-white") {
                    div(class = "w-full flex flex-col justify-center text-center") {
                        h1(class = "text-5xl xs:text-7xl sm:text-8xl font-bold mb-5") { (t!("plugins-title", cx)) }
                        br()
                        p(class = "mx-1 mb-2") { (t!("plugins-desc", cx)) }
                        div(class = "w-full flex justify-center text-center mb-3") {
                            input(class = "mx-2 max-w-7xl p-3 rounded-md border border-indigo-500 focus:outline-indigo-600 dark:focus:outline-indigo-700 search-bar-bg", on:input = |ev: web_sys::Event| {
                                // This longwinded code gets the actual value that the user typed in
                                let target: HtmlInputElement = ev.target().unwrap().unchecked_into();
                                let new_input = target.value();
                                filter.set(new_input);
                            }, placeholder = t!("plugin-search.placeholder", cx))
                        }
                    }
                    div(class = "w-full flex justify-center") {
                        ul(class = "text-center w-full max-w-7xl mx-2 mb-16") {
                            Indexed(
                                iterable = filtered_plugins,
                                view = |cx, plugin| view! { cx,
                                    li(class = "inline-block align-top m-2") {
                                        a(
                                            class = "block text-left cursor-pointer rounded-xl shadow-md hover:shadow-2xl transition-shadow duration-100 p-8 max-w-sm dark:text-white",
                                            href = &plugin.url // This is an external link to the plugin's homepage
                                        ) {
                                            p(class = "text-xl xs:text-2xl inline-flex") {
                                                (plugin.name)
                                                (if plugin.trusted {
                                                    view! { cx,
                                                        div(class = "ml-1 self-center", dangerously_set_inner_html = TRUSTED_SVG)
                                                    }
                                                } else {
                                                    View::empty()
                                                })
                                            }
                                            p(class = "text-sm text-gray-500 dark:text-gray-300 mb-1") { (t!("plugin-card-author", { "author" = &plugin.author }, cx)) }
                                            p { (plugin.description) }
                                        }
                                    }
                                }
                            )
                        }
                    }
                }
            }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { (format!("{} | {}", t!("plugins-title", cx), t!("perseus", cx))) }
        link(rel = "stylesheet", href = ".perseus/static/styles/markdown.css")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("plugins")
        .view_with_unreactive_state(plugins_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { locale, .. }: StateGeneratorInfo<()>,
) -> Result<PluginsPageProps, BlamedError<Error>> {
    // This is the root page, so we want a list of plugins and a small amount of
    // information about each This directory loop is relative to `.perseus/`
    let mut plugins = Vec::new();
    for entry in WalkDir::new(&format!("plugins/{}", locale)) {
        let entry = entry.map_err(Error::from)?;
        let path = entry.path();
        // Ignore any empty directories or the like
        if path.is_file() {
            // Get the JSON contents and parse them as plugin details
            let contents = fs::read_to_string(&path).map_err(Error::from)?;
            let details = serde_json::from_str::<PluginDetails>(&contents).map_err(Error::from)?;

            plugins.push(details);
        }
    }

    Ok(PluginsPageProps { plugins })
}
