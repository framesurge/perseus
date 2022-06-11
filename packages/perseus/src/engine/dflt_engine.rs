// This file contains functions exclusive to the default engine systems

use super::EngineOperation;
use crate::{i18n::TranslationsManager, stores::MutableStore, PerseusAppBase, SsrNode};
use fmterr::fmt_err;
use futures::Future;
use std::env;

/// A convenience function that automatically runs the necessary engine operation based on the given directive. This provides almost no options for customization, and is
/// usually elided by a macro. More advanced use-cases should bypass this and call the functions this calls manually, with their own configurations.
///
/// The third argument to this is a function to produce a server. In simple cases, this will be the `dflt_server` export from your server integration of choice (which is
/// assumed to use a Tokio 1.x runtime). This function must be infallible (any errors should be panics, as they *will* be treated as unrecoverable).
///
/// If the action is to export a single error page, the HTTP status code of the error page to export and the output will be read as the first and second arguments
/// to the binary invocation. If this is not the desired behavior, you should handle the `EngineOperation::ExportErrorPage` case manually.
///
/// This returns an exit code, which should be returned from the process. Any hanlded errors will be printed to the console.
pub async fn run_dflt_engine<M: MutableStore, T: TranslationsManager, F: Future<Output = ()>>(
    op: EngineOperation,
    app: PerseusAppBase<SsrNode, M, T>,
    serve_fn: impl Fn(PerseusAppBase<SsrNode, M, T>) -> F,
) -> i32 {
    match op {
        EngineOperation::Build => match super::engine_build(app).await {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("{}", fmt_err(&*err));
                1
            }
        },
        EngineOperation::Export => match super::engine_export(app).await {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("{}", fmt_err(&*err));
                1
            }
        },
        EngineOperation::ExportErrorPage => {
            // Get the HTTP status code to build from the arguments to this executable
            // We print errors directly here because we can, and because this behavior is unique to the default engine
            let args = env::args().collect::<Vec<String>>();
            let code = match args.get(1) {
                Some(arg) => {
                    match arg.parse::<u16>() {
                        Ok(err_code) => err_code,
                        Err(_) => {
                            eprintln!("HTTP status code for error page exporting must be a valid integer.");
                            return 1;
                        }
                    }
                }
                None => {
                    eprintln!("Error page exporting requires an HTTP status code for which to export the error page.");
                    return 1;
                }
            };
            // Get the output to write to from the second argument
            let output = match args.get(2) {
                Some(output) => output,
                None => {
                    eprintln!("Error page exporting requires an output location.");
                    return 1;
                }
            };
            match super::engine_export_error_page(app, code, output).await {
                Ok(_) => 0,
                Err(err) => {
                    eprintln!("{}", fmt_err(&*err));
                    1
                }
            }
        }
        EngineOperation::Serve => {
            serve_fn(app).await;
            0
        }
        EngineOperation::Tinker => {
            // This is infallible (though plugins could panic)
            super::engine_tinker(app);
            0
        }
    }
}
