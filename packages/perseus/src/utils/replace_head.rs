use wasm_bindgen::JsCast;
use web_sys::Element;

/// Replaces the current document `<head>` after the delimiter comment
/// with something new.
///
/// This will only touch the dynamic elements, thus avoiding re-requesting
/// any resources references in the component of the `<head>` shared between
/// all pages, which may create a flash of unstyled content.
pub(crate) fn replace_head(new: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    // Get the current head
    let head_node = document.query_selector("head").unwrap().unwrap();
    let head_elem: Element = head_node.unchecked_into();
    // Get everything after the dummy `<meta>` tag we use as a delimiter
    let els_to_remove = document
        .query_selector_all(r#"meta[itemprop='__perseus_head_boundary'] ~ *"#)
        .unwrap();
    // For some horrific reason, this isn't implemented as an iterator in `web_sys`
    for idx in 0..(els_to_remove.length()) {
        let el = els_to_remove.get(idx).unwrap();
        head_elem.remove_child(&el);
    }
    // And now append the new HTML to the head (yes, this position is untyped...)
    // This position is inside the element, after its last child
    head_elem.insert_adjacent_html("beforeend", new);
}
