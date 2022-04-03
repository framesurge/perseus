// This build script copies the `examples/core/basic/.perseus/` directory into `packages/perseus-cli/` for use in compilation
// Having this as a build script rather than an external script allows the CLI to be installed with `cargo install` from any commit hash

use std::fs;
use std::path::PathBuf;

// All this is run relative to the `packages/perseus-cli/` directory
fn main() {
    // Tell Cargo that this needs to be re-run if the direcotry storing the engine code has changed
    println!("cargo:rerun-if-changed=../../examples/core/basic/.perseus");

    let dest = PathBuf::from(".");
    let engine_dir = PathBuf::from("../../examples/core/basic/.perseus");

    // Replace the current `.perseus/` directory here with the latest version
    let _ = fs::remove_dir_all(dest.join(".perseus")); // It's fine if this doesn't exist
    fs_extra::dir::copy(engine_dir, &dest, &fs_extra::dir::CopyOptions::new()).unwrap();
    // Rename the manifests for appropriate usage
    fs::rename(
        dest.join(".perseus/Cargo.toml"),
        dest.join(".perseus/Cargo.toml.old"),
    )
    .unwrap();
    fs::rename(
        dest.join(".perseus/server/Cargo.toml"),
        dest.join(".perseus/server/Cargo.toml.old"),
    )
    .unwrap();
    fs::rename(
        dest.join(".perseus/builder/Cargo.toml"),
        dest.join(".perseus/builder/Cargo.toml.old"),
    )
    .unwrap();
    // Remove distribution artifacts so they don't clog up the final bundle
    fs::remove_dir_all(dest.join(".perseus/dist")).unwrap();
    // But we need to create the basic directory structure for outputs
    fs::create_dir(dest.join(".perseus/dist")).unwrap();
    fs::create_dir(dest.join(".perseus/dist/static")).unwrap();
    fs::create_dir(dest.join(".perseus/dist/exported")).unwrap();
    // Replace the example's package name with a token the CLI can use (compatible with alternative engines as well)
    // We only need to do this in the root package, the others depend on it
    // While we're at it, we'll update the dependencies to be tokens that can be replaced by the CLI (removing relative path references)
    let updated_root_manifest = fs::read_to_string(dest.join(".perseus/Cargo.toml.old"))
        .unwrap()
        .replace("perseus-example-basic", "USER_PKG_NAME")
        .replace("path = \"../../../../packages/perseus\"", "PERSEUS_VERSION");
    fs::write(dest.join(".perseus/Cargo.toml.old"), updated_root_manifest).unwrap();
    let updated_builder_manifest = fs::read_to_string(dest.join(".perseus/builder/Cargo.toml.old"))
        .unwrap()
        .replace(
            "path = \"../../../../../packages/perseus\"",
            "PERSEUS_VERSION",
        );
    fs::write(
        dest.join(".perseus/builder/Cargo.toml.old"),
        updated_builder_manifest,
    )
    .unwrap();
    let updated_server_manifest = fs::read_to_string(dest.join(".perseus/server/Cargo.toml.old"))
        .unwrap()
        .replace(
            "path = \"../../../../../packages/perseus\"",
            "PERSEUS_VERSION",
        )
        .replace(
            "path = \"../../../../../packages/perseus-actix-web\"",
            "PERSEUS_ACTIX_WEB_VERSION",
        )
        .replace(
            "path = \"../../../../../packages/perseus-warp\"",
            "PERSEUS_WARP_VERSION",
        );
    fs::write(
        dest.join(".perseus/server/Cargo.toml.old"),
        updated_server_manifest,
    )
    .unwrap();
}

/*
[
    # The CLI needs the `.perseus/` directory copied in for packaging (and we need to rename `Cargo.toml` to `Cargo.toml.old`)
    "cd packages/perseus-cli",
    "rm -rf ./.perseus",
    "cp -r ../../examples/core/basic/.perseus/ .perseus/",
    "mv .perseus/Cargo.toml .perseus/Cargo.toml.old",
    "mv .perseus/server/Cargo.toml .perseus/server/Cargo.toml.old",
    "mv .perseus/builder/Cargo.toml .perseus/builder/Cargo.toml.old",
    # Remove distribution artifacts (they clog up the final bundle)
    "rm -rf .perseus/dist",
    "mkdir -p .perseus/dist",
    "mkdir -p .perseus/dist/static",
    "mkdir -p .perseus/dist/exported",
    # Replace the example's package name with a token the CLI can use (compatible with alternative engines as well)
    # We only need to do this in the root package, the others depend on it
    "sed -i 's/perseus-example-basic/USER_PKG_NAME/' .perseus/Cargo.toml.old",
    # Replace the relative path references with tokens too
    "sed -i 's/path = \"\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/packages\\/perseus\"/PERSEUS_VERSION/' .perseus/Cargo.toml.old",
    "sed -i 's/path = \"\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/packages\\/perseus\"/PERSEUS_VERSION/' .perseus/builder/Cargo.toml.old",
    # These will need to be updated as more integrations are added
    "sed -i 's/path = \"\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/packages\\/perseus\"/PERSEUS_VERSION/' .perseus/server/Cargo.toml.old",
    "sed -i 's/path = \"\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/packages\\/perseus-actix-web\"/PERSEUS_ACTIX_WEB_VERSION/' .perseus/server/Cargo.toml.old",
    "sed -i 's/path = \"\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/\\.\\.\\/packages\\/perseus-warp\"/PERSEUS_WARP_VERSION/' .perseus/server/Cargo.toml.old"
]
*/
