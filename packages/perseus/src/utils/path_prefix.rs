/// Gets the path prefix to apply in the browser. This uses the HTML `<base>`
/// element, which would be required anyway to make Sycamore's router co-operate
/// with a relative path hosting.
#[cfg(any(client, doc))]
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
