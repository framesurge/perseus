use crate::cmd::run_cmd_directly;
use crate::errors::*;
use crate::parse::{InitOpts, NewOpts, Opts};
use std::fs;
use std::path::{Path, PathBuf};

/// Creates the named file with the given contents if it doesn't already exist,
/// printing a warning if it does.
fn create_file_if_not_present(
    filename: &Path,
    contents: &str,
    name: &str,
) -> Result<(), InitError> {
    let filename_str = filename.to_str().unwrap();
    if fs::metadata(filename).is_ok() {
        eprintln!("[WARNING]: Didn't create '{}', since it already exists. If you didn't mean for this to happen, you should remove this file and try again.", filename_str);
    } else {
        let contents = contents
            .replace("%name", name)
            .replace("%perseus_version", env!("CARGO_PKG_VERSION"));
        fs::write(filename, contents).map_err(|err| InitError::CreateInitFileFailed {
            source: err,
            filename: filename_str.to_string(),
        })?;
    }
    Ok(())
}

/// Initializes a new Perseus project in the given directory, based on either
/// the default template or one from a given URL.
pub fn init(dir: PathBuf, opts: &InitOpts) -> Result<i32, InitError> {
    // Create the basic directory structure (this will create both `src/` and
    // `src/templates/`)
    fs::create_dir_all(dir.join("src/templates"))
        .map_err(|err| InitError::CreateDirStructureFailed { source: err })?;
    // Now create each file
    create_file_if_not_present(&dir.join("Cargo.toml"), DFLT_INIT_CARGO_TOML, &opts.name)?;
    create_file_if_not_present(&dir.join(".gitignore"), DFLT_INIT_GITIGNORE, &opts.name)?;
    create_file_if_not_present(&dir.join("src/main.rs"), DFLT_INIT_MAIN_RS, &opts.name)?;
    create_file_if_not_present(
        &dir.join("src/templates/mod.rs"),
        DFLT_INIT_MOD_RS,
        &opts.name,
    )?;
    create_file_if_not_present(
        &dir.join("src/templates/index.rs"),
        DFLT_INIT_INDEX_RS,
        &opts.name,
    )?;

    // And now tell the user about some stuff
    println!("Your new app has been created! Run `perseus serve -w` to get to work! You can find more details, including about improving compilation speeds in the Perseus docs (https://arctic-hen7.github.io/perseus/en-US/docs/).");

    Ok(0)
}
/// Initializes a new Perseus project in a new directory that's a child of the
/// current one.
// The `dir` here is the current dir, the name of the one to create is in `opts`
pub fn new(dir: PathBuf, opts: &NewOpts, global_opts: &Opts) -> Result<i32, NewError> {
    // Create the directory (if the user provided a name explicitly, use that,
    // otherwise use the project name)
    let target = dir.join(opts.dir.as_ref().unwrap_or(&opts.name));

    // Check if we're using the default template or one from a URL
    if let Some(url) = &opts.template {
        let url_parts = url.split('@').collect::<Vec<&str>>();
        let engine_url = url_parts[0];
        // A custom branch can be specified after a `@`, or we'll use `stable`
        let cmd = format!(
            // We'll only clone the production branch, and only the top level, we don't need the
            // whole shebang
            "{} clone --single-branch {branch} --depth 1 {repo} {output}",
            global_opts.git_path,
            branch = if let Some(branch) = url_parts.get(1) {
                format!("--branch {}", branch)
            } else {
                String::new()
            },
            repo = engine_url,
            output = target.to_string_lossy()
        );
        println!(
            "Fetching custom initialization template with command: '{}'.",
            &cmd
        );
        // Tell the user what command we're running so that they can debug it
        let exit_code = run_cmd_directly(
            cmd,
            &dir, // We'll run this in the current directory and output into `.perseus/`
            vec![],
        )
        .map_err(|err| NewError::GetCustomInitFailed { source: err })?;
        if exit_code != 0 {
            return Err(NewError::GetCustomInitNonZeroExitCode { exit_code });
        }
        // Now delete the Git internals
        let git_target = target.join(".git");
        if let Err(err) = fs::remove_dir_all(&git_target) {
            return Err(NewError::RemoveCustomInitGitFailed {
                target_dir: git_target.to_str().map(|s| s.to_string()),
                source: err,
            });
        }
        Ok(0)
    } else {
        fs::create_dir(&target).map_err(|err| NewError::CreateProjectDirFailed { source: err })?;
        // Now initialize in there
        let exit_code = init(
            target,
            &InitOpts {
                name: opts.name.to_string(),
            },
        )?;
        Ok(exit_code)
    }
}

// --- BELOW ARE THE RAW FILES FOR DEFAULT INITIALIZATION ---
// The token `%name` in all of these will be replaced with the given project
// name
// NOTE: These must be updated for breaking changes

static DFLT_INIT_CARGO_TOML: &str = r#"[package]
name = "%name"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

# Dependencies for the engine and the browser go here
[dependencies]
perseus = { version = "=%perseus_version", features = [ "hydrate" ] }
sycamore = "^0.8.1"
serde = { version = "1", features = [ "derive" ] }
serde_json = "1"

# Engine-only dependencies go here
[target.'cfg(browser)'.dependencies]
tokio = { version = "1", features = [ "macros", "rt", "rt-multi-thread" ] }
perseus-warp = { version = "=%perseus_version", features = [ "dflt-server" ] }

# Browser-only dependencies go here
[target.'cfg(client)'.dependencies]"#;
static DFLT_INIT_GITIGNORE: &str = r#"dist/"#;
static DFLT_INIT_MAIN_RS: &str = r#"mod templates;

use perseus::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
}"#;
static DFLT_INIT_MOD_RS: &str = r#"pub mod index;"#;
static DFLT_INIT_INDEX_RS: &str = r#"use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
            h1 { "Welcome to Perseus!" }
            p {
                "This is just an example app. Try changing some code inside "
                code { "src/templates/index.rs" }
                " and you'll be able to see the results here!"
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}"#;
