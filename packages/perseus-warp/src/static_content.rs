use std::collections::HashMap;
use std::sync::Arc;
use warp::fs::{file_reply, ArcPath, Conditionals, File};
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
                    Ok(file_to_serve)
                }
            },
        )
}

/// Serves the file provided through the filter.
pub async fn serve_file(path: String) -> Result<File, Rejection> {
    let arc_path = ArcPath(Arc::new(path.into()));
    let conds = Conditionals::default();
    file_reply(arc_path, conds).await
}

// /// Serves the file provided through the filter. This returns an error because we assume that the file is supposed to exist at this point (this is used for static
// /// aliases).
// pub async fn serve_file(path: String) -> Result<Response, Rejection> {
//     match TkFile::open(path).await {
//         Ok(file) => {
//             let metadata = file.metadata().await.map_err(|e| warp::reject::not_found())?;
//             let stream = file_stream(file, metadata);
//             let res = Response::new(Body::wrap_stream(stream));

//             Ok(res)
//         },
//         // If a static alias can't be found, we'll act as if it doesn't exist and proceed to the next handler
//         Err(_) => Err(warp::reject::not_found())
//     }
// }

// // The default chunk size for streaming a file (taken from Warp's internals)
// const DFLT_BUF_SIZE: usize = 8_192;
// #[cfg(unix)]
// fn get_buf_size(metadata: Metadata) -> usize {
//     use std::os::unix::prelude::MetadataExt;

//     std::cmp::max(metadata.blksize() as usize, DFLT_BUF_SIZE)
// }
// #[cfg(not(unix))]
// fn get_buf_size(_metadata: Metadata) -> usize {
//     DFLT_BUF_SIZE // On Windows, we don't have a blocksize function based on the metadata
// }

// /// Reserves more space in a buffer if needed
// fn reserve_if_needed(buf: &mut BytesMut, cap: usize) {
//     if buf.capacity() - buf.len() < cap {
//         buf.reserve(cap);
//     }
// }

// fn file_stream(mut file: TkFile, metadata: Metadata) -> impl Stream<Item = Result<Bytes, std::io::Error>> + Send {
//     let buf_size = get_buf_size(metadata);
//     let stream = file.seek(SeekFrom::Start(0));

//     let mut buf = BytesMut::new();
//     reserve_if_needed(&mut buf, buf_size);

//     try_stream! {
//         for i in 0u8..3 {
//             reserve_if_needed(&mut buf, buf_size);
//             let n = file.read(&mut buf).await?;
//             yield Bytes::from(buf[..n]);
//         }
//     }
// }
