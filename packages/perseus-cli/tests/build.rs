use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;
use std::fs;

use crate::utils::init_test;

/// Makes sure that `perseus build` produces the correct artifacts.
///
/// This test is tightly coupled to the form of the static artifacts, and can
/// also act as a canary for some other problems.
#[test]
#[ignore]
fn build_produces_artifacts() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    init_test(&dir)?;

    // Add a stateless route with no explicit head to ensure Perseus still generates
    // the required page/head artifacts for direct loads.
    let mod_rs = dir.child("src/templates/mod.rs");
    let mod_contents = fs::read_to_string(mod_rs.path())?;
    fs::write(
        mod_rs.path(),
        format!("{}\npub mod about;\n", mod_contents.trim_end()),
    )?;

    fs::write(
        dir.child("src/templates/about.rs").path(),
        r#"use perseus::prelude::*;
use sycamore::prelude::*;

fn about_page() -> View {
    view! {
        div { "About Perseus!" }
    }
}

pub fn get_template() -> Template {
    Template::build("about").view(about_page).build()
}
"#,
    )?;

    let main_rs = dir.child("src/main.rs");
    let main_contents = fs::read_to_string(main_rs.path())?;
    fs::write(
        main_rs.path(),
        main_contents.replace(
            ".template(crate::templates::index::get_template())",
            ".template(crate::templates::index::get_template())\n        .template(crate::templates::about::get_template())",
        ),
    )?;

    // Build the app
    let mut cmd = crate::utils::perseus_cmd(&dir);

    cmd.arg("build");
    cmd.assert().success();

    // Assert on all the artifacts, based on the code in the `init` example
    dir.child("dist/render_conf.json")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine.d.ts")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine.js")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine_bg.wasm")
        .assert(predicate::path::exists());
    dir.child("dist/pkg/perseus_engine_bg.wasm.d.ts")
        .assert(predicate::path::exists());
    dir.child("dist/static/xx-XX-.html")
        // We don't assert any more than this due to hydration IDs and minification
        .assert(predicate::str::contains("Welcome to Perseus!"));
    dir.child("dist/static/xx-XX-.head.html")
        .assert(predicate::str::is_match("^<title>Welcome to Perseus!</title>$").unwrap());
    dir.child("dist/static/xx-XX-about.html")
        .assert(predicate::str::contains("About Perseus!"));
    dir.child("dist/static/xx-XX-about.head.html")
        .assert(predicate::path::exists());
    #[cfg(unix)] // It would have `.exe` on Windows
    dir.child("dist/target_engine/debug/my-app")
        .assert(predicate::path::exists());
    dir.child("dist/target_wasm/wasm32-unknown-unknown/debug/my-app.wasm")
        .assert(predicate::path::exists());

    Ok(())
}
