use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::prelude::*;

use crate::utils::init_test;

/// Makes sure that `perseus clean` removes the `dist/` directory entirely after
/// a build.
#[test]
#[ignore]
fn clean_removes_dist() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    init_test(&dir)?;

    // Build the app and make sure `clean` removes the `dist/` directory
    let mut cmd = crate::utils::perseus_cmd(&dir);

    cmd.arg("build");
    cmd.assert().success();

    // The render config will always be produced
    dir.child("dist/render_conf.json")
        .assert(predicate::path::exists());

    let mut cmd = crate::utils::perseus_cmd(&dir);

    cmd.arg("clean");
    cmd.assert().success();

    dir.child("dist").assert(predicate::path::missing());

    Ok(())
}
