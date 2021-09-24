use crate::cmd::{cfg_spinner, run_stage};
use crate::errors::*;
use crate::thread::{spawn_thread, ThreadHandle};
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// Emojis for stages
static EXPORTING: Emoji<'_, '_> = Emoji("📦", "");
static BUILDING: Emoji<'_, '_> = Emoji("🏗️ ", ""); // Yes, there's a space here, for some reason it's needed...

/// Returns the exit code if it's non-zero.
macro_rules! handle_exit_code {
    ($code:expr) => {
        let (_, _, code) = $code;
        if code != 0 {
            return $crate::errors::Result::Ok(code);
        }
    };
}

/// An internal macro for copying files into the export package. The `from` and `to` that this accepts should be extensions of the
/// `target`, and they'll be `.join()`ed on.
macro_rules! copy_file {
    ($from:expr, $to:expr, $target:expr) => {
        if let Err(err) = fs::copy($target.join($from), $target.join($to)) {
            bail!(ErrorKind::MoveExportAssetFailed(
                $from.to_string(),
                $to.to_string(),
                err.to_string()
            ));
        }
    };
}

/// Finalizes the export by copying assets. This is very different from the finalization process of normal building.
pub fn finalize_export(target: &Path) -> Result<()> {
    // Move the `pkg/` directory into `dist/pkg/` as usual
    let pkg_dir = target.join("dist/pkg");
    if pkg_dir.exists() {
        if let Err(err) = fs::remove_dir_all(&pkg_dir) {
            bail!(ErrorKind::MovePkgDirFailed(err.to_string()));
        }
    }
    // The `fs::rename()` function will fail on Windows if the destination already exists, so this should work (we've just deleted it as per https://github.com/rust-lang/rust/issues/31301#issuecomment-177117325)
    if let Err(err) = fs::rename(target.join("pkg"), target.join("dist/pkg")) {
        bail!(ErrorKind::MovePkgDirFailed(err.to_string()));
    }

    // Copy files over (the directory structure should already exist from exporting the pages)
    copy_file!(
        "dist/pkg/perseus_cli_builder.js",
        "dist/exported/.perseus/bundle.js",
        target
    );
    copy_file!(
        "dist/pkg/perseus_cli_builder_bg.wasm",
        "dist/exported/.perseus/bundle.wasm",
        target
    );
    // Copy any JS snippets over (if the directory doesn't exist though, don't do anything)
    // This takes a target of the `dist/` directory, and then extends on that
    fn copy_snippets(ext: &str, parent: &Path) -> Result<()> {
        // We read from the parent directory (`.perseus`), extended with `ext`
        if let Ok(snippets) = fs::read_dir(&parent.join(ext)) {
            for file in snippets {
                let path = match file {
                    Ok(file) => file.path(),
                    Err(err) => bail!(ErrorKind::MoveExportAssetFailed(
                        "js snippet".to_string(),
                        "exportable js snippet".to_string(),
                        err.to_string()
                    )),
                };
                // Recurse on any directories and copy any files
                if path.is_dir() {
                    // We continue to pass on the parent, but we add the filename of this directory to the extension
                    copy_snippets(
                        &format!("{}/{}", ext, path.file_name().unwrap().to_str().unwrap()),
                        parent,
                    )?;
                } else {
                    // `ext` holds the folder structure of this file, which we'll preserve
                    // We must remove the prefix though (which is hardcoded in the initial invocation of this function)
                    let dir_tree = ext.strip_prefix("dist/pkg/snippets").unwrap();
                    // This is to avoid `//`
                    let dir_tree = if dir_tree.is_empty() {
                        String::new()
                    } else if dir_tree.starts_with('/') {
                        dir_tree.to_string()
                    } else {
                        format!("/{}", dir_tree)
                    };
                    let filename = path.file_name().unwrap().to_str().unwrap();
                    let final_dir_tree =
                        parent.join(format!("dist/exported/.perseus/snippets{}", dir_tree));
                    let path_to_copy_to = parent.join(&format!(
                        "dist/exported/.perseus/snippets{}/{}",
                        dir_tree, filename
                    ));
                    // Create the directory structure needed for this
                    if fs::create_dir_all(&final_dir_tree).is_err() {
                        bail!(ErrorKind::ExportDirStructureFailed);
                    }
                    copy_file!(
                        path.to_str().unwrap(),
                        path_to_copy_to.to_str().unwrap(),
                        parent
                    );
                }
            }
        }

        Ok(())
    }
    copy_snippets("dist/pkg/snippets", target)?;

    Ok(())
}

/// Actually exports the user's code, program arguments having been interpreted. This needs to know how many steps there are in total
/// because the serving logic also uses it. This also takes a `MultiProgress` to interact with so it can be used truly atomically.
/// This returns handles for waiting on the component threads so we can use it composably.
#[allow(clippy::type_complexity)]
pub fn export_internal(
    dir: PathBuf,
    spinners: &MultiProgress,
    num_steps: u8,
) -> Result<(
    ThreadHandle<impl FnOnce() -> Result<i32>, Result<i32>>,
    ThreadHandle<impl FnOnce() -> Result<i32>, Result<i32>>,
)> {
    let target = dir.join(".perseus");

    // Exporting pages message
    let ep_msg = format!(
        "{} {} Exporting your app's pages",
        style(format!("[1/{}]", num_steps)).bold().dim(),
        EXPORTING
    );
    // Wasm building message
    let wb_msg = format!(
        "{} {} Building your app to Wasm",
        style(format!("[2/{}]", num_steps)).bold().dim(),
        BUILDING
    );

    // We parallelize the first two spinners (static generation and Wasm building)
    // We make sure to add them at the top (the server spinner may have already been instantiated)
    let ep_spinner = spinners.insert(0, ProgressBar::new_spinner());
    let ep_spinner = cfg_spinner(ep_spinner, &ep_msg);
    let ep_target = target.clone();
    let wb_spinner = spinners.insert(1, ProgressBar::new_spinner());
    let wb_spinner = cfg_spinner(wb_spinner, &wb_msg);
    let wb_target = target;
    let ep_thread = spawn_thread(move || {
        handle_exit_code!(run_stage(
            vec![&format!(
                "{} run --bin perseus-exporter",
                env::var("PERSEUS_CARGO_PATH").unwrap_or_else(|_| "cargo".to_string())
            )],
            &ep_target,
            &ep_spinner,
            &ep_msg
        )?);

        Ok(0)
    });
    let wb_thread = spawn_thread(move || {
        handle_exit_code!(run_stage(
            vec![&format!(
                "{} build --target web",
                env::var("PERSEUS_WASM_PACK_PATH").unwrap_or_else(|_| "wasm-pack".to_string())
            )],
            &wb_target,
            &wb_spinner,
            &wb_msg
        )?);

        Ok(0)
    });

    Ok((ep_thread, wb_thread))
}

/// Builds the subcrates to get a directory that we can serve. Returns an exit code.
pub fn export(dir: PathBuf, _prog_args: &[String]) -> Result<i32> {
    let spinners = MultiProgress::new();

    let (ep_thread, wb_thread) = export_internal(dir.clone(), &spinners, 2)?;
    let ep_res = ep_thread
        .join()
        .map_err(|_| ErrorKind::ThreadWaitFailed)??;
    if ep_res != 0 {
        return Ok(ep_res);
    }
    let wb_res = wb_thread
        .join()
        .map_err(|_| ErrorKind::ThreadWaitFailed)??;
    if wb_res != 0 {
        return Ok(wb_res);
    }

    // And now we can run the finalization stage
    finalize_export(&dir.join(".perseus"))?;

    // We've handled errors in the component threads, so the exit code is now zero
    Ok(0)
}
