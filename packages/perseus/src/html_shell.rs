use crate::serve::PageData;
use std::collections::HashMap;
use std::env;

/// Initializes the HTML shell by interpolating necessary scripts into it, as well as by adding the render configuration.
pub fn prep_html_shell(html_shell: String, render_cfg: &HashMap<String, String>) -> String {
    // Define the script that will load the Wasm bundle (inlined to avoid unnecessary extra requests)
    let load_script = r#"<script type="module">
    import init, { run } from "/.perseus/bundle.js";
    async function main() {
        await init("/.perseus/bundle.wasm");
        run();
    }
    main();
</script>"#;
    // We inject a script that defines the render config as a global variable, which we put just before the close of the head
    // We also inject a delimiter comment that will be used to wall off the constant document head from the interpolated document head
    // We also inject the above script to load the Wasm bundle (avoids extra trips)
    // We also inject a global variable to identify that we're testing if we are (picked up by app shell to trigger helper DOM events)
    let prepared = html_shell.replace(
        "</head>",
        // It's safe to assume that something we just deserialized will serialize again in this case
        &format!(
            "<script>window.__PERSEUS_RENDER_CFG = '{}';{testing_var}</script>\n{}\n<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->\n</head>",
            serde_json::to_string(&render_cfg).unwrap(),
            load_script,
            testing_var=if env::var("PERSEUS_TESTING").is_ok() {
                "window.__PERSEUS_TESTING = true;"
            } else {
                ""
            }
        ),
    );

    prepared
}

/// Interpolates content, metadata, and state into the HTML shell, ready to be sent to the user for initial loads. This should be passed
/// an HTMl shell prepared with `prep_html_shell`. This also takes the HTML `id` of the element in the shell to interpolate content
/// into.
pub fn interpolate_page_data(html_shell: &str, page_data: &PageData, root_id: &str) -> String {
    // Interpolate the document `<head>`
    let html_with_head = html_shell.replace(
        "<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->",
        &format!("<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->{}", &page_data.head),
    );

    // Interpolate a global variable of the state so the app shell doesn't have to make any more trips
    // The app shell will unset this after usage so it doesn't contaminate later non-initial loads
    // Error pages (above) will set this to `error`
    let state_var = format!("<script>window.__PERSEUS_INITIAL_STATE = '{}';</script>", {
        if let Some(state) = &page_data.state {
            state
                // If we don't escape quotes, we get runtime syntax errors
                .replace(r#"'"#, r#"\'"#)
                .replace(r#"""#, r#"\""#)
        } else {
            "None".to_string()
        }
    });
    // We put this at the very end of the head (after the delimiter comment) because it doesn't matter if it's expunged on subsequent loads
    let html_with_state = html_with_head.replace("</head>", &format!("{}\n</head>", state_var));

    // Figure out exactly what we're interpolating in terms of content
    // The user MUST place have a `<div>` of this exact form (documented explicitly)
    // We permit either double or single quotes
    let html_to_replace_double = format!("<div id=\"{}\">", root_id);
    let html_to_replace_single = format!("<div id='{}'>", root_id);
    let html_replacement = format!(
        // We give the content a specific ID so that it can be deleted if an error page needs to be rendered on the client-side
        "{}<div id=\"__perseus_content_initial\" class=\"__perseus_content\">{}</div>",
        &html_to_replace_double,
        &page_data.content
    );
    // Now interpolate that HTML into the HTML shell
    html_with_state
        .replace(&html_to_replace_double, &html_replacement)
        .replace(&html_to_replace_single, &html_replacement)
}
