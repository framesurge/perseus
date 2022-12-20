mod capsules;
mod templates;

use perseus::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .capsule(crate::capsules::greeting::get_capsule())
        .error_views(ErrorViews::unlocalized_development_default())
}
