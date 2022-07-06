use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::{collections::HashMap, env};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::RwLock;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::Message;
use warp::Filter;

/// A representation of the clients to the server. These are only the clients
/// that will be told about reloads, any user can command a reload over the HTTP
/// endpoint (unauthenticated because this is a development server).
type Clients = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;

/// A simple counter that can be incremented from anywhere. This will be used as
/// the source of the next user ID. This is an atomic `usize` for maximum
/// platofrm portability (see the Rust docs on atomic primtives).
static NEXT_UID: AtomicUsize = AtomicUsize::new(0);

/// Runs the reload server, which is used to instruct the browser on when to
/// reload for updates.
pub async fn run_reload_server() {
    let (host, port) = get_reload_server_host_and_port();

    // Parse `localhost` into `127.0.0.1` (picky Rust `std`)
    let host = if host == "localhost" {
        "127.0.0.1".to_string()
    } else {
        host
    };
    // Parse the host and port into an address
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    let clients = Clients::default();
    let clients = warp::any().map(move || clients.clone());

    // This will be used by the CLI to order reloads
    let command = warp::path("send")
        .and(clients.clone())
        .then(|clients: Clients| async move {
            // Iterate through all the clients and tell them all to reload
            for (_id, tx) in clients.read().await.iter() {
                // We don't care if this fails, that means the client has disconnected and the
                // disconnection code will be running
                let _ = tx.send(Message::text("reload"));
            }

            "sent".to_string()
        });
    // This will be used by the browser to listen for reload orders
    let receive = warp::path("receive").and(warp::ws()).and(clients).map(
        |ws: warp::ws::Ws, clients: Clients| {
            // This code will run once the WS handshake completes
            ws.on_upgrade(|ws| async move {
                // Assign a new ID to this user
                // This nifty operation just gets the current value and then increments
                let id = NEXT_UID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                // Split out their sender/receiver
                let (mut ws_tx, mut ws_rx) = ws.split();
                // Use an unbounded channel as an intermediary to the WebSocket
                let (tx, rx) = unbounded_channel();
                let mut rx = UnboundedReceiverStream::new(rx);
                tokio::task::spawn(async move {
                    // Whenever a message come sin on that intermediary channel, we'll just relay it
                    // to the client
                    while let Some(message) = rx.next().await {
                        let _ = ws_tx.send(message).await;
                    }
                });

                // Save the sender and their intermediary channel
                clients.write().await.insert(id, tx);

                // Because we don't accept messages from listening clients, we'll just hold a
                // loop until the client disconnects Then, this will become
                // `None` and we'll move on
                while ws_rx.next().await.is_some() {
                    continue;
                }

                // Once we're here, the client has disconnected
                clients.write().await.remove(&id);
            })
        },
    );

    let routes = command.or(receive);
    warp::serve(routes).run(addr).await
}

/// Orders all connected browsers to reload themselves. This spawns a blocking
/// task through Tokio under the hood. Note that this will only do anything if
/// `PERSEUS_USE_RELOAD_SERVER` is set to `true`.
pub fn order_reload() {
    if env::var("PERSEUS_USE_RELOAD_SERVER").is_ok() {
        let (host, port) = get_reload_server_host_and_port();

        tokio::task::spawn_blocking(move || {
            // We don't care if this fails because we have no guarnatees that the server is
            // actually up
            let _ = ureq::get(&format!("http://{}:{}/send", host, port)).call();
        });
    }
}

/// Gets the host and port to run the reload server on.
fn get_reload_server_host_and_port() -> (String, u16) {
    let host = env::var("PERSEUS_RELOAD_SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PERSEUS_RELOAD_SERVER_PORT").unwrap_or_else(|_| "3100".to_string());
    let port = port
        .parse::<u16>()
        .expect("reload server port must be a number");

    (host, port)
}
