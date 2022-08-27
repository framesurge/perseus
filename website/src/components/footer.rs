use sycamore::prelude::*;
use perseus::t;

static COPYRIGHT_YEARS: &str = "2021-2022";

/// The footer for the entire app, which can be styled arbitrarily.
#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        footer(
            class = "w-full flex justify-center py-5 bg-black text-white"

        ) {
            p(class = "mx-5 text-center") {
                span(dangerously_set_inner_html = &t!("footer.copyright", {
                    "years" = COPYRIGHT_YEARS
                }, cx))
            }
        }
    }
}
