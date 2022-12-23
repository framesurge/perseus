use crate::capsules::ip::IP;
use crate::capsules::links::LINKS;
use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // This will display the user's IP address
        (IP.widget(cx, "", ()))
        (LINKS.widget(cx, "", ()))
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about")
        .view(about_page)
        // This is extremely important. Notice that this template doesn't have any state of its own?
        // Well, that means it should be able to be built at build-time! However, the `ip`
        // capsule uses request state, which means anything that uses it also has to be built at
        // request-time. That means Perseus needs to 'reschedule' the build of this page from
        // build-time to request-time. This can incur a performance penalty for users of your site
        // (as they'll have to wait for the server to generate the `ip` capsule's state, rather then
        // just sending them some pre-generated HTML), so Perseus makes sure it has your permission
        // first. Try commenting out this line, the app will fail to build.
        .allow_rescheduling()
        .build()
}
