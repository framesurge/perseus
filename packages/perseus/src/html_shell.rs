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
