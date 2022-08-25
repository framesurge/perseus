use sycamore::prelude::*;
use perseus::t;

pub static COPYRIGHT_YEARS: &str = "2021";

#[derive(Prop)]
pub struct FooterProps {
    pub classes: String,
}

/// The footer for the entire app, which can be styled arbitrarily.
#[component]
pub fn Footer<G: Html>(cx: Scope, FooterProps { classes }: FooterProps) -> View<G> {
    view! { cx,
        footer(
            class = format!(
                "w-full flex justify-center py-5 {}",
                &classes
            )
        ) {
            p(class = "mx-5 text-center") {
                span(dangerously_set_inner_html = &t!("footer.copyright", {
                    "years" = COPYRIGHT_YEARS
                }, cx))
            }
        }
    }
}
