mod capsules;
mod templates;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .template(crate::templates::clock::get_template())
        .template(crate::templates::calc::get_template())
        .template(crate::templates::four::get_template())
        // We use the reference pattern here, storing the capsule in a static. However, we had
        // to specify a concrete type for `G`, the rendering backend. Since we used
        // `PerseusNodeType`, which will always intelligently line up with the `G` here, we
        // know they'll match, but the compiler doesn't. Unlike `.capsule()`,
        // `.capsule_ref()` can perform internal type coercions to bridge this
        // gap. (It learn more about the reference pattern vs. the function one, see the book.)
        .capsule_ref(&*crate::capsules::greeting::GREETING)
        .capsule_ref(&*crate::capsules::wrapper::WRAPPER)
        .capsule_ref(&*crate::capsules::ip::IP)
        .capsule_ref(&*crate::capsules::time::TIME)
        .capsule_ref(&*crate::capsules::number::NUMBER)
        .capsule_ref(&*crate::capsules::links::LINKS)
        .error_views(ErrorViews::unlocalized_development_default())
}
