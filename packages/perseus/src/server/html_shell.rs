use crate::error_views::ServerErrorData;
use crate::page_data::PageData;
use crate::state::TemplateState;
use crate::utils::minify;
use std::{env, fmt};

/// Escapes special characters in page data that might interfere with JavaScript
/// processing.
fn escape_page_data(data: &str) -> String {
    data.to_string()
        // We escape any backslashes to prevent their interfering with JSON delimiters
        .replace('\\', r#"\\"#)
        // We escape any backticks, which would interfere with JS's raw strings system
        .replace('`', r#"\`"#)
        // We escape any interpolations into JS's raw string system
        .replace(r#"${"#, r#"\${"#)
}

/// The shell used to interpolate the Perseus app into, including associated
/// scripts and content defined by the user, components of the Perseus core, and
/// plugins.
#[derive(Clone, Debug)]
pub(crate) struct HtmlShell {
    /// The actual shell content, on which interpolations will be performed.
    pub shell: String,
    /// Additional contents of the head before the interpolation boundary.
    pub head_before_boundary: Vec<String>,
    /// Scripts to be inserted before the interpolation boundary.
    pub scripts_before_boundary: Vec<String>,
    /// Additional contents of the head after the interpolation boundary. These
    /// will be wiped out after a page transition.
    pub head_after_boundary: Vec<String>,
    /// Scripts to be interpolated after the interpolation boundary. These will
    /// be wiped out after a page transition.
    pub scripts_after_boundary: Vec<String>,
    /// Content to be interpolated into the body of the shell.
    pub content: String,
    /// Code to be inserted into the shell before the Perseus contents of the
    /// page. This is designed to be modified by plugins.
    pub before_content: Vec<String>,
    /// Code to be inserted into the shell after the Perseus contents of the
    /// page. This is designed to be modified by plugins.
    pub after_content: Vec<String>,
    /// The locale to be used for this page, which will be set in the `lang`
    /// attribute on `<html>`.
    ///
    /// **IMPORTANT:** This *does* use `xx-XX` for denoting the locale of an
    /// app not using i18n, but, if this value is set, no interpolation will
    /// be performed. For apps that do use i18n, manually setting the `lang`
    /// attribute in teh index view may lead to unpredictable errors (as
    /// interpolation will nonetheless be performed, leading to conflicts).
    ///
    /// Note that `xx-XX` will be automatically set for locale redirection
    /// pages, preventing interpolation.
    pub locale: String,
    /// The ID of the element into which we'll interpolate content.
    root_id: String,
    /// The path prefix to use.
    #[cfg_attr(not(feature = "preload-wasm-on-redirect"), allow(dead_code))]
    path_prefix: String,
}
impl HtmlShell {
    /// Initializes the HTML shell by interpolating necessary scripts into it
    /// and adding the render configuration.
    pub(crate) fn new(shell: String, root_id: &str, path_prefix: &str) -> Self {
        let mut head_before_boundary = Vec::new();
        let mut scripts_before_boundary = Vec::new();

        // Inject a global variable to identify whether we are testing (picked up by app
        // shell to trigger helper DOM events)
        if env::var("PERSEUS_TESTING").is_ok() {
            scripts_before_boundary.push("window.__PERSEUS_TESTING = true;".into());
        }

        // This script stores all user-provided clicks in a JS array, blocking them
        // until the app has loaded, at which time they'll all be re-emitted.
        // This *can* improve user experience in some cases, and it's available
        // as a feature flag. Clicks on links are deliberately exempted from
        // this, because they can be resolved by the browser without needing
        // interactivity (it'll be slower, but instant navigation is generally a
        // better experience).
        //
        // This feature is heavily opinionated, and should not be enabled by default.
        //
        // This is injected directly, rather than through a separate JS file, in
        // order to ensure it captures *all* user interactions from the moment the
        // user can see the page. In this case, we actually want to block.
        #[cfg(feature = "suspended-interaction")]
        let suspend_script = r#"
        window.__PERSEUS_SUSPENDED = true;
        window.__PERSEUS_SUSPENDED_EVENTS = [];

        const clickHandler = (ev) => {{
            if (window.__PERSEUS_SUSPENDED && ev.target.tagName !== "A") {{
                ev.preventDefault();
                ev.stopPropagation();
                window.__PERSEUS_SUSPENDED_EVENTS.push(ev);
            }}
        }};

        document.addEventListener("click", clickHandler, true);
        document.addEventListener("__perseus_loaded", () => {{
            window.__PERSEUS_SUSPENDED = false;
            for (const ev of window.__PERSEUS_SUSPENDED_EVENTS) {{
                ev.target.dispatchEvent(ev);
            }}
            document.removeEventListener("click", clickHandler);
        }});
        "#;
        #[cfg(not(feature = "suspended-interaction"))]
        let suspend_script = "";
        // Define the script that will load the Wasm bundle (inlined to avoid
        // unnecessary extra requests) If we're using the `wasm2js` feature,
        // this will try to load a JS version instead (expected to be at
        // `/.perseus/bundle.wasm.js`)
        //
        // Note: because we're using binary bundles, we don't need to import
        // a `main` function or the like, `init()` just works
        //
        // Using a `Promise` to wait for the render config to resolve means we
        // avoid blocking the main thread, so failures will just leave the page
        // uninteractive, rather than unresponsive.
        #[cfg(not(feature = "wasm2js"))]
        let load_wasm_bundle = format!(
            r#"
        import init from "{path_prefix}/.perseus/bundle.js";
        async function main() {{
            new Promise((resolve) => {{
                const checker = () => {{
                    if (window.__PERSEUS_RENDER_CFG !== undefined) {{
                        resolve();
                    }} else {{
                        setTimeout(checker, 10);
                    }}
                }};
                checker();
            }}).then(async () => {{
                await init("{path_prefix}/.perseus/bundle.wasm");
            }});
        }}
        main();

        {suspend_script}
        "#,
            path_prefix = path_prefix
        );
        #[cfg(feature = "wasm2js")]
        let load_wasm_bundle = format!(
            r#"
        import init from "{path_prefix}/.perseus/bundle.js";
        async function main() {{
            new Promise((resolve) => {{
                const checker = () => {{
                    if (window.__PERSEUS_RENDER_CFG !== undefined) {{
                        resolve();
                    }} else {{
                        setTimeout(checker, 10);
                    }}
                }};
                checker();
            }}).then(async () => {{
                await init("{path_prefix}/.perseus/bundle.wasm");
            }});
        }}
        main();

        {suspend_script}
        "#,
            path_prefix = path_prefix
        );
        scripts_before_boundary.push(load_wasm_bundle);

        // If we're in development, pass through the host/port of the reload server if
        // we're using it We'll depend on the `PERSEUS_USE_RELOAD_SERVER`
        // environment variable here, which is set by the CLI's controller process, not
        // the user That way, we won't do this if the reload server doesn't
        // exist
        #[cfg(debug_assertions)]
        if env::var("PERSEUS_USE_RELOAD_SERVER").is_ok() {
            let host =
                env::var("PERSEUS_RELOAD_SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port =
                env::var("PERSEUS_RELOAD_SERVER_PORT").unwrap_or_else(|_| "3100".to_string());
            scripts_before_boundary
                .push(format!("window.__PERSEUS_RELOAD_SERVER_HOST = '{}';", host));
            scripts_before_boundary
                .push(format!("window.__PERSEUS_RELOAD_SERVER_PORT = '{}';", port));
        }

        // Add in the `<base>` element at the very top so that it applies to everything
        // in the HTML shell Otherwise any stylesheets loaded before it won't
        // work properly We add a trailing `/` to the base URL (https://stackoverflow.com/a/26043021)
        // Note that it's already had any pre-existing ones stripped away
        let base = format!(r#"<base href="{}/" />"#, path_prefix);
        head_before_boundary.push(base);

        Self {
            shell,
            head_before_boundary,
            scripts_before_boundary,
            head_after_boundary: Vec::new(),
            scripts_after_boundary: Vec::new(),
            before_content: Vec::new(),
            after_content: Vec::new(),
            content: "".into(),
            root_id: root_id.into(),
            path_prefix: path_prefix.into(),
            // Assume until we know otherwise
            locale: "xx-XX".to_string(),
        }
    }

    /// Interpolates page data, global state, and translations into the shell.
    ///
    /// The translations provided should be the source string from which a
    /// translator can be derived on the client-side. These are provided in
    /// a window variable to avoid page interactivity requiring a network
    /// request to get them.
    ///
    /// This needs to know what the locale is so it can set the HTML `lang`
    /// attribute for improved SEO.
    pub(crate) fn page_data(
        mut self,
        page_data: &PageData,
        global_state: &TemplateState,
        locale: &str,
    ) -> Self {
        self.locale = locale.to_string();
        // Interpolate a global variable of the state so the app shell doesn't have to
        // make any more trips The app shell will unset this after usage so it
        // doesn't contaminate later non-initial loads Error pages (above) will
        // set this to `error`
        let initial_state = escape_page_data(&page_data.state.to_string());
        // We know the form of this, and it won't fail
        let initial_widget_states =
            escape_page_data(&serde_json::to_string(&page_data.widget_states).unwrap());
        let global_state = escape_page_data(&global_state.state.to_string());

        // We put this at the very end of the head (after the delimiter comment) because
        // it doesn't matter if it's expunged on subsequent loads
        let initial_state = format!("window.__PERSEUS_INITIAL_STATE = `{}`;", initial_state);
        self.scripts_after_boundary.push(initial_state);
        let initial_widget_states = format!(
            "window.__PERSEUS_INITIAL_WIDGET_STATES = `{}`;",
            initial_widget_states
        );
        self.scripts_after_boundary.push(initial_widget_states);
        // But we'll need the global state as a variable until a template accesses it,
        // so we'll keep it around (even though it should actually instantiate validly
        // and not need this after the initial load)
        let global_state = format!("window.__PERSEUS_GLOBAL_STATE = `{}`;", global_state);
        self.scripts_before_boundary.push(global_state);

        // Now import the constants we'll need for the initial load of this locale
        // (i.e. render config and translations)
        //
        // We can put all this after the boundary, because everything will be
        // automatically cached for later use
        let consts_import = format!(
            "<script src=\".perseus/initial_consts/{}.js\"></script>",
            locale
        );
        self.head_after_boundary.push(consts_import);

        // Interpolate the document `<head>` (this should of course be removed between
        // page loads)
        self.head_after_boundary.push((&page_data.head).into());
        // And set the content
        self.content = (&page_data.content).into();

        self
    }

    /// Interpolates a fallback for locale redirection pages such that, even if
    /// JavaScript is disabled, the user will still be redirected to the default
    /// locale. From there, Perseus' inbuilt progressive enhancement can
    /// occur, but without this a user directed to an unlocalized page with JS
    /// disabled would see a blank screen, which is terrible UX. Note that
    /// this also includes a fallback for if JS is enabled but Wasm is disabled.
    /// Note that the redirect URL is expected to be generated with a path
    /// prefix inbuilt.
    ///
    /// This also adds a `__perseus_initial_state` `<div>` in case it's needed
    /// (for Wasm redirections).
    ///
    /// Further, this will preload the Wasm binary, making redirection snappier
    /// (but initial load slower), a tradeoff that generally improves UX.
    pub(crate) fn locale_redirection_fallback(
        mut self,
        redirect_url: &str,
        global_state: &TemplateState,
    ) -> Self {
        self.locale = "xx-XX".to_string();

        // We still have to inject the global state, which is unlocalized (see #267)
        let global_state = escape_page_data(&global_state.state.to_string());
        let global_state = format!("window.__PERSEUS_GLOBAL_STATE = `{}`;", global_state);
        self.scripts_before_boundary.push(global_state);

        // We also still have to inject an unlocalized render config
        self.head_after_boundary
            .push("<script src=\".perseus/initial_consts.js\"></script>".to_string());

        // This will be used if JavaScript is completely disabled (it's then the site's
        // responsibility to show a further message)
        let dumb_redirect = format!(
            r#"<noscript>
        <meta http-equiv="refresh" content="0; url={}" />
    </noscript>"#,
            redirect_url
        );

        // This will be used if JS is enabled, but Wasm is disabled or not supported
        // (it's then the site's responsibility to show a further message)
        //
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

        self.head_after_boundary.push(dumb_redirect);
        self.scripts_after_boundary.push(js_redirect);
        #[cfg(feature = "preload-wasm-on-redirect")]
        {
            // Interpolate a preload of the Wasm bundle
            // This forces the browser to get the bundle before loading the page, which
            // makes the time users spend on a blank screen much shorter We have
            // no leading `/` here because of the `<base>` interpolation
            // Note that this has to come before the code that actually loads the Wasm
            // bundle The aim of this is to make the time loading increase so
            // that the time blanking decreases
            let wasm_preload = format!(
                r#"<link rel="preload" href="{path_prefix}/.perseus/bundle.wasm" as="fetch" />"#,
                path_prefix = self.path_prefix
            );
            self.head_before_boundary.push(wasm_preload.into());
        }

        self
    }

    /// Interpolates page error data into the shell in the event of a failure.
    ///
    /// This takes an optional translations string if it's available, injecting
    /// it if possible. If the reactor finds this variable to be empty on an
    /// error extracted from the initial state variable, it will assume the
    /// error is unlocalized.
    pub(crate) fn error_page(
        mut self,
        error_page_data: &ServerErrorData,
        error_html: &str,
        error_head: &str,
        locale: Option<String>, // For internal convenience
    ) -> Self {
        // If we have a locale, we'll use the localized JS constants
        if let Some(locale) = locale {
            let consts_import = format!(
                "<script src=\".perseus/initial_consts/{}.js\"></script>",
                &locale
            );
            self.head_after_boundary.push(consts_import);
            self.locale = locale;
        } else {
            self.head_after_boundary
                .push("<script src=\".perseus/initial_consts.js\"></script>".to_string());
        }

        let error = serde_json::to_string(error_page_data).unwrap();
        let state_var = format!(
            "window.__PERSEUS_INITIAL_STATE = `error-{}`;",
            escape_page_data(&error),
        );
        self.scripts_after_boundary.push(state_var);

        self.head_after_boundary.push(error_head.to_string());
        self.content = error_html.into();

        self
    }
}
// This code actually interpolates everything in the correct places
// Because of the way these string interpolations work, there MUST NOT be
// hydration IDs on the `<head>` or `<body>` tags, or Perseus will break in very
// unexpected ways
impl fmt::Display for HtmlShell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let head_start = self.head_before_boundary.join("\n");
        // We also inject a delimiter dummy `<meta>` tag that will be used to wall off
        // the constant document head from the interpolated document head
        // There's another one of these for denoting the end so we can semi-reliably
        // extract the head of initially loaded pages for PSS caching
        let head_end = format!(
            r#"
            <script type="module">{scripts_before_boundary}</script>
            <meta itemprop="__perseus_head_boundary" content="">
            {head_after_boundary}
            <script>{scripts_after_boundary}</script>
            <meta itemprop="__perseus_head_end" content="">
            <script>//</script>
            "#,
            scripts_before_boundary = self.scripts_before_boundary.join("\n"),
            head_after_boundary = self.head_after_boundary.join("\n"),
            scripts_after_boundary = self.scripts_after_boundary.join("\n"),
        );

        let shell_with_head = self
            .shell
            .replace("<head>", &format!("<head>{}", head_start))
            .replace("</head>", &format!("{}</head>", head_end));

        let body_start = self.before_content.join("\n");
        let body_end = self.after_content.join("\n");
        // We also insert the popup error handler here
        let shell_with_body = shell_with_head
            .replace("<body>", &format!("<body>{}", body_start))
            .replace(
                "</body>",
                &format!(
                    "{}<div id=\"__perseus_popup_error\"></div></body>",
                    body_end
                ),
            );

        // The user MUST place have a `<div>` of this exact form (documented explicitly)
        // We permit either double or single quotes
        let html_to_replace_double = format!("<div id=\"{}\">", self.root_id);
        let html_to_replace_single = format!("<div id='{}'>", self.root_id);
        let html_replacement = format!(
            // We give the content a specific ID so that it can be deleted if an error page needs
            // to be rendered on the client-side
            "{}{}",
            &html_to_replace_double, self.content,
        );
        // Now interpolate that HTML into the HTML shell
        let new_shell = shell_with_body
            .replace(&html_to_replace_double, &html_replacement)
            .replace(&html_to_replace_single, &html_replacement);

        // Finally, set the `lang` tag if we should
        let final_shell = if self.locale != "xx-XX" {
            new_shell.replace("<html", &format!(r#"<html lang="{}""#, self.locale))
        } else {
            new_shell
        };

        // And minify everything
        // Because this is run on live requests, we have to be fault-tolerant (if we
        // can't minify, we'll fall back to unminified)
        let minified = match minify(&final_shell, true) {
            Ok(minified) => minified,
            Err(_) => final_shell,
        };

        f.write_str(&minified)
    }
}
