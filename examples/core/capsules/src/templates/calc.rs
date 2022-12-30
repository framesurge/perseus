use crate::capsules::links::LINKS;
use crate::capsules::number::NUMBER;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[auto_scope]
fn calc_page<G: Html>(cx: Scope, state: &CalcStateRx) -> View<G> {
    view! { cx,
        // This was *not* built at build-time in `number`, so we're incrementally
        // generating it. Importantly, Perseus can figure out that this should just
        // be added to the build paths list of the `number` widget, so we don't need
        // to reschedule the building of this widget
        p {
            "The number fifty-six: "
            // See `number.rs` for why this yields `56`
            (NUMBER.widget(cx, "/5", ()))
            "."
        }
        // Now, let me be clear. Using a widget as an addition function is a woeful abuse
        // of Perseus, but it does serve as an excellent example for how powerful widgets really
        // are. This is using incremental generation, and the user is simply providing the last
        // number to add to the present sum (42, of course). Every change to the input will trigger
        // a request to the server, which will generate the appropriate widget (aka. sum) and cache
        // it (meaning future requests will just return that). Note that the browser also performs some
        // caching, so, if you try typing, say, `3`, and then `33`, and then backspacing the last `3`
        // so you're back to just `3`, you'll notice in the DevTools that there are no new requests,
        // since you've already typed in `3` before.
        //
        // This works because *everything* is reactive, literally everything.
        p {
            "The sum of the state numbers: "
            (NUMBER.widget(
                cx,
                // We're using this widget as a glorified addition function
                &format!(
                    "/{}/{}",
                    // We need to make them strings first
                    state
                        .numbers
                        .get()
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join("/"),
                    state.user_number.get()
                ),
                ()
            ))
            "."
        }
        p { "Type your number below..." }
        input(bind:value = state.user_number) {}
        (LINKS.widget(cx, "", ()))
    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "CalcStateRx")]
struct CalcState {
    numbers: Vec<u16>,
    // This has to be a string to work with `bind:value`
    user_number: String,
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("calc")
        .view_with_state(calc_page)
        .build_state_fn(get_build_state)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> CalcState {
    CalcState {
        numbers: vec![5, 10, 27],
        // This can be modified by the user
        user_number: "0".to_string(),
    }
}
