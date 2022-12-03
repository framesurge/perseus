use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the data necessary to render a page, including document metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageData {
    /// Prerendered HTML content.
    pub content: String,
    /// The state for hydration.
    pub state: Value,
    /// The states of all the widgets involved in rendering this page. This will not
    /// include the states of delayed widgets.
    ///
    /// This is a map of widget path to capsule name and state, preventing the need to
    /// run route resolution algorithms on the browser-side.
    pub widget_states: HashMap<String, (String, Value)>,
    /// The string to interpolate into the document's `<head>`.
    pub head: String,
}

/// A version of [`PageData`] that doesn't contain the HTML content of the page.
/// This is designed for being to sent to the client as part of a subsequent
/// load, since the browser can render the HTML content of a page on its own.
///
/// Note that this still contains the `<head>`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageDataPartial {
    /// The state for hydration.
    pub state: Value,
    /// The string to interpolate into the document's `<head>`.
    pub head: String,
}
