use std::collections::HashMap;

use warp::{path::FullPath, Filter, Rejection};

/// A filter for static aliases that determines which file to serve.
pub fn static_aliases_filter(
    paths: HashMap<String, String>,
) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::any()
        .and(warp::path::full())
        .and(warp::any().map(move || paths.clone()))
        .and_then(
            |path: FullPath, paths: HashMap<String, String>| async move {
                // Match a specific static alias and break on the first match
                let mut file_to_serve = String::new();
                for (url, static_dir) in paths.iter() {
                    if path.as_str() == url {
                        file_to_serve = static_dir.to_string();
                        break;
                    }
                }

                if file_to_serve.is_empty() {
                    Err(warp::reject::not_found())
                } else {
                    // TODO Return the actual file (see Warp internals for this)
                    Ok(file_to_serve)
                }
            },
        )
}
