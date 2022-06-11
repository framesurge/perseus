use std::env;

/// Determines the engine operation to be performed by examining environment variables (set automatically by the CLI as appropriate).
pub fn get_op() -> Option<EngineOperation> {
    let var = env::var("PERSEUS_ENGINE_OPERATION").ok()?;
    match var.as_str() {
        "serve" => Some(EngineOperation::Serve),
        "build" => Some(EngineOperation::Build),
        "export" => Some(EngineOperation::Export),
        "export_error_page" => Some(EngineOperation::ExportErrorPage),
        "tinker" => Some(EngineOperation::Tinker),
        _ => None
    }
}

/// A representation of the server-side engine operations that can be performed.
pub enum EngineOperation {
    /// Run the server for the app. This assumes the app has already been built.
    Serve,
    /// Build the app. This process involves statically generating HTML and the like to be sent to the client.
    Build,
    /// Export the app by building it and also creating a file layout suitable for static file serving.
    Export,
    /// Export a single error page to a single file.
    ExportErrorPage,
    /// Run the tinker plugin actions.
    Tinker,
}
