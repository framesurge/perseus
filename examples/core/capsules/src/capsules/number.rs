use std::num::ParseIntError;

use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// This is a fairly pointless capsule that uses incremental generation to
// generate widgets for numbers, which it then displays. This shows how errors
// work in capsules (by passing through a non-number).

lazy_static! {
    pub static ref NUMBER: Capsule<PerseusNodeType, ()> = get_capsule();
}

// Note the use of props as `()`, indicating that this capsule doesn't take any
// properties
fn time_capsule<G: Html>(cx: Scope, state: Number, _props: ()) -> View<G> {
    view! { cx,
        span { (state.number) }
        // This is an example to demonstrate self-recursion, as well as taking
        // a particular incremental path that has incremental dependencies
        // itself. Perseus resolves this without problems.
        (if state.number == 5 {
            view! { cx, (NUMBER.widget(cx, "/6", ())) }
        } else {
            View::empty()
        })
    }
}

#[derive(Serialize, Deserialize, Clone, UnreactiveState)]
struct Number {
    number: u16,
}

pub fn get_capsule<G: Html>() -> Capsule<G, ()> {
    Capsule::build(
        Template::build("number")
            .build_paths_fn(get_build_paths)
            .build_state_fn(get_build_state)
            .incremental_generation(),
    )
    .empty_fallback()
    .view_with_unreactive_state(time_capsule)
    .build()
}

#[engine_only_fn]
async fn get_build_state(
    info: StateGeneratorInfo<()>,
) -> Result<Number, BlamedError<ParseIntError>> {
    // The path should be a simple number
    let number = if info.path.contains('/') {
        // Easter egg! We'll add multiple numbers together
        let mut final_num = 0;
        for num in info.path.split('/') {
            let parsed = num.parse::<u16>().map_err(|e| BlamedError {
                error: e,
                blame: ErrorBlame::Client(None),
            })?;
            final_num += parsed;
        }
        final_num
    } else {
        // If the number is invalid, that's the client's fault
        info.path.parse::<u16>().map_err(|e| BlamedError {
            error: e,
            blame: ErrorBlame::Client(None),
        })?
    };

    Ok(Number { number })
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["4".to_string()],
        extra: ().into(),
    }
}
