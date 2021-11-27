use crate::errors::*;
use crate::html_shell::{
    interpolate_locale_redirection_fallback, interpolate_page_data, prep_html_shell,
};
use crate::locales::Locales;
use crate::serve::get_render_cfg;
use crate::serve::PageData;
use crate::stores::ImmutableStore;
use crate::template::TemplateMap;
use crate::translations_manager::TranslationsManager;
use crate::SsrNode;
use std::fs;

/// Gets the static page data.
async fn get_static_page_data(
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
pub async fn export_app(
    templates: &TemplateMap<SsrNode>,
    html_shell_path: &str,
    locales: &Locales,
    root_id: &str,
    immutable_store: &ImmutableStore,
    translations_manager: &impl TranslationsManager,
    path_prefix: String,
) -> Result<(), ServerError> {
    // The render configuration acts as a guide here, it tells us exactly what we need to iterate over (no request-side pages!)
    let render_cfg = get_render_cfg(immutable_store).await?;
    // Get the HTML shell and prepare it by interpolating necessary values
    let raw_html_shell =
        fs::read_to_string(html_shell_path).map_err(|err| BuildError::HtmlShellNotFound {
            path: html_shell_path.to_string(),
            source: err,
        })?;
    let html_shell = prep_html_shell(raw_html_shell, &render_cfg, &path_prefix);

    // Loop over every partial
    for (path, template_path) in render_cfg {
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
                    path: template_path,
                }
                .into())
            }
        };
        // Create a locale detection file for it if we're using i18n
        // These just send the app shell, which will perform a redirect as necessary
        // Notably, these also include fallback redirectors if either Wasm or JS is disabled (or both)
        // TODO put everything inside its own folder for initial loads?
        if locales.using_i18n {
            immutable_store
                .write(
                    &format!("exported/{}.html", &initial_load_path),
                    &interpolate_locale_redirection_fallback(
                        &html_shell,
                        // If we don't include  the path prefix, fallback redirection will fail for relative paths
                        &format!("{}/{}/{}", path_prefix, locales.default, &path),
                    ),
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
                let full_html = interpolate_page_data(&html_shell, &page_data, root_id);
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
            let full_html = interpolate_page_data(&html_shell, &page_data, root_id);
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
    }
    // If we're using i18n, loop through the locales to create translations files
    if locales.using_i18n {
        for locale in locales.get_all() {
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
        }
    }
    // Copying in bundles from the filesystem is left to the CLI command for exporting, so we're done!

    Ok(())
}
