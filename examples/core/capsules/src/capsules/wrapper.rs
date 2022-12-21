use lazy_static::lazy_static;
use perseus::prelude::*;
use sycamore::prelude::*;

use super::greeting::{GREETING, GreetingProps};

lazy_static! {
    pub static ref WRAPPER: Capsule<PerseusNodeType, GreetingProps> = get_capsule();
}

// A simple wrapper capsule to show how capsules can use capsules
fn wrapper_capsule<G: Html>(cx: Scope, props: GreetingProps) -> View<G> {
    view! { cx,
        // Because `props` is an owned variable, it has to be cloned here
        (GREETING.widget(cx, "", props.clone()))
    }
}

pub fn get_capsule<G: Html>() -> Capsule<G, GreetingProps> {
    Capsule::build(Template::build("greeting"))
        .empty_fallback()
        .view(wrapper_capsule)
        .build()
}
