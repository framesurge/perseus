use crate::errors::*;
use crate::i18n::{Locales, TranslationsManager};
use crate::server::{get_render_cfg, HtmlShell, PageData};
use crate::stores::ImmutableStore;
use crate::template::TemplateMap;
use crate::SsrNode;
use futures::future::{try_join, try_join_all};
use std::fs;

/// Gets the static page data.
pub async fn get_static_page_data(
    path: &str,
    has_state: bool,
    immutable_store: &ImmutableStore,
) -> Result<PageData, ServerError> {
    // Get the partial HTML content and a state to go with it (if applicable)
    let content = immutable_store
        .read(&format!("static/{}.html", path))
        .await?;
    let head = immutable_store
        .read(&format!("static/{}.head.html", path))
        .await?;
    let state = match has_state {
        true => Some(
            immutable_store
                .read(&format!("static/{}.json", path))
                .await?,
        ),
        false => None,
    };
    // Create an instance of `PageData`
    Ok(PageData {
        content,
        state,
        head,
    })
}

/// Exports your app to static files, which can be served from anywhere, without needing a server. This assumes that the app has already
/// been built, and that no templates are using non-static features (which can be ensured by passing `true` as the last parameter to
/// `build_app`).
#[allow(clippy::too_many_arguments)]
pub async fn export_app(
    templates: &TemplateMap<SsrNode>,
    html_shell_path: &str,
    locales: &Locales,
    root_id: &str,
    immutable_store: &ImmutableStore,
    translations_manager: &impl TranslationsManager,
    path_prefix: String,
    global_state: &Option<String>,
) -> Result<(), ServerError> {
    // The render configuration acts as a guide here, it tells us exactly what we need to iterate over (no request-side pages!)
    let render_cfg = get_render_cfg(immutable_store).await?;
    // Get the HTML shell and prepare it by interpolating necessary values
    let raw_html_shell =
        fs::read_to_string(html_shell_path).map_err(|err| BuildError::HtmlShellNotFound {
            path: html_shell_path.to_string(),
            source: err,
        })?;
    let html_shell = HtmlShell::new(raw_html_shell, root_id, &render_cfg, &path_prefix);

    // We can do literally everything concurrently here
    let mut export_futs = Vec::new();
    // Loop over every partial
    for (path, template_path) in render_cfg {
        let fut = export_path(
            (path.to_string(), template_path.to_string()),
            templates,
            locales,
            &html_shell,
            immutable_store,
            path_prefix.to_string(),
            global_state,
        );
        export_futs.push(fut);
    }
    // If we're using i18n, loop through the locales to create translations files
    let mut translations_futs = Vec::new();
    if locales.using_i18n {
        for locale in locales.get_all() {
            let fut = create_translation_file(locale, immutable_store, translations_manager);
            translations_futs.push(fut);
        }
    }

    try_join(try_join_all(export_futs), try_join_all(translations_futs)).await?;

    // Copying in bundles from the filesystem is left to the CLI command for exporting, so we're done!

    Ok(())
}

/// Creates a translation file for exporting. This is broken out for concurrency.
pub async fn create_translation_file(
    locale: &str,
    immutable_store: &ImmutableStore,
    translations_manager: &impl TranslationsManager,
) -> Result<(), ServerError> {
    // Get the translations string for that
    let translations_str = translations_manager
        .get_translations_str_for_locale(locale.to_string())
        .await?;
    // Write it to an asset so that it can be served directly
    immutable_store
        .write(
            &format!("exported/.perseus/translations/{}", locale),
            &translations_str,
        )
        .await?;

    Ok(())
}

/// Exports a single path within a template.
pub async fn export_path(
    (path, template_path): (String, String),
    templates: &TemplateMap<SsrNode>,
    locales: &Locales,
    html_shell: &HtmlShell<'_>,
    immutable_store: &ImmutableStore,
    path_prefix: String,
    global_state: &Option<String>,
) -> Result<(), ServerError> {
    // We need the encoded path to reference flattened build artifacts
    // But we don't create a flattened system with exporting, everything is properly created in a directory structure
    let path_encoded = urlencoding::encode(&path).to_string();
    // All initial load pages should be written into their own folders, which prevents a situation of a template root page outside the directory for the rest of that template's pages (see #73)
    // The `.html` file extension is added when this variable is used (for contrast to the `.json`s)
    let initial_load_path = if path.ends_with("index") {
        // However, if it's already an index page, we dont want `index/index.html`
        path.to_string()
    } else {
        format!("{}/index", &path)
    };

    // Get the template itself
    let template = templates.get(&template_path);
    let template = match template {
        Some(template) => template,
        None => {
            return Err(ServeError::PageNotFound {
                path: template_path.to_string(),
            }
            .into())
        }
    };
    // Create a locale detection file for it if we're using i18n
    // These just send the app shell, which will perform a redirect as necessary
    // Notably, these also include fallback redirectors if either Wasm or JS is disabled (or both)
    if locales.using_i18n {
        immutable_store
            .write(
                &format!("exported/{}.html", &initial_load_path),
                &html_shell
                    .clone()
                    .locale_redirection_fallback(&format!(
                        "{}/{}/{}",
                        path_prefix, locales.default, &path
                    ))
                    .to_string(),
            )
            .await?;
    }
    // Check if that template uses build state (in which case it should have a JSON file)
    let has_state = template.uses_build_state();
    if locales.using_i18n {
        // Loop through all the app's locales
        for locale in locales.get_all() {
            let page_data = get_static_page_data(
                &format!("{}-{}", locale, &path_encoded),
                has_state,
                immutable_store,
            )
            .await?;
            // Create a full HTML file from those that can be served for initial loads
            // The build process writes these with a dummy default locale even though we're not using i18n
            let full_html = html_shell
                .clone()
                .page_data(&page_data, global_state)
                .to_string();
            immutable_store
                .write(
                    &format!("exported/{}/{}.html", locale, initial_load_path),
                    &full_html,
                )
                .await?;

            // Serialize the page data to JSON and write it as a partial (fetched by the app shell for subsequent loads)
            let partial = serde_json::to_string(&page_data).unwrap();
            immutable_store
                .write(
                    &format!("exported/.perseus/page/{}/{}.json", locale, &path),
                    &partial,
                )
                .await?;
        }
    } else {
        let page_data = get_static_page_data(
            &format!("{}-{}", locales.default, &path_encoded),
            has_state,
            immutable_store,
        )
        .await?;
        // Create a full HTML file from those that can be served for initial loads
        // The build process writes these with a dummy default locale even though we're not using i18n
        let full_html = html_shell
            .clone()
            .page_data(&page_data, global_state)
            .to_string();
        // We don't add an extension because this will be queried directly by the browser
        immutable_store
            .write(&format!("exported/{}.html", initial_load_path), &full_html)
            .await?;

        // Serialize the page data to JSON and write it as a partial (fetched by the app shell for subsequent loads)
        let partial = serde_json::to_string(&page_data).unwrap();
        immutable_store
            .write(
                &format!("exported/.perseus/page/{}/{}.json", locales.default, &path),
                &partial,
            )
            .await?;
    }

    Ok(())
}
