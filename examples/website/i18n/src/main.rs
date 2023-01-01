use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(Template::build("index").view(index_page).build())
        // EXCERPT_START
        .locales_and_translations_manager(
            "en-US",             // Default locale
            &["fr-FR", "es-ES"], // Other supported locales
        )
    // EXCERPT_END
}

// EXCERPT_START
// Our landing page. Going to `/` will cause a redirect to `/en-US`,
// `/es-ES`, or `/fr-FR` based on the user's locale settings in their browser,
// all automatically. If nothing matches, the default locale (`en-US`) will be
// used.
fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        h1 { (t!("greeting", cx)) }
    }
}

// `translations/en-US.ftl`:
//      greeting = Hello, world!
// `translations/es-ES.ftl`:
//      greeting = ¡Hola, mundo!
// `translations/fr-FR.ftl`:
//      greeting = Bonjour, le monde!
// EXCERPT_END
