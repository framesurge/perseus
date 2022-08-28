/// Replaces the current document `<head>` after the delimiter comment
/// with something new.
pub(crate) fn replace_head(new: &str) {
    // Get the current head
    let head_elem = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("head")
        .unwrap()
        .unwrap();
    let head_html = head_elem.inner_html();
    // We'll assume that there's already previously interpolated head in
    // addition to the hardcoded stuff, but it will be separated by the
    // server-injected delimiter comment
    // Thus, we replace the stuff after that delimiter comment with the
    // new head
    let head_parts: Vec<&str> = head_html
        .split("<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->")
        .collect();
    let new_head = format!(
        "{}\n<!--PERSEUS_INTERPOLATED_HEAD_BEGINS-->\n{}",
        head_parts[0], new
    );
    head_elem.set_inner_html(&new_head);
}
