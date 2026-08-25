use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;

use crate::utils::init_test;

/// Makes sure that `perseus export-error-page` produces the correct error page.
#[test]
#[ignore]
fn export_error_page_produces_page() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    init_test(&dir)?;

    // Build the app
    let mut cmd = crate::utils::perseus_cmd(&dir);

    cmd.arg("build");
    cmd.assert().success();

    let mut cmd = crate::utils::perseus_cmd(&dir);

    cmd.arg("export-error-page")
        .arg("--code")
        .arg("404")
        .arg("--output")
        .arg("404.html");
    cmd.assert().success();

    dir.child("404.html")
        // Going off the default development error pages
        .assert(predicate::str::contains("Page not found!"));

    Ok(())
}
