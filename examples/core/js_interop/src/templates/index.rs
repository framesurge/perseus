use perseus::{Html, Template};
use sycamore::prelude::{view, Scope, View};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // We'll use JS to change this message manually
        p(id = "message") { "Hello World!" }
        button(id = "change-message", on:click = |_| {
            #[cfg(target_arch = "wasm32")]
            change_message()
        }) { "Change message with JS" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page)
}

// Of course, JS will only run in the browser, so this should be browser-only
#[cfg(target_arch = "wasm32")]
// This path should be relative to the root of your project
// That file will then be hosted behind `/.perseus/` and automatically fetched as needed
#[wasm_bindgen(module = "/src/changeMessage.js")]
extern "C" {
    #[wasm_bindgen(js_name = "changeMessage")]
    fn change_message();
}
