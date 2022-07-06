mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};

// pub fn get_app<G: Html>() -> PerseusApp<G> {
//     PerseusApp::new()
//         .template(crate::templates::index::get_template)
//         .template(crate::templates::about::get_template)
//         .error_pages(crate::error_pages::get_error_pages)
// }

// #[perseus::engine_main]
// async fn main() {
//     use perseus::builder::{get_op, run_dflt_engine};

//     let op = get_op().unwrap();
//     let exit_code = run_dflt_engine(op, get_app,
// perseus_warp::dflt_server).await;     std::process::exit(exit_code);
// }

// #[perseus::browser_main]
// pub fn main() -> perseus::ClientReturn {
//     use perseus::run_client;

//     run_client(get_app)
// }

// Note: we use fully-qualified paths in the types to this function so we don't
// have to target-gate some more imports
#[cfg(not(target_arch = "wasm32"))] // We only have access to `warp` etc. on the engine-side, so this function
                                    // should only exist there
pub async fn dflt_server<
    M: perseus::stores::MutableStore + 'static,
    T: perseus::internal::i18n::TranslationsManager + 'static,
>(
    props: perseus::internal::serve::ServerProps<M, T>,
    (host, port): (String, u16),
) {
    use perseus_warp::perseus_routes;
    use std::net::SocketAddr;
    use warp::Filter;

    // The Warp integration takes a `SocketAddr`, so we convert the given host and
    // port into that format
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    // Now, we generate the routes from the properties we were given
    // All integrations provide some function for setting them up that just takes
    // those universal properties Usually, you shouldn't ever have to worry
    // about the value of the properties, which are set from your `PerseusApp`
    // config
    let perseus_routes = perseus_routes(props).await;
    // And now set up our own routes
    // You could set up as many of these as you like in a production app
    // Note that they mustn't define anything under `/.perseus` or anything
    // conflicting with any of your static aliases This one will just echo
    // whatever is sent to it
    let api_route = warp::path!("api" / "echo" / String).map(|msg| {
        // You can do absolutely anything in here that you can do with Warp as usual
        msg
    });
    // We could add as many routes as we wanted here, but the Perseus routes, no
    // matter what integration you're using, MUST always come last! This is
    // because they define a wildcard handler for pages, which has to be defined
    // last, or none of your routes will do anything.
    let routes = api_route.or(perseus_routes);

    warp::serve(routes).run(addr).await;

    // If you try interacting with the app as usual, everything will work fine
    // If you try going to `/api/echo/test`, you'll get `test` printed back to
    // you! Try replacing `test` with anything else and it'll print whatever
    // you put in back to you!
}

#[perseus::main(dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
}
