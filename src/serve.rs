// This file contains the universal logic for a serving process, regardless of framework

use std::fs;
use serde::{Serialize, Deserialize};
use crate::errors::*;
use crate::render_cfg::RenderCfg;
use crate::config_manager::ConfigManager;

/// Represents the data necessary to render a page.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageData {
    /// Prerendered HTML content.
    pub content: String,
    /// The state for hydration. This is kept as a string for ease of typing. Some pages may not need state or generate it in another way,
    /// so this might be `None`.
    pub state: Option<String>
}

/// Gets the configuration of how to render each page.
pub fn get_render_cfg() -> Result<RenderCfg> {
    let content = fs::read_to_string("../app/dist/render_conf.json")?;
    let cfg = serde_json::from_str::<RenderCfg>(&content)?;

    Ok(cfg)
}

/// Gets the HTML/JSON data for the given page path. This will call SSG/SSR/etc., whatever is needed for that page.
pub fn get_page(raw_path: &str, render_cfg: &RenderCfg, config_manager: &impl ConfigManager) -> Result<PageData> {
    // Remove `/` from the path by encoding it as a URL (that's what we store)
    let path = urlencoding::encode(raw_path).to_string();
    // TODO Match the path to one of the templates
    // TODO support SSR

    // Get the static HTML
    let html = config_manager.read(&format!("../app/dist/static/{}.html", path))?;
    // Get the static JSON
    let state = config_manager.read(&format!("../app/dist/static/{}.json", path));

    // Combine everything into one JSON object
    let res = PageData {
        content: html,
        state: match state {
            Ok(state) => Some(state),
            // TODO bail on errors that aren't `NotFound`
            Err(err) => None
        },
    };

    Ok(res)
}
