use lazy_static::lazy_static;
use perseus::prelude::*;
use sycamore::prelude::*;

use super::greeting::{GreetingProps, GREETING};

lazy_static! {
    pub static ref WRAPPER: Capsule<GreetingProps> = get_capsule();
}

// A simple wrapper capsule to show how capsules can use capsules
fn wrapper_capsule(props: GreetingProps) -> View {
    view! {
        // Because `props` is an owned variable, it has to be cloned here
        (GREETING.widget( "", props.clone()))
    }
}

pub fn get_capsule() -> Capsule<GreetingProps> {
    Capsule::build(Template::build("wrapper"))
        .empty_fallback()
        .view(wrapper_capsule)
        .build()
}
