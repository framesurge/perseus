use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

mod templates;

#[cfg(client)]
pub mod components;

#[cfg(engine)]
mod server;

// Shared types (used by both client and server)
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T = serde_json::Value> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[cfg(engine)]
use server::custom_server;

#[perseus::main(custom_server)]
pub fn main() -> PerseusApp {
    PerseusApp::new()
        .template(templates::index::get_template())
        .template(templates::register::get_template())
        .template(templates::login::get_template())
        .template(templates::posts::get_template())
        .template(templates::post_view::get_template())
        .template(templates::categories::get_template())
        .template(templates::tags::get_template())
        .template(templates::profile::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
        .index_view(|| {
            view! {
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        title { "Perseus Membership System" }
                        style {
                            r#"
                                * { margin: 0; padding: 0; box-sizing: border-box; }
                                body {
                                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                    min-height: 100vh;
                                    padding: 2rem;
                                }
                                .container { max-width: 500px; margin: 0 auto; }
                                h1 { color: #333; margin-bottom: 1.5rem; text-align: center; }
                                h2 { color: #444; margin-bottom: 1rem; }
                                .card {
                                    background: white;
                                    padding: 2rem;
                                    border-radius: 12px;
                                    box-shadow: 0 10px 40px rgba(0,0,0,0.2);
                                    margin-bottom: 1.5rem;
                                }
                                .form-group { margin-bottom: 1.5rem; }
                                label {
                                    display: block;
                                    margin-bottom: 0.5rem;
                                    color: #555;
                                    font-weight: 500;
                                }
                                input {
                                    width: 100%;
                                    padding: 0.875rem;
                                    border: 2px solid #e0e0e0;
                                    border-radius: 8px;
                                    font-size: 1rem;
                                    transition: border-color 0.2s, box-shadow 0.2s;
                                }
                                input:focus {
                                    outline: none;
                                    border-color: #667eea;
                                    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
                                }
                                button {
                                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                    color: white;
                                    padding: 0.875rem 2rem;
                                    border: none;
                                    border-radius: 8px;
                                    font-size: 1rem;
                                    font-weight: 600;
                                    cursor: pointer;
                                    width: 100%;
                                    transition: transform 0.2s, box-shadow 0.2s;
                                }
                                button:hover:not(:disabled) {
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
                                }
                                button:disabled {
                                    opacity: 0.7;
                                    cursor: not-allowed;
                                }
                                .message {
                                    padding: 1rem;
                                    border-radius: 8px;
                                    margin-top: 1rem;
                                    text-align: center;
                                }
                                .message.success {
                                    background: #d4edda;
                                    color: #155724;
                                    border: 1px solid #c3e6cb;
                                }
                                .message.error {
                                    background: #f8d7da;
                                    color: #721c24;
                                    border: 1px solid #f5c6cb;
                                }
                                .nav-links {
                                    text-align: center;
                                    margin-top: 1rem;
                                }
                                .nav-links a {
                                    color: #667eea;
                                    text-decoration: none;
                                    margin: 0 0.5rem;
                                    font-weight: 500;
                                }
                                .nav-links a:hover {
                                    text-decoration: underline;
                                }
                                .otp-input {
                                    text-align: center;
                                    font-size: 1.5rem;
                                    letter-spacing: 0.5rem;
                                }
                                .secondary-btn {
                                    background: transparent;
                                    color: #667eea;
                                    border: 2px solid #667eea;
                                    margin-top: 1rem;
                                }
                                .secondary-btn:hover:not(:disabled) {
                                    background: rgba(102, 126, 234, 0.1);
                                    transform: none;
                                    box-shadow: none;
                                }
                                /* WYSIWYG Editor Styles */
                                .wysiwyg-editor {
                                    border: 2px solid #e0e0e0;
                                    border-radius: 8px;
                                    overflow: hidden;
                                    background: white;
                                }
                                .wysiwyg-toolbar {
                                    display: flex;
                                    flex-wrap: wrap;
                                    gap: 4px;
                                    padding: 8px;
                                    background: #f8f9fa;
                                    border-bottom: 1px solid #e0e0e0;
                                }
                                .toolbar-group {
                                    display: flex;
                                    gap: 2px;
                                }
                                .toolbar-separator {
                                    width: 1px;
                                    background: #ddd;
                                    margin: 0 6px;
                                }
                                .wysiwyg-toolbar button {
                                    width: 32px;
                                    height: 32px;
                                    padding: 4px;
                                    background: white;
                                    border: 1px solid #ddd;
                                    border-radius: 4px;
                                    cursor: pointer;
                                    font-size: 14px;
                                    font-weight: 600;
                                    color: #333;
                                    transition: all 0.15s ease;
                                }
                                .wysiwyg-toolbar button:hover {
                                    background: #e9ecef;
                                    border-color: #adb5bd;
                                    transform: none;
                                    box-shadow: none;
                                }
                                .wysiwyg-toolbar button.active {
                                    background: #667eea;
                                    color: white;
                                    border-color: #667eea;
                                }
                                .wysiwyg-toolbar .toolbar-italic {
                                    font-style: italic;
                                }
                                .wysiwyg-toolbar .toolbar-underline {
                                    text-decoration: underline;
                                }
                                .wysiwyg-toolbar .toolbar-strike {
                                    text-decoration: line-through;
                                }
                                .wysiwyg-content {
                                    padding: 16px;
                                    min-height: 200px;
                                    outline: none;
                                    line-height: 1.6;
                                }
                                .wysiwyg-content:focus {
                                    box-shadow: inset 0 0 0 2px rgba(102, 126, 234, 0.2);
                                }
                                .wysiwyg-content:empty:before {
                                    content: attr(data-placeholder);
                                    color: #adb5bd;
                                    pointer-events: none;
                                }
                                .wysiwyg-content img {
                                    max-width: 100%;
                                    height: auto;
                                    border-radius: 4px;
                                    margin: 8px 0;
                                }
                                .wysiwyg-content blockquote {
                                    border-left: 4px solid #667eea;
                                    margin: 1em 0;
                                    padding: 0.5em 1em;
                                    background: #f8f9fa;
                                }
                                .wysiwyg-content pre {
                                    background: #2d2d2d;
                                    color: #f8f8f2;
                                    padding: 1em;
                                    border-radius: 4px;
                                    overflow-x: auto;
                                    font-family: 'Monaco', 'Consolas', monospace;
                                }
                                .wysiwyg-content code {
                                    background: #f1f3f4;
                                    padding: 2px 6px;
                                    border-radius: 3px;
                                    font-family: 'Monaco', 'Consolas', monospace;
                                    font-size: 0.9em;
                                }
                                .wysiwyg-content pre code {
                                    background: transparent;
                                    padding: 0;
                                }
                                .wysiwyg-content ul, .wysiwyg-content ol {
                                    margin: 1em 0;
                                    padding-left: 2em;
                                }
                                .wysiwyg-content h1, .wysiwyg-content h2, .wysiwyg-content h3 {
                                    margin: 1em 0 0.5em 0;
                                    color: #333;
                                }
                                .wysiwyg-content h1 { font-size: 2em; }
                                .wysiwyg-content h2 { font-size: 1.5em; }
                                .wysiwyg-content h3 { font-size: 1.25em; }
                                .wysiwyg-content a {
                                    color: #667eea;
                                    text-decoration: underline;
                                }
                                .wysiwyg-content hr {
                                    border: none;
                                    border-top: 2px solid #e0e0e0;
                                    margin: 1.5em 0;
                                }
                                /* Link Modal */
                                .link-modal-overlay {
                                    position: fixed;
                                    top: 0;
                                    left: 0;
                                    right: 0;
                                    bottom: 0;
                                    background: rgba(0, 0, 0, 0.5);
                                    display: flex;
                                    align-items: center;
                                    justify-content: center;
                                    z-index: 1000;
                                }
                                .link-modal {
                                    background: white;
                                    padding: 24px;
                                    border-radius: 12px;
                                    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
                                    min-width: 400px;
                                }
                                .link-modal h3 {
                                    margin-bottom: 16px;
                                    color: #333;
                                }
                                .link-modal input {
                                    margin-bottom: 16px;
                                }
                                .link-modal-buttons {
                                    display: flex;
                                    gap: 12px;
                                    justify-content: flex-end;
                                }
                                .link-modal-buttons button {
                                    width: auto;
                                    padding: 10px 20px;
                                }
                                .upload-progress {
                                    color: #667eea;
                                    font-size: 0.875rem;
                                    padding: 8px;
                                    text-align: center;
                                }
                                .hidden-file-input {
                                    display: none;
                                }
                            "#
                        }
                    }
                    body {
                        PerseusRoot()
                    }
                }
            }
        })
}
