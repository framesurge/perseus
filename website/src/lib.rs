#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod components;
mod error_pages;
mod templates;

use perseus::{Html, PerseusApp, PerseusRoot};

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(templates::index::get_template)
        .template(templates::comparisons::get_template)
        .template(templates::docs::get_template)
        .template(templates::plugins::get_template)
        .error_pages(error_pages::get_error_pages)
        .locales_and_translations_manager("en-US", &[])
        .index_view(|cx| {
            sycamore::view! { cx,
                html(class = "light") {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        link(rel = "stylesheet", href = ".perseus/static/tailwind.css")
                        link(rel = "stylesheet", href = ".perseus/static/styles/style.css")
                    }
                    body(class = "bg-white dark:bg-navy") {
                        PerseusRoot()
                    }
                }
            }
        })
}
