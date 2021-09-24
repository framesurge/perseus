use crate::errors::*;
use crate::ConfigManager;
use crate::TemplateMap;
use crate::SsrNode;
use crate::Locales;
use crate::html_shell::{prep_html_shell, interpolate_page_data};
use crate::serve::PageData;
use crate::get_render_cfg;
use std::fs;

/// Gets the static page data.
async fn get_static_page_data(path: &str, has_state: bool, config_manager: &impl ConfigManager) -> Result<PageData> {
    // Get the partial HTML content and a state to go with it (if applicable)
    let content = config_manager.read(&format!("static/{}.html", path)).await?;
    let head = config_manager.read(&format!("static/{}.head.html", path)).await?;
    let state = match has_state {
        true => Some(config_manager.read(&format!("static/{}.json", path)).await?),
        false => None
    };
    // Create an instance of `PageData`
    Ok(PageData {
        content,
        state,
        head
    })
}

/// Exports your app to static files, which can be served from anywhere, without needing a server. This assumes that the app has already
/// been built, and that no templates are using non-static features (which can be ensured by passing `true` as the last parameter to
/// `build_app`).
pub async fn export_app(
    templates: TemplateMap<SsrNode>,
    html_shell_path: &str,
    locales: &Locales,
    root_id: &str,
    config_manager: &impl ConfigManager
) -> Result<()> {
    // The render configuration acts as a guide here, it tells us exactly what we need to iterate over (no request-side pages!)
    let render_cfg = get_render_cfg(config_manager)
        .await?;
    // Get the HTML shell and prepare it by interpolating necessary values
    let raw_html_shell = fs::read_to_string(html_shell_path).map_err(|err| ErrorKind::HtmlShellNotFound(html_shell_path.to_string(), err.to_string()))?;
    let html_shell = prep_html_shell(raw_html_shell, &render_cfg);

    // Loop over every partial
    for (path, template_path) in render_cfg {
        // We need the encoded path to reference flattened build artifacts
        let path_encoded = urlencoding::encode(&path).to_string();
        // Get the template itself
        let template = templates.get(&template_path);
        let template = match template {
            Some(template) => template,
            None => bail!(ErrorKind::PageNotFound(template_path))
        };
        // Create a locale detection file for it if we're using i18n
        // These just send the app shell, which will perform a redirect as necessary
        if locales.using_i18n {
            config_manager.write(&format!("exported/{}", path), &html_shell).await?;
        }
        // Check if that template uses build state (in which case it should have a JSON file)
        let has_state = template.uses_build_state();
        if locales.using_i18n {
            // Loop through all the app's locales
            for locale in locales.get_all() {
                let page_data = get_static_page_data(&format!("{}-{}", locale, &path_encoded), has_state, config_manager).await?;
                // Create a full HTML file from those that can be served for initial loads
                // The build process writes these with a dummy default locale even though we're not using i18n
                let full_html = interpolate_page_data(&html_shell, &page_data, root_id);
                // We don't add an extension because this will be queried directly
                config_manager.write(&format!("exported/{}/{}", locale, &path), &full_html).await?;

                // Serialize the page data to JSON and write it as a partial (fetched by the app shell for subsequent loads)
                let partial = serde_json::to_string(&page_data).unwrap();
                config_manager.write(&format!("exported/.perseus/page/{}/{}", locale, &path_encoded), &partial).await?;
            }
        } else {
            let page_data = get_static_page_data(&format!("{}-{}", locales.default, &path_encoded), has_state, config_manager).await?;
            // Create a full HTML file from those that can be served for initial loads
            // The build process writes these with a dummy default locale even though we're not using i18n
            let full_html = interpolate_page_data(&html_shell, &page_data, root_id);
            // We don't add an extension because this will be queried directly by the browser
            config_manager.write(&format!("exported/{}", &path), &full_html).await?;

            // Serialize the page data to JSON and write it as a partial (fetched by the app shell for subsequent loads)
            let partial = serde_json::to_string(&page_data).unwrap();
            config_manager.write(&format!("exported/.perseus/page/{}/{}", locales.default, &path_encoded), &partial).await?;
        }
    }

    Ok(())
}
