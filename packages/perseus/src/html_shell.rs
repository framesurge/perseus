use crate::page_data::PageData;
use std::borrow::Cow;
use std::collections::HashMap;
use std::{env, fmt};

/// Initializes the HTML shell by interpolating necessary scripts into it, as well as by adding the render configuration.
#[derive(Clone)]
pub struct HtmlShell<'a> {
    shell: String,
    prepend_elements: Vec<Cow<'a, str>>,
    scripts: Vec<Cow<'a, str>>,
    interpolated_content: Vec<Cow<'a, str>>,
    interpolated_scripts: Vec<Cow<'a, str>>,
}

/// Interpolates content, metadata, and state into the HTML shell, ready to be sent to the user for initial loads. This should be passed
/// an HTMl shell prepared with `prep_html_shell`. This also takes the HTML `id` of the element in the shell to interpolate content
/// into.
pub struct HtmlShellWithPageData<'a> {
    shell: HtmlShell<'a>,
    content: &'a str,
    root_id: &'a str,
}

/// Intepolates a fallback for locale redirection pages such that, even if JavaScript is disabled, the user will still be redirected to the default locale.
/// From there, Perseus' inbuilt progressive enhancement can occur, but without this a user directed to an unlocalized page with JS disabled would see a
/// blank screen, which is terrible UX. Note that this also includes a fallback for if JS is enabled but Wasm is disabled. Note that the redirect URL
/// is expected to be generated with a path prefix inbuilt.
///
/// This also adds a `__perseus_initial_state` `<div>` in case it's needed (for Wasm redirections).
pub struct HtmlShellWithRedirect<'a> {
    shell: HtmlShell<'a>,
    root_id: &'a str,
}

impl<'a> HtmlShell<'a> {
    /// Initializes the HTML shell by interpolating necessary scripts into it, as well as by adding the render configuration.
    pub fn new(shell: String, render_cfg: &HashMap<String, String>, path_prefix: &str) -> Self {
        let mut prepend_elements = Vec::new();
        let mut scripts = Vec::new();

        // Define the render config as a global variable
        let render_cfg = format!(
            "window.__PERSEUS_RENDER_CFG = '{render_cfg}';",
            // It's safe to assume that something we just deserialized will serialize again in this case
            render_cfg = serde_json::to_string(render_cfg).unwrap()
        );
        scripts.push(render_cfg.into());

        // Inject a global variable to identify whether we are testing (picked up by app shell to trigger helper DOM events)
        if env::var("PERSEUS_TESTING").is_ok() {
            scripts.push("window.__PERSEUS_TESTING = true;".into());
        }

        // Define the script that will load the Wasm bundle (inlined to avoid unnecessary extra requests)
        let load_wasm_bundle = format!(
            r#"
        import init, {{ run }} from "{path_prefix}/.perseus/bundle.js";
        async function main() {{
            await init("{path_prefix}/.perseus/bundle.wasm");
            run();
        }}
        main();
        "#,
            path_prefix = path_prefix
        );
        scripts.push(load_wasm_bundle.into());

        // Add in the `<base>` element at the very top so that it applies to everything in the HTML shell
        // Otherwise any stylesheets loaded before it won't work properly
        //
        // We add a trailing `/` to the base URL (https://stackoverflow.com/a/26043021)
        // Note that it's already had any pre-existing ones stripped away
        let base = format!(r#"<base href="{}/" />"#, path_prefix);
        prepend_elements.push(base.into());

        Self {
            shell,
            prepend_elements,
            scripts,
            interpolated_content: Vec::new(),
            interpolated_scripts: Vec::new(),
        }
    }

    /// Interpolates page data into the shell.
    pub fn page_data(self, page_data: &'a PageData, root_id: &'a str) -> HtmlShellWithPageData<'a> {
        HtmlShellWithPageData::new(self, page_data, root_id)
    }

    /// Interpolates redirection fallbacks into the shell.
    pub fn locale_redirection_fallback(
        self,
        redirect_url: &'a str,
        root_id: &'a str,
    ) -> HtmlShellWithRedirect<'a> {
        HtmlShellWithRedirect::new(self, redirect_url, root_id)
    }
}

trait ShellWithContent {
    fn shell(&self) -> &HtmlShell;
    fn root_id(&self) -> &str;
    fn content(&self) -> Option<&str>;
}

impl fmt::Display for HtmlShell<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We also inject a delimiter comment that will be used to wall off the constant document head from the interpolated document head

        let head_start = self.prepend_elements.join("\n");
        let head_end = format!(
            r#"
            <script type="module">{scripts}</script>
            <!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->
            {interpolated_content}
            <script>{interpolated_scripts}</script>
            "#,
            scripts = self.scripts.join("\n"),
            interpolated_content = self.interpolated_content.join("\n"),
            interpolated_scripts = self.interpolated_scripts.join("\n"),
        );

        let new_shell = self
            .shell
            .replace("<head>", &format!("<head>{}", head_start))
            .replace("</head>", &format!("{}</head>", head_end));

        f.write_str(&new_shell)
    }
}

impl<'a> HtmlShellWithPageData<'a> {
    fn new(mut shell: HtmlShell<'a>, page_data: &'a PageData, root_id: &'a str) -> Self {
        // Interpolate a global variable of the state so the app shell doesn't have to make any more trips
        // The app shell will unset this after usage so it doesn't contaminate later non-initial loads
        // Error pages (above) will set this to `error`
        let initial_state = if let Some(state) = &page_data.state {
            state
                // We escape any backslashes to prevent their interfering with JSON delimiters
                .replace(r#"\"#, r#"\\"#)
                // We escape any backticks, which would interfere with JS's raw strings system
                .replace(r#"`"#, r#"\`"#)
                // We escape any interpolations into JS's raw string system
                .replace(r#"${"#, r#"\${"#)
        } else {
            "None".to_string()
        };

        // We put this at the very end of the head (after the delimiter comment) because it doesn't matter if it's expunged on subsequent loads
        let initial_state = format!("window.__PERSEUS_INITIAL_STATE = `{}`", initial_state);
        shell.interpolated_scripts.push(initial_state.into());

        // Interpolate the document `<head>`
        shell.interpolated_content.push((&page_data.head).into());

        Self {
            shell,
            content: &page_data.content,
            root_id,
        }
    }
}

impl ShellWithContent for HtmlShellWithPageData<'_> {
    fn shell(&self) -> &HtmlShell {
        &self.shell
    }

    fn root_id(&self) -> &str {
        self.root_id
    }

    fn content(&self) -> Option<&str> {
        Some(self.content)
    }
}

impl<'a> HtmlShellWithRedirect<'a> {
    fn new(mut shell: HtmlShell<'a>, redirect_url: &'a str, root_id: &'a str) -> Self {
        // This will be used if JavaScript is completely disabled (it's then the site's responsibility to show a further message)
        let dumb_redirect = format!(
            r#"<noscript>
        <meta http-equiv="refresh" content="0; url={}" />
    </noscript>"#,
            redirect_url
        );

        // This will be used if JS is enabled, but Wasm is disabled or not supported (it's then the site's responsibility to show a further message)
        // Wasm support detector courtesy https://stackoverflow.com/a/47880734
        let js_redirect = format!(
            r#"
        function wasmSupported() {{
            try {{
                if (typeof WebAssembly === "object"
                    && typeof WebAssembly.instantiate === "function") {{
                    const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
                    if (module instanceof WebAssembly.Module) {{
                        return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
                    }}
                }}
            }} catch (e) {{}}
            return false;
        }}
    
        if (!wasmSupported()) {{
            window.location.replace("{}");
        }}
            "#,
            redirect_url
        );

        shell.interpolated_content.push(dumb_redirect.into());
        shell.interpolated_scripts.push(js_redirect.into());

        Self { shell, root_id }
    }
}

impl ShellWithContent for HtmlShellWithRedirect<'_> {
    fn shell(&self) -> &HtmlShell {
        &self.shell
    }

    fn root_id(&self) -> &str {
        self.root_id
    }

    fn content(&self) -> Option<&str> {
        None
    }
}

macro_rules! impl_display_for_shell_with_content {
    ($t:ident) => {
        impl std::fmt::Display for $t<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let shell = ShellWithContent::shell(self).to_string();
                let root_id = ShellWithContent::root_id(self);
                let content = match ShellWithContent::content(self) {
                    Some(content) => content,
                    None => "",
                };

                // The user MUST place have a `<div>` of this exact form (documented explicitly)
                // We permit either double or single quotes
                let html_to_replace_double = format!("<div id=\"{}\">", root_id);
                let html_to_replace_single = format!("<div id='{}'>", root_id);
                let html_replacement = format!(
                    // We give the content a specific ID so that it can be deleted if an error page needs to be rendered on the client-side
                    r#"{}<div id="__perseus_content_initial" class="__perseus_content">{}</div>"#,
                    &html_to_replace_double, content,
                );
                // Now interpolate that HTML into the HTML shell
                let new_shell = shell
                    .replace(&html_to_replace_double, &html_replacement)
                    .replace(&html_to_replace_single, &html_replacement);

                f.write_str(&new_shell)
            }
        }
    };
}

impl_display_for_shell_with_content!(HtmlShellWithPageData);
impl_display_for_shell_with_content!(HtmlShellWithRedirect);

#[cfg(test)]
mod tests {
    use crate::page_data::PageData;
    use std::{collections::HashMap, iter::FromIterator};

    use super::HtmlShell;

    const SHELL: &str = r#"
    <html>
        <head>
            <title>Shell</title>
        </head>
        <body>
            <p>Content</p>
            <div id="root_id"></div>
        </body>
    </html>
    "#;

    fn get_render_config() -> HashMap<String, String> {
        HashMap::from_iter([("key".into(), "value".into())])
    }

    #[test]
    fn basic_shell() {
        let shell = HtmlShell::new(SHELL.into(), &get_render_config(), "prefix");
        println!("{}", shell);
    }

    #[test]
    fn page_data_shell() {
        let page_data = PageData {
            content: "page_data.content".to_string(),
            state: Some("page_data.state".to_string()),
            head: "page_data.head".to_string(),
        };

        let shell = HtmlShell::new(SHELL.into(), &get_render_config(), "prefix")
            .page_data(&page_data, "root_id");

        println!("{}", shell);
    }

    #[test]
    fn redirect_fallback_shell() {
        let shell = HtmlShell::new(SHELL.into(), &get_render_config(), "prefix")
            .locale_redirection_fallback("redirect_url", "root_id");

        println!("{}", shell);
    }
}
