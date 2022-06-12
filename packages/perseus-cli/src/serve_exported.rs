use console::Emoji;
use std::net::SocketAddr;
use std::path::PathBuf;
use warp::Filter;

static SERVING: Emoji<'_, '_> = Emoji("🛰️ ", "");

/// Serves an exported app, assuming it's already been exported.
pub async fn serve_exported(dir: PathBuf, host: String, port: u16) {
    let dir = dir.join("dist/exported");
    // We actually don't have to worry about HTML file extensions at all
    let files = warp::any().and(warp::fs::dir(dir));
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

    warp::serve(files).run(addr).await
}
