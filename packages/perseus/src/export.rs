use crate::errors::*;
use crate::ConfigManager;
use crate::TemplateMap;
use crate::SsrNode;
use crate::Locales;
use crate::html_shell::prep_html_shell;
use crate::get_render_cfg;
use std::fs;

/// Creates a full HTML file, ready for initial loads, from the given data.
async fn create_full_html(
    html_path: String,
    json_path: Option<String>,
    html_shell: &str,
    config_manager: &impl ConfigManager
) -> Result<String> {
    // Get the partial HTML content and a state to go with it (if applicable)
    let content = config_manager.read(&html_path).await?;
    let state = match json_path {
        Some(json_path) => Some(config_manager.read(&json_path).await?),
        None => None
    };

    todo!()
}

/// Exports your app to static files, which can be served from anywhere, without needing a server. This assumes that the app has already
/// been built, and that no templates are using non-static features (which can be ensured by passing `true` as the last parameter to
/// `build_app`).
pub async fn export_app(
    templates: TemplateMap<SsrNode>,
    html_shell_path: &str,
    locales: &Locales,
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
        // Get the template itself
        let template = templates.get(&template_path);
        let template = match template {
            Some(template) => template,
            None => bail!(ErrorKind::PageNotFound(template_path))
        };
        // Create a locale detection file for it if we're using i18n
        // These just send the app shell, which will perform a redirect as necessary
        // TODO test this on the i18n example
        if locales.using_i18n {
            config_manager.write(&format!("exported/{}", path), &html_shell).await?;
        }
        // Check if that template uses build state (in which case it should have a JSON file)
        let has_json = template.uses_build_state();
        if locales.using_i18n {
            // Loop through all the app's locales
            todo!()
        } else {
            let html_path = format!("exported/{}.html", path);
            let json_path = match has_json {
                true => Some(format!("exported/{}.json", path)),
                false => None
            };
            // Create from those a full HTMl file that can be served for initial loads
            let full_html = create_full_html(html_path, json_path, &html_shell, config_manager).await?;
        }
        // Read the HTML partial (doesn't have a shell yet)
        // Interpolate the HTML partial into the shell
        // Set the initial state accordingly as to the use of build state
        // Interpolate that initial state into the document `<head>`
        // Write the full file
    }

    Ok(())
}
