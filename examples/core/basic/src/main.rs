#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use perseus::builder::{get_op, run_dflt_engine};

    let op = get_op().unwrap();
    let exit_code = run_dflt_engine(op, lib::get_app(), perseus_warp::dflt_server).await;
    std::process::exit(exit_code);
}

#[cfg(target_arch = "wasm32")]
fn main() {}
