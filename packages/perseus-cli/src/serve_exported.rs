use console::Emoji;
use std::net::SocketAddr;
use std::path::PathBuf;
use warp::Filter;

use crate::{
    errors::ExecutionError,
    export_error_page,
    parse::{ExportErrorPageOpts, Opts},
    Tools,
};

static SERVING: Emoji<'_, '_> = Emoji("🛰️ ", "");

/// Serves an exported app, assuming it's already been exported.
pub async fn serve_exported(
    dir: PathBuf,
    host: String,
    port: u16,
    tools: &Tools,
    global_opts: &Opts,
) -> Result<i32, ExecutionError> {
    // Export the 404 page so we can serve that directly for convenience (we don't
    // need to delete this, since we'll just put it in the `dist/exported`
    // directory)
    let exit_code = export_error_page(
        dir.clone(),
        &ExportErrorPageOpts {
            code: "404".to_string(),
            output: "dist/exported/__export_404.html".to_string(),
        },
        tools,
        global_opts,
        false, // Don't prompt the user
    )?;
    if exit_code != 0 {
        return Ok(exit_code);
    }

    let dir = dir.join("dist/exported");
    // We actually don't have to worry about HTML file extensions at all
    let files = warp::any()
        .and(warp::fs::dir(dir))
        .or(warp::fs::file("dist/exported/__export_404.html"));
    // Parse `localhost` into `127.0.0.1` (picky Rust `std`)
    let host = if host == "localhost" {
        "127.0.0.1".to_string()
    } else {
        host
    };
    // Parse the host and port into an address
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    // Notify the user that we're serving their files
    println!(
        "  [3/3] {} Your exported app is now live at <http://{host}:{port}>!",
        SERVING,
        host = host,
        port = port
    );

    let _ = warp::serve(files).run(addr).await;
    // We will never get here (the above runs forever)
    Ok(0)
}
