use std::env;

/// Gets the host and port to serve on based on environment variables, which are
/// universally used for configuration regardless of engine.
pub(crate) fn get_host_and_port() -> (String, u16) {
    // We have to use two sets of environment variables until v0.4.0
    let host = env::var("PERSEUS_HOST");
    let port = env::var("PERSEUS_PORT");

    let host = host.unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = port
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Port must be a number.");

    (host, port)
}
