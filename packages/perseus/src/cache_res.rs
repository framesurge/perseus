use std::any::Any;
use std::convert::Infallible;

use futures::Future;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Runs the given function once and then caches the result to the filesystem for future execution. Think of this as filesystem-level memoizing. In future, this will be broken out into
/// its own crate and wrapped by Perseus. The second parameter to this allows forcing the function to re-fetch data every time, which is useful if you want to revalidate data or test
/// your fetching logic again. Note that a change to the logic will not trigger a reload unless you make it do so. For this reason, it's recommended to only use this wrapper once
/// you've tested your fetching logic.
///
/// When running automated tests, you may wish to set `force_run` to the result of an environment variable check that you'll use when testing.
///
/// This function expects to be run in the context of `.perseus/`, or any directory in which a folder `cache/` is available. If you're using Perseus without the CLI and you don't want
/// that directory to exist, you shouldn't use this function.
///
/// # Panics
/// If this filesystem operations fail, this function will panic. It can't return a graceful error since it's expected to return the type you requested.
pub async fn cache_res<D, F, Ft>(name: &str, f: F, force_run: bool) -> D
where
    // By making this `Any`, we can downcast it to manage errors intuitively
    D: Serialize + for<'de> Deserialize<'de> + Any,
    F: Fn() -> Ft,
    Ft: Future<Output = D>,
{
    let f_res = || async { Ok::<D, Infallible>(f().await) };
    // This can't fail, we just invented an error type for an infallible function
    cache_fallible_res(name, f_res, force_run).await.unwrap()
}

/// Same as `cache_res`, but takes a function that returns a `Result`, allowing you to use `?` and the like inside your logic.
pub async fn cache_fallible_res<D, E, F, Ft>(name: &str, f: F, force_run: bool) -> Result<D, E>
where
    // By making this `Any`, we can downcast it to manage errors intuitively
    D: Serialize + for<'de> Deserialize<'de>,
    E: std::error::Error,
    F: Fn() -> Ft,
    Ft: Future<Output = Result<D, E>>,
{
    // Replace any slashes with dashes to keep a flat directory structure
    let name = name.replace("/", "-");
    // In production, we'll just run the function directly
    if cfg!(debug_assertions) {
        // Check if the cache file exists
        let filename = format!("cache/{}.json", &name);
        match File::open(&filename).await {
            Ok(mut file) => {
                if force_run {
                    let res = f().await?;
                    // Now cache the result
                    let str_res = serde_json::to_string(&res).unwrap_or_else(|err| {
                        panic!(
                            "couldn't serialize result of entry '{}' for caching: {}",
                            &filename, err
                        )
                    });
                    let mut file = File::create(&filename).await.unwrap_or_else(|err| {
                        panic!(
                            "couldn't create cache file for entry '{}': {}",
                            &filename, err
                        )
                    });
                    file.write_all(str_res.as_bytes())
                        .await
                        .unwrap_or_else(|err| {
                            panic!(
                                "couldn't write cache to file for entry '{}': {}",
                                &filename, err
                            )
                        });

                    Ok(res)
                } else {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .await
                        .unwrap_or_else(|err| {
                            panic!(
                                "couldn't read cache from file for entry '{}': {}",
                                &filename, err
                            )
                        });
                    let res: D = match serde_json::from_str(&contents) {
                        Ok(cached_res) => cached_res,
                        // If the stuff in the cache can't be deserialized, we'll force a recreation (we don't recurse because that requires boxing the future)
                        Err(_) => {
                            let res = f().await?;
                            // Now cache the result
                            let str_res = serde_json::to_string(&res).unwrap_or_else(|err| {
                                panic!(
                                    "couldn't serialize result of entry '{}' for caching: {}",
                                    &filename, err
                                )
                            });
                            let mut file = File::create(&filename).await.unwrap_or_else(|err| {
                                panic!(
                                    "couldn't create cache file for entry '{}': {}",
                                    &filename, err
                                )
                            });
                            file.write_all(str_res.as_bytes())
                                .await
                                .unwrap_or_else(|err| {
                                    panic!(
                                        "couldn't write cache to file for entry '{}': {}",
                                        &filename, err
                                    )
                                });

                            res
                        }
                    };

                    Ok(res)
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                // The file doesn't exist yet, create the parent cache directory
                create_dir_all("cache")
                    .await
                    .unwrap_or_else(|err| panic!("couldn't create cache directory: {}", err));
                // We have no cache, so we'll have to run the function
                let res = f().await?;
                // Now cache the result
                let str_res = serde_json::to_string(&res).unwrap_or_else(|err| {
                    panic!(
                        "couldn't serialize result of entry '{}' for caching: {}",
                        &filename, err
                    )
                });
                let mut file = File::create(&filename).await.unwrap_or_else(|err| {
                    panic!(
                        "couldn't create cache file for entry '{}': {}",
                        &filename, err
                    )
                });
                file.write_all(str_res.as_bytes())
                    .await
                    .unwrap_or_else(|err| {
                        panic!(
                            "couldn't write cache to file for entry '{}': {}",
                            &filename, err
                        )
                    });

                Ok(res)
            }
            // Any other filesystem errors are unacceptable
            Err(err) => panic!(
                "filesystem error occurred while trying to read cache file for entry '{}': {}",
                &filename, err
            ),
        }
    } else {
        f().await
    }
}
