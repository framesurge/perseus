use std::env;

/// Determines the engine operation to be performed by examining environment
/// variables (set automatically by the CLI as appropriate).
///
/// *Note:* in production, it would be inconvenient to provide an environment
/// variable just to get the server to work, so release builds run the server
/// by default if the relevant environment variable is unset. If the same
/// situation occurs in development however, `None` will be returned.
pub fn get_op() -> Option<EngineOperation> {
    let var = match env::var("PERSEUS_ENGINE_OPERATION").ok() {
        Some(var) => var,
        None => {
            return {
                // The only typical use of a release-built binary is as a server, in which case
                // we shouldn't need to specify this environment variable So, in
                // production, we take the server as the default If a user wants
                // a builder though, they can just set the environment variable
                if cfg!(debug_assertions) {
                    None
                } else {
                    Some(EngineOperation::Serve)
                }
            };
        }
    };

    match var.as_str() {
        "serve" => Some(EngineOperation::Serve),
        "build" => Some(EngineOperation::Build),
        "export" => Some(EngineOperation::Export),
        "export_error_page" => Some(EngineOperation::ExportErrorPage),
        "tinker" => Some(EngineOperation::Tinker),
        _ => {
            if cfg!(debug_assertions) {
                None
            } else {
                Some(EngineOperation::Serve)
            }
        }
    }
}

/// A representation of the server-side engine operations that can be performed.
#[derive(Debug, Clone, Copy)]
pub enum EngineOperation {
    /// Run the server for the app. This assumes the app has already been built.
    Serve,
    /// Build the app. This process involves statically generating HTML and the
    /// like to be sent to the client.
    Build,
    /// Export the app by building it and also creating a file layout suitable
    /// for static file serving.
    Export,
    /// Export a single error page to a single file.
    ExportErrorPage,
    /// Run the tinker plugin actions.
    Tinker,
}
