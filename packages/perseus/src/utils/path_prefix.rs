/// Gets the path prefix to apply on the server. This uses the
/// `PERSEUS_BASE_PATH` environment variable, which avoids hardcoding
/// something as changeable as this into the final binary. Hence however, that
/// variable must be the same as what's set in `<base>` (done automatically).
/// Trailing forward slashes will be trimmed automatically.
#[cfg(engine)]
pub fn get_path_prefix_server() -> String {
    use std::env;

    let base_path = env::var("PERSEUS_BASE_PATH").unwrap_or_else(|_| "".to_string());
    base_path
        .strip_suffix('/')
        .unwrap_or(&base_path)
        .to_string()
}

/// Gets the path prefix to apply in the browser. This uses the HTML `<base>`
/// element, which would be required anyway to make Sycamore's router co-operate
/// with a relative path hosting.
#[cfg(client)]
pub fn get_path_prefix_client() -> String {
    use wasm_bindgen::JsCast;
    use web_sys::{HtmlBaseElement, Url};

    let base_path = match web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("base[href]")
    {
        Ok(Some(base)) => {
            let base = base.unchecked_into::<HtmlBaseElement>().href();

            let url = Url::new(&base).unwrap();
            url.pathname()
        }
        _ => "".to_string(),
    };
    // Strip any trailing slashes, a `//` makes the browser query `https://.perseus/...`
    base_path
        .strip_suffix('/')
        .unwrap_or(&base_path)
        .to_string()
}
