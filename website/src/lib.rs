mod components;
mod error_pages;
mod templates;

use perseus::{Html, PerseusApp, PerseusRoot, Plugins};
use perseus_size_opt::{perseus_size_opt, SizeOpts};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(templates::index::get_template)
        .template(templates::comparisons::get_template)
        .template(templates::docs::get_template)
        .template(templates::plugins::get_template)
        .error_pages(error_pages::get_error_pages)
        .locales_and_translations_manager("en-US", &[])
        .plugins(Plugins::new().plugin(
            perseus_size_opt,
            // Because we're using Rust 2018, we can take advantage of more aggressive size optimizations on `fluent_bundle`
            SizeOpts::default_2018(),
        ))
        .index_view(|| {
            sycamore::view! {
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
