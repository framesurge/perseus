use crate::errors::*;
use crate::export;
use crate::parse::{DeployOpts, ExportOpts, ServeOpts};
use crate::serve;
use fs_extra::copy_items;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::fs;
use std::path::PathBuf;

/// Deploys the user's app to the `pkg/` directory (can be changed with
/// `-o/--output`). This will build everything for release and then put it all
/// together in one folder that can be conveniently uploaded to a server, file
/// host, etc. This can return any kind of error because deploying involves
/// working with other subcommands.
pub fn deploy(dir: PathBuf, opts: DeployOpts) -> Result<i32, Error> {
    // Fork at whether we're using static exporting or not
    let exit_code = if opts.export_static {
        deploy_export(dir, opts.output)?
    } else {
        deploy_full(dir, opts.output)?
    };

    Ok(exit_code)
}

/// Deploys the user's app in its entirety, with a bundled server. This can
/// return any kind of error because deploying involves working with other
/// subcommands.
fn deploy_full(dir: PathBuf, output: String) -> Result<i32, Error> {
    // Build everything for production, not running the server
    let (serve_exit_code, server_path) = serve(
        dir.clone(),
        ServeOpts {
            no_run: true,
            no_build: false,
            release: true,
            standalone: true,
            watch: false,
            // These have no impact if `no_run` is `true` (which it is), so we can use the defaults
            // here
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
    )?;
    if serve_exit_code != 0 {
        return Ok(serve_exit_code);
    }
    if let Some(server_path) = server_path {
        // Delete the output directory if it exists and recreate it
        let output_path = PathBuf::from(&output);
        if output_path.exists() {
            if let Err(err) = fs::remove_dir_all(&output_path) {
                return Err(DeployError::ReplaceOutputDirFailed {
                    path: output,
                    source: err,
                }
                .into());
            }
        }
        if let Err(err) = fs::create_dir(&output_path) {
            return Err(DeployError::ReplaceOutputDirFailed {
                path: output,
                source: err,
            }
            .into());
        }
        // Copy in the server executable
        #[cfg(target_os = "windows")]
        let to = output_path.join("server.exe");
        #[cfg(not(target_os = "windows"))]
        let to = output_path.join("server");
        
        if let Err(err) = fs::copy(&server_path, &to) {
            return Err(DeployError::MoveAssetFailed {
                to: to.to_str().map(|s| s.to_string()).unwrap(),
                from: server_path,
                source: err,
            }
            .into());
        }
        // Copy in the `static/` directory if it exists
        let from = dir.join("static");
        if from.exists() {
            if let Err(err) = copy_dir(&from, &output, &CopyOptions::new()) {
                return Err(DeployError::MoveDirFailed {
                    to: output,
                    from: from.to_str().map(|s| s.to_string()).unwrap(),
                    source: err,
                }
                .into());
            }
        }
        // Copy in the `translations` directory if it exists
        let from = dir.join("translations");
        if from.exists() {
            if let Err(err) = copy_dir(&from, &output, &CopyOptions::new()) {
                return Err(DeployError::MoveDirFailed {
                    to: output,
                    from: from.to_str().map(|s| s.to_string()).unwrap(),
                    source: err,
                }
                .into());
            }
        }
        // Copy in the entire `dist` directory (it must exist)
        let from = dir.join("dist");
        if let Err(err) = copy_dir(&from, &output, &CopyOptions::new()) {
            return Err(DeployError::MoveDirFailed {
                to: output,
                from: from.to_str().map(|s| s.to_string()).unwrap(),
                source: err,
            }
            .into());
        }

        println!();
        println!("Deployment complete 🚀! Your app is now available for serving in the standalone folder '{}'! You can run it by executing the `server` binary in that folder.", &output_path.to_str().map(|s| s.to_string()).unwrap());

        Ok(0)
    } else {
        // If we don't have the executable, throw an error
        Err(DeployError::GetServerExecutableFailed.into())
    }
}

/// Uses static exporting to deploy the user's app. This can return any kind of
/// error because deploying involves working with other subcommands.
fn deploy_export(dir: PathBuf, output: String) -> Result<i32, Error> {
    // Export the app to `.perseus/exported`, using release mode
    let export_exit_code = export(
        dir.clone(),
        ExportOpts {
            release: true,
            serve: false,
            host: String::new(),
            port: 0,
            watch: false,
        },
    )?;
    if export_exit_code != 0 {
        return Ok(export_exit_code);
    }
    // That subcommand produces a self-contained static site at `dist/exported/`
    // Just copy that out to the output directory
    let from = dir.join("dist/exported");
    let output_path = PathBuf::from(&output);
    // Delete the output directory if it exists and recreate it
    if output_path.exists() {
        if let Err(err) = fs::remove_dir_all(&output_path) {
            return Err(DeployError::ReplaceOutputDirFailed {
                path: output,
                source: err,
            }
            .into());
        }
    }
    if let Err(err) = fs::create_dir(&output_path) {
        return Err(DeployError::ReplaceOutputDirFailed {
            path: output,
            source: err,
        }
        .into());
    }
    // Now read the contents of the export directory so that we can copy each asset
    // in individually That avoids a `pkg/exported/` situation
    let items = fs::read_dir(&from);
    let items: Vec<PathBuf> = match items {
        Ok(items) => {
            let mut ok_items = Vec::new();
            for item in items {
                match item {
                    Ok(item) => ok_items.push(item.path()),
                    Err(err) => {
                        return Err(DeployError::ReadExportDirFailed {
                            path: from.to_str().map(|s| s.to_string()).unwrap(),
                            source: err,
                        }
                        .into())
                    }
                }
            }

            ok_items
        }
        Err(err) => {
            return Err(DeployError::ReadExportDirFailed {
                path: from.to_str().map(|s| s.to_string()).unwrap(),
                source: err,
            }
            .into())
        }
    };
    // Now run the copy of each item
    if let Err(err) = copy_items(&items, &output, &CopyOptions::new()) {
        return Err(DeployError::MoveExportDirFailed {
            to: output,
            from: from.to_str().map(|s| s.to_string()).unwrap(),
            source: err,
        }
        .into());
    }

    println!();
    println!("Deployment complete 🚀! Your app is now available for serving in the standalone folder '{}'! You can run it by serving the contents of that folder however you'd like.", &output_path.to_str().map(|s| s.to_string()).unwrap());

    Ok(0)
}
