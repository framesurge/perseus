use std::{collections::HashMap, convert::Infallible};

use warp::{
    http::Response,
    path::{FullPath, Tail},
    Filter, Rejection,
};

/// A filter for static content directories (not aliases) that determines which static directory to serve.
pub fn static_dirs_filter(
    paths: HashMap<String, String>,
) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::any()
        .and(warp::path::tail())
        .and(warp::any().map(move || paths.clone()))
        .and_then(|path: Tail, paths: HashMap<String, String>| async move {
            // Match a specific static directory and break on the first match
            let mut dir_to_serve = String::new();
            for (url, static_dir) in paths.iter() {
                if &path.as_str() == url {
                    dir_to_serve = static_dir.to_string();
                    break;
                }
            }

            if dir_to_serve.is_empty() {
                Err(warp::reject::not_found())
            } else {
                Ok(dir_to_serve)
            }
        })
}

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
                    if &path.as_str() == url {
                        file_to_serve = static_dir.to_string();
                        break;
                    }
                }

                if file_to_serve.is_empty() {
                    Err(warp::reject::not_found())
                } else {
                    Ok(file_to_serve)
                }
            },
        )
}

/// A handler for the static aliases.
pub fn static_aliases_handler(file_to_serve: String) -> Result<Response<String>, Infallible> {
    // Ok(warp::fs::file(file_to_serve));
    todo!()
}

/// A handler for static content directories.
pub fn static_dirs_handler(dir_to_serve: String) -> Result<Response<String>, Infallible> {
    // Ok(warp::fs::dir(dir_to_serve));
    todo!()
}
