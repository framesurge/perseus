// The structure of the `plugins` directory is to have a subdirectory for each
// locale, and then a list of plugins inside Note that the same plugins must be
// defined for every locale

use crate::components::container::{Container, ContainerProps};
use crate::components::trusted_svg::TRUSTED_SVG;
use perseus::{t, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::fs;
use sycamore::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use walkdir::WalkDir;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

macro_rules! t {
    // When there are no arguments to interpolate
    ($id:expr, $cx:expr) => {
        perseus::internal::i18n::t_macro_backend($id, $cx)
    };
    // When there are arguments to interpolate
    ($id:expr, {
        $($key:literal = $value:expr),+
    }, $cx:expr) => {{
        let mut args = perseus::internal::i18n::TranslationArgs::new();
        $(
            args.set($key, $value);
        )+
        perseus::internal::i18n::t_macro_backend_with_args($id, args, $cx)
    }};
}

#[derive(Serialize, Deserialize)]
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

#[perseus::template(PluginsPage)]
#[component(PluginsPage<G>)]
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
        Container(ContainerProps {
            title: t!("perseus", cx),
            children: view! { cx,
                div(class = "mt-14 xs:mt-16 sm:mt-20 lg:mt-25 dark:text-white") {
                    div(class = "w-full flex flex-col justify-center text-center") {
                        h1(class = "text-5xl xs:text-7xl sm:text-8xl font-extrabold mb-5") { (t!("plugins-title", cx)) }
                        br()
                        p(class = "mx-1 mb-2") { (t!("plugins-desc", cx)) }
                        div(class = "w-full flex justify-center text-center mb-3") {
                            input(class = "mx-2 max-w-7xl p-3 rounded-lg border-2 border-indigo-600 dark:bg-navy", on:input = |ev: web_sys::Event| {
                                // This longwinded code gets the actual value that the user typed in
                                let target: HtmlInputElement = ev.target().unwrap().unchecked_into();
                                let new_input = target.value();
                                filter.set(new_input);
                            }, placeholder = t!("plugin-search.placeholder", cx))
                        }
                    }
                    div(class = "w-full flex justify-center") {
                        ul(class = "text-center w-full max-w-7xl mx-2 mb-16") {
                            Indexed {
                                iterable: filtered_plugins,
                                view: |cx, plugin| view! { cx,
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
                                            p(class = "text-sm text-gray-500 dark:text-gray-300 mb-1") { (t!("plugin-card-author", { "author" = plugin.author.clone() }, cx)) }
                                            p { (plugin.description) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

#[perseus::head]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { (format!("{} | {}", t!("plugins-title", cx), t!("perseus", cx))) }
        link(rel = "stylesheet", href = ".perseus/static/styles/markdown.css")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("plugins")
        .template(plugins_page)
        .head(head)
        .build_state_fn(get_build_state)
}

#[perseus::build_state]
async fn get_build_state(
    _path: String,
    locale: String,
) -> RenderFnResultWithCause<PluginsPageProps> {
    // This is the root page, so we want a list of plugins and a small amount of
    // information about each This directory loop is relative to `.perseus/`
    let mut plugins = Vec::new();
    for entry in WalkDir::new(&format!("plugins/{}", locale)) {
        let entry = entry?;
        let path = entry.path();
        // Ignore any empty directories or the like
        if path.is_file() {
            // Get the JSON contents and parse them as plugin details
            let contents = fs::read_to_string(&path)?;
            let details = serde_json::from_str::<PluginDetails>(&contents)?;

            plugins.push(details);
        }
    }

    Ok(PluginsPageProps { plugins })
}
