use crate::errors::*;
use crate::stores::ImmutableStore;
use std::collections::HashMap;

/// Gets the configuration of how to render each page using an immutable store.
///
/// The render configuration is an internal build artifact stored somewhere like
/// `dist/`, generated automatically by the build process. The server provides
/// it automatically to the client to optimize routing.
pub async fn get_render_cfg(
    immutable_store: &ImmutableStore,
) -> Result<HashMap<String, String>, ServerError> {
    let content = immutable_store.read("render_conf.json").await?;
    let cfg = serde_json::from_str::<HashMap<String, String>>(&content).map_err(|e| {
        // We have to convert it into a build error and then into a server error
        let build_err: BuildError = e.into();
        build_err
    })?;

    Ok(cfg)
}
