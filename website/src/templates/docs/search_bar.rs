// use perseus::t;
use sycamore::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::{Event, KeyboardEvent};

// #[component]
// pub fn SearchBar() -> View {
//     let search = create_signal(String::new());

//     view! {
//         input(
//             class = "p-2 border rounded-md mb-2 focus:outline-indigo-500
//                 let event: KeyboardEvent = ev.unchecked_into();
//                 if event.key() == "Enter" {
//                     let search = search.get();
//                     if !search.is_empty() {
//                         #[cfg(target_arch = "wasm32")]
//                         search_site(&search);
//                     }
//                 }
//             }
//         )
//     }
// }

#[component]
pub fn SearchBar() -> View {
    View::default()
}

// /// Searches the site using Google as a proxy.
// // BUG This should be Framesurge instead, but search engines have been slow
// to index... #[cfg(target_arch = "wasm32")]
// fn search_site(search: &str) {
//     use js_sys::encode_uri_component;

//     let search_query = format!("site:framesurge.sh/perseus/en-US/docs
// {}", search);     let search_query =
// encode_uri_component(&search_query).to_string();     let search_url = format!("https://google.com/search?q={}", search_query);
//     // Open that in a new tab
//     let window = web_sys::window().unwrap();
//     window
//         .open_with_url_and_target(&search_url, "_blank")
//         .unwrap();
// }
