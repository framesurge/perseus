use app::{get_config_manager, get_templates_vec};
use futures::executor::block_on;
use perseus::{build_templates, SsrNode};

fn main() {
    let config_manager = get_config_manager();

    let fut = build_templates(get_templates_vec::<SsrNode>(), &config_manager);
    let res = block_on(fut);
    if let Err(err) = res {
        eprintln!("Static generation failed: '{}'", err);
    } else {
        println!("Static generation successfully completed!");
    }
}
