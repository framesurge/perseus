use crate::cmd::{cfg_spinner, run_stage};
use crate::install::Tools;
use crate::parse::{ExportOpts, Opts};
use crate::thread::{spawn_thread, ThreadHandle};
use crate::{errors::*, get_user_crate_name};
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar};
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
            return ::std::result::Result::Ok(code);
        }
    };
}

/// An internal macro for symlinking/copying files into the export package. The
/// `from` and `to` that this accepts should be extensions of the `target`, and
/// they'll be `.join()`ed on.
///
/// This will attempt to symlink, and, if that fails, it will print a warning
/// and copy normally.
///
/// For symlinking to work on Windows, developer mode must be enabled.
macro_rules! copy_file {
    ($from:expr, $to:expr, $target:expr) => {
        // Try symlinking first
        #[cfg(unix)]
        if std::os::unix::fs::symlink($target.join($from), $target.join($to)).is_err() {
            // That failed, try a usual copy
            if let Err(err) = fs::copy($target.join($from), $target.join($to)) {
                return Err(ExportError::MoveAssetFailed {
                    to: $to.to_string(),
                    from: $from.to_string(),
                    source: err,
                });
            }
        }
        #[cfg(windows)]
        if std::os::windows::fs::symlink_file($target.join($from), $target.join($to)).is_err() {
            // That failed, try a usual copy
            if let Err(err) = fs::copy($target.join($from), $target.join($to)) {
                return Err(ExportError::MoveAssetFailed {
                    to: $to.to_string(),
                    from: $from.to_string(),
                    source: err,
                });
            }
        }
    };
}

/// An internal macro for symlinking/copying directories into the export
/// package. The `from` and `to` that this accepts should be extensions of the
/// `target`, and they'll be `.join()`ed on.
///
/// This will attempt to symlink, and, if that fails, it will print a warning
/// and copy normally.
///
/// For symlinking to work on Windows, developer mode must be enabled.
macro_rules! copy_directory {
    ($from:expr, $to:expr, $to_symlink:expr, $target:expr) => {
        // Try symlinking first
        #[cfg(unix)]
        if std::os::unix::fs::symlink($target.join($from), $target.join($to_symlink)).is_err() {
            // That failed, try a usual copy
            if let Err(err) = fs_extra::dir::copy(
                $target.join($from),
                $target.join($to),
                &fs_extra::dir::CopyOptions::new(),
            ) {
                return Err(ExportError::MoveDirFailed {
                    to: $to.to_string(),
                    from: $from.to_string(),
                    source: err,
                });
            }
        }
        #[cfg(windows)]
        if std::os::unix::fs::symlink_dir($target.join($from), $target.join($to_symlink)).is_err() {
            // That failed, try a usual copy
            if let Err(err) = fs_extra::dir::copy(
                $target.join($from),
                $target.join($to),
                &fs_extra::dir::CopyOptions::new(),
            ) {
                return Err(ExportError::MoveDirFailed {
                    to: $to.to_string(),
                    from: $from.to_string(),
                    source: err,
                });
            }
        }
    };
}

/// Finalizes the export by copying assets. This is very different from the
/// finalization process of normal building.
pub fn finalize_export(target: &Path) -> Result<(), ExportError> {
    // Copy files over (the directory structure should already exist from exporting
    // the pages)
    copy_file!(
        "dist/pkg/perseus_engine.js",
        "dist/exported/.perseus/bundle.js",
        target
    );
    copy_file!(
        "dist/pkg/perseus_engine_bg.wasm",
        "dist/exported/.perseus/bundle.wasm",
        target
    );
    // Copy any JS snippets over (if the directory doesn't exist though, don't do
    // anything)
    if fs::metadata(target.join("dist/pkg/snippets")).is_ok() {
        copy_directory!(
            "dist/pkg/snippets",
            "dist/exported/.perseus",          // For a usual copy
            "dist/exported/.perseus/snippets", // For a symlink
            target
        );
    }

    Ok(())
}

/// Actually exports the user's code, program arguments having been interpreted.
/// This needs to know how many steps there are in total because the serving
/// logic also uses it. This also takes a `MultiProgress` to interact with so it
/// can be used truly atomically. This returns handles for waiting on the
/// component threads so we can use it composably.
#[allow(clippy::type_complexity)]
pub fn export_internal(
    dir: PathBuf,
    spinners: &MultiProgress,
    num_steps: u8,
    is_release: bool,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<
    (
        ThreadHandle<impl FnOnce() -> Result<i32, ExportError>, Result<i32, ExportError>>,
        ThreadHandle<impl FnOnce() -> Result<i32, ExportError>, Result<i32, ExportError>>,
    ),
    ExportError,
> {
    let tools = tools.clone();
    let Opts {
        cargo_browser_args,
        cargo_engine_args,
        wasm_bindgen_args,
        wasm_opt_args,
        wasm_release_rustflags,
        ..
    } = global_opts.clone();
    let crate_name = get_user_crate_name(&dir)?;

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
    // We make sure to add them at the top (the server spinner may have already been
    // instantiated)
    let ep_spinner = spinners.insert(0, ProgressBar::new_spinner());
    let ep_spinner = cfg_spinner(ep_spinner, &ep_msg);
    let ep_target = dir.clone();
    let wb_spinner = spinners.insert(1, ProgressBar::new_spinner());
    let wb_spinner = cfg_spinner(wb_spinner, &wb_msg);
    let wb_target = dir;
    let cargo_engine_exec = tools.cargo_engine.clone();
    let ep_thread = spawn_thread(
        move || {
            handle_exit_code!(run_stage(
                vec![&format!(
                    "{} run {} {}",
                    cargo_engine_exec,
                    if is_release { "--release" } else { "" },
                    cargo_engine_args
                )],
                &ep_target,
                &ep_spinner,
                &ep_msg,
                vec![
                    ("PERSEUS_ENGINE_OPERATION", "export"),
                    ("CARGO_TARGET_DIR", "dist/target_engine")
                ]
            )?);

            Ok(0)
        },
        global_opts.sequential,
    );
    let wb_thread = spawn_thread(
        move || {
            let mut cmds = vec![
            // Build the Wasm artifact first (and we know where it will end up, since we're setting the target directory)
            format!(
                "{} build --target wasm32-unknown-unknown {} {}",
                tools.cargo_browser,
                if is_release { "--release" } else { "" },
                cargo_browser_args
            ),
            // NOTE The `wasm-bindgen` version has to be *identical* to the dependency version
            format!(
                "{cmd} ./dist/target_wasm/wasm32-unknown-unknown/{profile}/{crate_name}.wasm --out-dir dist/pkg --out-name perseus_engine --target web {args}",
                cmd=tools.wasm_bindgen,
                profile={ if is_release { "release" } else { "debug" } },
                args=wasm_bindgen_args,
                crate_name=crate_name
            )
        ];
            // If we're building for release, then we should run `wasm-opt`
            if is_release {
                cmds.push(format!(
                "{cmd} -Oz ./dist/pkg/perseus_engine_bg.wasm -o ./dist/pkg/perseus_engine_bg.wasm {args}",
                cmd=tools.wasm_opt,
                args=wasm_opt_args
            ));
            }
            let cmds = cmds.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
            handle_exit_code!(run_stage(
                cmds,
                &wb_target,
                &wb_spinner,
                &wb_msg,
                if is_release {
                    vec![
                        ("CARGO_TARGET_DIR", "dist/target_wasm"),
                        ("RUSTFLAGS", &wasm_release_rustflags),
                    ]
                } else {
                    vec![("CARGO_TARGET_DIR", "dist/target_wasm")]
                }
            )?);

            Ok(0)
        },
        global_opts.sequential,
    );

    Ok((ep_thread, wb_thread))
}

/// Builds the subcrates to get a directory that we can serve. Returns an exit
/// code.
pub fn export(
    dir: PathBuf,
    opts: &ExportOpts,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExportError> {
    let spinners = MultiProgress::new();
    // We'll add another not-quite-spinner if we're serving
    let num_spinners = if opts.serve { 3 } else { 2 };

    let (ep_thread, wb_thread) = export_internal(
        dir.clone(),
        &spinners,
        num_spinners,
        opts.release,
        tools,
        global_opts,
    )?;
    let ep_res = ep_thread
        .join()
        .map_err(|_| ExecutionError::ThreadWaitFailed)??;
    if ep_res != 0 {
        return Ok(ep_res);
    }
    let wb_res = wb_thread
        .join()
        .map_err(|_| ExecutionError::ThreadWaitFailed)??;
    if wb_res != 0 {
        return Ok(wb_res);
    }

    // And now we can run the finalization stage
    finalize_export(&dir)?;

    // We've handled errors in the component threads, so the exit code is now zero
    Ok(0)
}
