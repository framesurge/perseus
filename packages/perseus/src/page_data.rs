use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the data necessary to render a page, including document metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageData {
    /// Prerendered HTML content.
    pub content: String,
    /// The state for hydration.
    pub state: Value,
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
