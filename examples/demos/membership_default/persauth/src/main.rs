use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

mod templates;
mod types;

#[cfg(client)]
pub mod components;

#[cfg(engine)]
mod server;

// Re-export shared types from types module
pub use types::ApiResponse;

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
        .template(templates::create_post::get_template())
        .template(templates::categories::get_template())
        .template(templates::tags::get_template())
        .template(templates::profile::get_template())
        .template(templates::contact::get_template())
        .template(templates::dashboard::get_template())
        .template(templates::admin::get_template())
        .template(templates::preferences::get_template())
        .template(templates::logout::get_template())
        .template(templates::roles::get_template())
        .template(templates::support::get_template())
        .template(templates::users::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
        .index_view(|| {
            view! {
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        title { "Perseus Membership System" }
                        link(rel = "stylesheet", href = "/.perseus/static/css/bootstrap.css")
                        link(rel = "stylesheet", href = "/.perseus/static/css/fontawesome.css")
                        link(rel = "stylesheet", href = "/.perseus/static/css/feather-icon.css")
                        link(rel = "stylesheet", href = "/.perseus/static/css/style.css")
                        link(rel = "stylesheet", href = "/.perseus/static/css/responsive.css")
                        style {
                            r#"
                                /* App-specific overrides and editor styles */
                                /* sycawysgy Editor Styles */
                                .editor-container {
                                    border: 2px solid #e0e0e0;
                                    border-radius: 8px;
                                    overflow: hidden;
                                    background: white;
                                }
                                .editor-toolbar {
                                    display: flex;
                                    flex-wrap: wrap;
                                    gap: 4px;
                                    padding: 8px;
                                    background: #f8f9fa;
                                    border-bottom: 1px solid #e0e0e0;
                                    align-items: center;
                                }
                                /* .toolbar is the actual class sycawysgy uses */
                                .toolbar {
                                    display: flex;
                                    flex-wrap: wrap;
                                    gap: 4px;
                                    padding: 8px;
                                    background: #f8f9fa;
                                    border-bottom: 1px solid #e0e0e0;
                                    align-items: center;
                                }
                                .toolbar-btn {
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
                                    display: flex;
                                    align-items: center;
                                    justify-content: center;
                                }
                                .toolbar-btn:hover {
                                    background: #e9ecef;
                                    border-color: #adb5bd;
                                    transform: none;
                                    box-shadow: none;
                                }
                                .toolbar-btn.active {
                                    background: #667eea;
                                    color: white;
                                    border-color: #667eea;
                                }
                                .toolbar-btn:disabled {
                                    opacity: 0.5;
                                    cursor: not-allowed;
                                }
                                .toolbar-separator {
                                    width: 1px;
                                    background: #ddd;
                                    margin: 0 6px;
                                    height: 24px;
                                }
                                .editor-content {
                                    padding: 16px;
                                    min-height: 250px;
                                    outline: none;
                                    line-height: 1.6;
                                }
                                .editor-content:focus {
                                    box-shadow: inset 0 0 0 2px rgba(102, 126, 234, 0.2);
                                }
                                .editor-content:empty:before {
                                    content: attr(data-placeholder);
                                    color: #adb5bd;
                                    pointer-events: none;
                                }
                                .editor-content img {
                                    max-width: 100%;
                                    height: auto;
                                    border-radius: 4px;
                                    margin: 8px 0;
                                }
                                .editor-content blockquote {
                                    border-left: 4px solid #667eea;
                                    margin: 1em 0;
                                    padding: 0.5em 1em;
                                    background: #f8f9fa;
                                }
                                .editor-content pre {
                                    background: #2d2d2d;
                                    color: #f8f8f2;
                                    padding: 1em;
                                    border-radius: 4px;
                                    overflow-x: auto;
                                    font-family: 'Monaco', 'Consolas', monospace;
                                }
                                .editor-content code {
                                    background: #f1f3f4;
                                    padding: 2px 6px;
                                    border-radius: 3px;
                                    font-family: 'Monaco', 'Consolas', monospace;
                                    font-size: 0.9em;
                                }
                                .editor-content pre code {
                                    background: transparent;
                                    padding: 0;
                                }
                                .editor-content ul, .editor-content ol {
                                    margin: 1em 0;
                                    padding-left: 2em;
                                }
                                .editor-content h1, .editor-content h2, .editor-content h3 {
                                    margin: 1em 0 0.5em 0;
                                    color: #333;
                                }
                                .editor-content h1 { font-size: 2em; }
                                .editor-content h2 { font-size: 1.5em; }
                                .editor-content h3 { font-size: 1.25em; }
                                .editor-content a {
                                    color: #667eea;
                                    text-decoration: underline;
                                }
                                .editor-content hr {
                                    border: none;
                                    border-top: 2px solid #e0e0e0;
                                    margin: 1.5em 0;
                                }
                                /* Dropdown menu for headings */
                                .heading-dropdown {
                                    position: relative;
                                }
                                .heading-dropdown-menu {
                                    position: absolute;
                                    top: 100%;
                                    left: 0;
                                    background: white;
                                    border: 1px solid #ddd;
                                    border-radius: 4px;
                                    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                                    z-index: 100;
                                    min-width: 120px;
                                }
                                .heading-dropdown-item {
                                    padding: 8px 12px;
                                    cursor: pointer;
                                    transition: background 0.15s;
                                }
                                .heading-dropdown-item:hover {
                                    background: #f0f0f0;
                                }
                                /* Image upload indicator */
                                .image-upload-indicator {
                                    position: fixed;
                                    bottom: 20px;
                                    right: 20px;
                                    background: #667eea;
                                    color: white;
                                    padding: 12px 20px;
                                    border-radius: 8px;
                                    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
                                    z-index: 1000;
                                    display: none;
                                }
                                .image-upload-indicator.visible {
                                    display: block;
                                }
                                /* Generic Modal Overlay (used by demo admin pages) */
                                .link-modal-overlay {
                                    position: fixed;
                                    top: 0;
                                    left: 0;
                                    right: 0;
                                    bottom: 0;
                                    padding: 1.5rem;
                                    background: rgba(0, 0, 0, 0.55);
                                    display: flex;
                                    justify-content: center;
                                    align-items: center;
                                    z-index: 1100;
                                }
                                @media (max-width: 640px) {
                                    .link-modal-overlay {
                                        padding: 0.75rem;
                                    }
                                }
                                /* Link Dialog Styles */
                                .link-dialog-overlay {
                                    position: fixed;
                                    top: 0;
                                    left: 0;
                                    right: 0;
                                    bottom: 0;
                                    background: rgba(0, 0, 0, 0.5);
                                    display: flex;
                                    justify-content: center;
                                    align-items: center;
                                    z-index: 1000;
                                }
                                .link-dialog-overlay.hidden {
                                    display: none;
                                }
                                .link-dialog {
                                    background: white;
                                    padding: 24px;
                                    border-radius: 12px;
                                    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
                                    min-width: 320px;
                                    max-width: 400px;
                                }
                                .link-dialog h3 {
                                    margin: 0 0 16px 0;
                                    font-size: 1.25rem;
                                    color: #333;
                                }
                                .link-url-input {
                                    width: 100%;
                                    padding: 12px;
                                    border: 2px solid #e0e0e0;
                                    border-radius: 8px;
                                    font-size: 1rem;
                                    margin-bottom: 16px;
                                    box-sizing: border-box;
                                }
                                .link-url-input:focus {
                                    outline: none;
                                    border-color: #667eea;
                                    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
                                }
                                .link-dialog-buttons {
                                    display: flex;
                                    gap: 12px;
                                    justify-content: flex-end;
                                }
                                .link-dialog-button {
                                    padding: 10px 20px;
                                    border: none;
                                    border-radius: 8px;
                                    font-size: 0.875rem;
                                    font-weight: 600;
                                    cursor: pointer;
                                    background: #e0e0e0;
                                    color: #333;
                                    transition: all 0.15s ease;
                                }
                                .link-dialog-button:hover {
                                    background: #d0d0d0;
                                }
                                .link-dialog-button-primary {
                                    background: #667eea;
                                    color: white;
                                }
                                .link-dialog-button-primary:hover {
                                    background: #5a6fd6;
                                }
                                /* Clear Confirmation Dialog Styles */
                                .confirm-dialog-overlay {
                                    position: fixed;
                                    top: 0;
                                    left: 0;
                                    right: 0;
                                    bottom: 0;
                                    background: rgba(0, 0, 0, 0.5);
                                    display: flex;
                                    justify-content: center;
                                    align-items: center;
                                    z-index: 1000;
                                }
                                .confirm-dialog {
                                    background: white;
                                    padding: 24px;
                                    border-radius: 12px;
                                    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
                                    min-width: 320px;
                                    max-width: 400px;
                                }
                                .confirm-dialog h3 {
                                    margin: 0 0 12px 0;
                                    font-size: 1.25rem;
                                    color: #333;
                                }
                                .confirm-dialog p {
                                    color: #666;
                                    margin: 0 0 20px 0;
                                    line-height: 1.5;
                                }
                                .confirm-dialog-buttons {
                                    display: flex;
                                    gap: 12px;
                                    justify-content: flex-end;
                                }
                                .confirm-dialog-button {
                                    padding: 10px 20px;
                                    border: none;
                                    border-radius: 8px;
                                    font-size: 0.875rem;
                                    font-weight: 600;
                                    cursor: pointer;
                                    background: #e0e0e0;
                                    color: #333;
                                    transition: all 0.15s ease;
                                }
                                .confirm-dialog-button:hover {
                                    background: #d0d0d0;
                                }
                                .confirm-dialog-button-danger {
                                    background: #dc3545;
                                    color: white;
                                }
                                .confirm-dialog-button-danger:hover {
                                    background: #c82333;
                                }
                                /* Viho template wrapper styles - classes require parent .page-wrapper */
                                .page-body {
                                    margin-left: 0;
                                    padding: 20px;
                                    min-height: calc(100vh - 60px);
                                }
                                .page-header {
                                    margin-bottom: 20px;
                                }
                                .page-header-left h3 {
                                    margin: 0 0 5px 0;
                                    font-size: 1.5rem;
                                    color: #333;
                                }
                                .page-header-left .d-block {
                                    display: block;
                                    color: #6c757d;
                                    font-size: 0.875rem;
                                }
                                .breadcrumb {
                                    display: flex;
                                    flex-wrap: wrap;
                                    padding: 0;
                                    margin: 0;
                                    list-style: none;
                                }
                                .breadcrumb-item {
                                    position: relative;
                                    padding-left: 20px;
                                }
                                .breadcrumb-item:before {
                                    content: "/";
                                    position: absolute;
                                    left: 5px;
                                    color: #6c757d;
                                }
                                .breadcrumb-item:first-child:before {
                                    content: none;
                                }
                                .breadcrumb-item a {
                                    color: #667eea;
                                    text-decoration: none;
                                }
                                .breadcrumb-item.active {
                                    color: #6c757d;
                                }
                                /* Card enhancements */
                                .card {
                                    border: none;
                                    border-radius: 8px;
                                    box-shadow: 0 2px 10px rgba(0,0,0,0.08);
                                    margin-bottom: 20px;
                                }
                                .card-header {
                                    background: white;
                                    border-bottom: 1px solid #eee;
                                    padding: 15px 20px;
                                    border-radius: 8px 8px 0 0;
                                }
                                .card-header h5 {
                                    margin: 0;
                                    font-size: 1.1rem;
                                    color: #333;
                                }
                                .card-body {
                                    padding: 20px;
                                }
                                /* Form controls */
                                .form-control {
                                    display: block;
                                    width: 100%;
                                    padding: 10px 15px;
                                    font-size: 1rem;
                                    line-height: 1.5;
                                    color: #495057;
                                    background-color: #fff;
                                    background-clip: padding-box;
                                    border: 1px solid #ced4da;
                                    border-radius: 4px;
                                    transition: border-color 0.15s ease-in-out;
                                }
                                .form-control:focus {
                                    border-color: #667eea;
                                    outline: none;
                                    box-shadow: 0 0 0 0.2rem rgba(102, 126, 234, 0.25);
                                }
                                .form-group {
                                    margin-bottom: 15px;
                                }
                                .form-group label {
                                    display: block;
                                    margin-bottom: 5px;
                                    color: #333;
                                    font-weight: 500;
                                }
                                /* Buttons */
                                .btn {
                                    display: inline-block;
                                    font-weight: 500;
                                    text-align: center;
                                    vertical-align: middle;
                                    padding: 10px 20px;
                                    font-size: 0.875rem;
                                    border-radius: 4px;
                                    cursor: pointer;
                                    transition: all 0.15s;
                                }
                                .btn-primary {
                                    color: #fff;
                                    background: #667eea;
                                    border: 1px solid #667eea;
                                }
                                .btn-primary:hover {
                                    background: #5a6fd6;
                                    border-color: #5a6fd6;
                                }
                                .btn-secondary {
                                    color: #fff;
                                    background: #6c757d;
                                    border: 1px solid #6c757d;
                                }
                                .btn-secondary:hover {
                                    background: #5a6268;
                                    border-color: #545b62;
                                }
                                .btn-info {
                                    color: #fff;
                                    background: #17a2b8;
                                    border: 1px solid #17a2b8;
                                }
                                .btn-danger {
                                    color: #fff;
                                    background: #dc3545;
                                    border: 1px solid #dc3545;
                                }
                                .btn-block {
                                    display: block;
                                    width: 100%;
                                }
                                /* Alerts */
                                .alert {
                                    padding: 15px 20px;
                                    margin-bottom: 20px;
                                    border-radius: 4px;
                                }
                                .alert-success {
                                    color: #155724;
                                    background-color: #d4edda;
                                    border: 1px solid #c3e6cb;
                                }
                                .alert-danger {
                                    color: #721c24;
                                    background-color: #f8d7da;
                                    border: 1px solid #f5c6cb;
                                }
                                .alert-info {
                                    color: #0c5460;
                                    background-color: #d1ecf1;
                                    border: 1px solid #bee5eb;
                                }
                                .alert-warning {
                                    color: #856404;
                                    background-color: #fff3cd;
                                    border: 1px solid #ffeeba;
                                }
                                /* Tables */
                                .table {
                                    width: 100%;
                                    margin-bottom: 1rem;
                                    color: #212529;
                                }
                                .table th,
                                .table td {
                                    padding: 12px;
                                    vertical-align: top;
                                    border-top: 1px solid #dee2e6;
                                }
                                .table thead th {
                                    vertical-align: bottom;
                                    border-bottom: 2px solid #dee2e6;
                                    background: #f8f9fa;
                                }
                                .table-responsive {
                                    display: block;
                                    width: 100%;
                                    overflow-x: auto;
                                }
                                /* Nav tabs */
                                .nav-tabs {
                                    display: flex;
                                    flex-wrap: wrap;
                                    padding-left: 0;
                                    margin-bottom: 0;
                                    list-style: none;
                                    border-bottom: 1px solid #dee2e6;
                                }
                                .nav-item {
                                    margin-bottom: -1px;
                                }
                                .nav-link {
                                    display: block;
                                    padding: 10px 20px;
                                    color: #495057;
                                    text-decoration: none;
                                    border: 1px solid transparent;
                                    border-top-left-radius: 4px;
                                    border-top-right-radius: 4px;
                                }
                                .nav-link:hover {
                                    color: #667eea;
                                }
                                .nav-link.active {
                                    color: #667eea;
                                    background-color: #fff;
                                    border-color: #dee2e6 #dee2e6 #fff;
                                }
                                /* Stats cards */
                                .o-hidden {
                                    overflow: hidden;
                                }
                                .d-flex {
                                    display: flex;
                                }
                                .flex-grow-1 {
                                    flex-grow: 1;
                                }
                                .f-w-600 {
                                    font-weight: 600;
                                }
                                .f-16 {
                                    font-size: 1rem;
                                }
                                .counter {
                                    font-size: 1.75rem;
                                    font-weight: 700;
                                    color: #333;
                                }
                                .f-28 {
                                    font-size: 1.75rem;
                                }
                                /* Profile styles */
                                .profile-details {
                                    text-align: center;
                                }
                                .rounded-circle {
                                    border-radius: 50%;
                                }
                                .mb-0 {
                                    margin-bottom: 0;
                                }
                                .m-r-10 {
                                    margin-right: 10px;
                                }
                                .m-t-20 {
                                    margin-top: 20px;
                                }
                                .m-b-20 {
                                    margin-bottom: 20px;
                                }
                                .profile-social {
                                    list-style: none;
                                    padding: 0;
                                    margin: 15px 0;
                                }
                                .profile-social li {
                                    margin-bottom: 10px;
                                }
                                /* Loader */
                                .loader-box {
                                    display: flex;
                                    justify-content: center;
                                    align-items: center;
                                    min-height: 200px;
                                }
                                .loader {
                                    width: 40px;
                                    height: 40px;
                                    border: 3px solid #f3f3f3;
                                    border-top: 3px solid #667eea;
                                    border-radius: 50%;
                                    animation: spin 1s linear infinite;
                                }
                                @keyframes spin {
                                    0% { transform: rotate(0deg); }
                                    100% { transform: rotate(360deg); }
                                }
                                /* Text utilities */
                                .text-muted {
                                    color: #6c757d;
                                }
                                .text-center {
                                    text-align: center;
                                }
                                /* Media utilities */
                                .media {
                                    display: flex;
                                    align-items: flex-start;
                                }
                                .media-body {
                                    flex: 1;
                                }
                                /* FAQ styles */
                                .faq-item h6 {
                                    margin: 0 0 5px 0;
                                    color: #333;
                                    font-weight: 600;
                                }
                                .faq-item p {
                                    margin: 0;
                                }
                                /* Support info styles */
                                .support-info {
                                    padding: 10px 0;
                                }
                                /* Grid utilities */
                                .col-lg-4, .col-lg-6, .col-lg-8, .col-md-6, .col-md-12, .col-sm-4 {
                                    position: relative;
                                    width: 100%;
                                    padding-right: 15px;
                                    padding-left: 15px;
                                }
                                @media (min-width: 576px) {
                                    .col-sm-4 { flex: 0 0 33.333333%; max-width: 33.333333%; }
                                }
                                @media (min-width: 768px) {
                                    .col-md-6 { flex: 0 0 50%; max-width: 50%; }
                                    .col-md-12 { flex: 0 0 100%; max-width: 100%; }
                                }
                                @media (min-width: 992px) {
                                    .col-lg-4 { flex: 0 0 33.333333%; max-width: 33.333333%; }
                                    .col-lg-6 { flex: 0 0 50%; max-width: 50%; }
                                    .col-lg-8 { flex: 0 0 66.666667%; max-width: 66.666667%; }
                                }
                                /* Navigation link classes for Link component */
                                .nav-link-purple {
                                    display: block;
                                    padding: 1rem;
                                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                    color: white !important;
                                    border-radius: 8px;
                                    font-weight: 600;
                                    text-align: center;
                                    text-decoration: none;
                                }
                                .nav-link-purple:hover {
                                    opacity: 0.9;
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
                                }
                                .nav-link-green {
                                    display: block;
                                    padding: 1rem;
                                    background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
                                    color: white !important;
                                    border-radius: 8px;
                                    font-weight: 600;
                                    text-align: center;
                                    text-decoration: none;
                                }
                                .nav-link-green:hover {
                                    opacity: 0.9;
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(17, 153, 142, 0.4);
                                }
                                .nav-link-orange {
                                    display: block;
                                    padding: 1rem;
                                    background: linear-gradient(135deg, #ee0979 0%, #ff6a00 100%);
                                    color: white !important;
                                    border-radius: 8px;
                                    font-weight: 600;
                                    text-align: center;
                                    text-decoration: none;
                                }
                                .nav-link-orange:hover {
                                    opacity: 0.9;
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(238, 9, 121, 0.4);
                                }
                                .nav-link-pink {
                                    display: block;
                                    padding: 1rem;
                                    background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
                                    color: white !important;
                                    border-radius: 8px;
                                    font-weight: 600;
                                    text-align: center;
                                    text-decoration: none;
                                }
                                .nav-link-pink:hover {
                                    opacity: 0.9;
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(240, 147, 251, 0.4);
                                }
                                .nav-link-gray {
                                    display: block;
                                    padding: 1rem;
                                    background: linear-gradient(135deg, #536976 0%, #292E49 100%);
                                    color: white !important;
                                    border-radius: 8px;
                                    font-weight: 600;
                                    text-align: center;
                                    text-decoration: none;
                                }
                                .nav-link-gray:hover {
                                    opacity: 0.9;
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(83, 105, 118, 0.4);
                                }
                                .nav-link-blue {
                                    display: block;
                                    padding: 1rem;
                                    background: linear-gradient(135deg, #00c6ff 0%, #0072ff 100%);
                                    color: white !important;
                                    border-radius: 8px;
                                    font-weight: 600;
                                    text-align: center;
                                    text-decoration: none;
                                }
                                .nav-link-blue:hover {
                                    opacity: 0.9;
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(0, 198, 255, 0.4);
                                }
                                .btn-register {
                                    display: inline-block;
                                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                    color: white;
                                    padding: 0.875rem 2rem;
                                    border-radius: 8px;
                                    font-weight: 600;
                                    text-decoration: none;
                                }
                                .btn-register:hover {
                                    opacity: 0.9;
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
                                }
                                .btn-login {
                                    display: inline-block;
                                    background: transparent;
                                    color: #667eea;
                                    padding: 0.875rem 2rem;
                                    border: 2px solid #667eea;
                                    border-radius: 8px;
                                    font-weight: 600;
                                    text-decoration: none;
                                }
                                .btn-login:hover {
                                    background: #667eea;
                                    color: white !important;
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
