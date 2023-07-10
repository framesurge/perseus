mod error_views;
mod templates;

use perseus::{plugins::Plugins, prelude::*};
use sycamore::view;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
        .index_view(|cx| {
            view! { cx,
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        // Perseus automatically resolves `/.perseus/static/` URLs to the contents of the `static/` directory at the project root
                        link(rel = "stylesheet", href = ".perseus/static/styles/style.css")
                        // link(rel = "stylesheet", href = ".perseus/static/tailwind.css") // Deploy
                        link(rel = "stylesheet", href = ".perseus/static/tailwind.css") // Development
                        // link(rel = "icon", type="image/png", href = ".perseus/static/images/favicon/rust_tight_final.png")
                    }
                    body {
                        // Quirk: this creates a wrapper `<div>` around the root `<div>` by necessity
                        PerseusRoot()
                    }
                }
            }
        })
        .plugins(Plugins::new().plugin(
            perseus_tailwind::get_tailwind_plugin,
            perseus_tailwind::TailwindOptions {
                in_file: "src/tailwind.css".into(),
                // Don't put this in /static, it will trigger build loops.
                // Put this in /dist and use a static alias instead.
                out_file: "dist/tailwind.css".into(),
            },
        ))
        .static_alias("/tailwind.css", "dist/tailwind.css")
}
