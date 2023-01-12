use crate::reactor::WindowVariable;

/// Marks a checkpoint in the code and alerts any tests that it's been reached
/// by creating an element that represents it. The preferred solution would be
/// emitting a DOM event, but the WebDriver specification currently doesn't
/// support waiting on those (go figure). This will only create a custom element
/// if the `__PERSEUS_TESTING` JS global variable is set to `true`.
///
/// This adds a `<div id="__perseus_checkpoint-<event-name>" />` to the `<div
/// id="__perseus_checkpoints"></div>` element, creating the latter if it
/// doesn't exist. Each checkpoint must have a unique name, and if the same
/// checkpoint is executed twice, it'll be added with a `-<number>` after it,
/// starting from `0`. In this way, we have a functional checkpoints queue for
/// signalling to test code! Note that the checkpoint queue is NOT cleared on
/// subsequent loads.
///
/// Note: this is not just for internal usage, it's highly recommended that you
/// use this for your own checkpoints as well! Just make sure your tests don't
/// conflict with any internal Perseus checkpoint names (preferably prefix yours
/// with `custom_` or the like, as Perseus' checkpoints may change at any time,
/// but won't ever use that namespace).
///
/// **Warning:** your checkpoint names must not include hyphens! This will
/// result in a `panic!`.
pub fn checkpoint(name: &str) {
    if name.contains('-') {
        panic!("checkpoint must not contain hyphens, use underscores instead (hyphens are used as an internal delimiter)");
    }

    let is_testing = WindowVariable::new_bool("__PERSEUS_TESTING");
    match is_testing {
        WindowVariable::Some(val) if val => (),
        // If the boolean was some other type in JS, just abort (this would be a *very* weird
        // environment that implies user tampering)
        _ => return,
    };

    // If we're here, we're testing
    // We dispatch a console warning to reduce the likelihood of literal 'testing in
    // prod'
    crate::web_log!("Perseus is in testing mode. If you're an end-user and seeing this message, please report this as a bug to the website owners!");
    // Create a custom element that can be waited for by the WebDriver
    // This will be removed by the next checkpoint
    let document = web_sys::window().unwrap().document().unwrap();
    let container_opt = document.query_selector("#__perseus_checkpoints").unwrap();
    let container = match container_opt {
        Some(container_i) => container_i,
        None => {
            // If the container doesn't exist yet, create it
            let container = document.create_element("div").unwrap();
            container.set_id("__perseus_checkpoints");
            document
                .query_selector("body")
                .unwrap()
                .unwrap()
                .append_with_node_1(&container)
                .unwrap();
            container
        }
    };

    // Get the number of checkpoints that already exist with the same ID
    // We prevent having to worry about checkpoints whose names are subsets of
    // others by using the hyphen as a delimiter
    let num_checkpoints = document
        .query_selector_all(&format!("[id^=__perseus_checkpoint-{}-]", name))
        .unwrap()
        .length();
    // Append the new checkpoint
    let checkpoint = document.create_element("div").unwrap();
    checkpoint.set_id(&format!(
        "__perseus_checkpoint-{}-{}",
        name, num_checkpoints
    ));
    container.append_with_node_1(&checkpoint).unwrap();
}
