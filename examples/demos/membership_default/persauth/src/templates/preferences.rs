use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::types::{ApiResponse, SessionData};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: i32,
    pub email: String,
    pub roles: Vec<String>,
    pub otp_confirmed: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePreferencesRequest {
    pub session_id: i32,
    pub session_verifier: String,
    pub theme: Option<String>,
    pub email_notifications: Option<bool>,
}

// Simple view with basic HTML - no complex reactivity for initial implementation
fn preferences_page() -> View {
    view! {
        // Simple navigation header
        nav(class = "navbar", style = "background: white; box-shadow: 0 2px 10px rgba(0,0,0,0.08); padding: 0.75rem 2rem; display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;") {
            div(class = "navbar-brand", style = "display: flex; align-items: center; gap: 10px;") {
                Link(to = "/", style = "font-size: 1.25rem; font-weight: 700; color: #333; text-decoration: none;") {
                    span(style = "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent;") { "Perseus" }
                    span(style = "color: #666; font-weight: 400;") { "Membership" }
                }
            }
            div(class = "navbar-nav", style = "display: flex; gap: 0.5rem; align-items: center;") {
                Link(to = "/dashboard", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Dashboard" }
                Link(to = "/posts", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Posts" }
                Link(to = "/profile", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Profile" }
                Link(to = "/preferences", style = "display: inline-block; padding: 0.5rem 1rem; color: #667eea; background: rgba(102,126,234,0.1); text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Preferences" }
                Link(to = "/contact", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Contact" }
                Link(to = "/logout", style = "display: inline-block; padding: 0.5rem 1rem; color: #495057; text-decoration: none; border-radius: 4px; font-size: 0.875rem; font-weight: 500;") { "Logout" }
            }
        }

        div(class = "page-body") {
            div(class = "container-fluid") {
                div(class = "card") {
                    div(class = "card-header") {
                        h5 { "User Preferences" }
                        span(class = "d-block m-t-5", style = "color: #6c757d; font-size: 0.875rem;") { "Customize your account settings" }
                    }
                    div(class = "card-body") {
                        div(class = "alert alert-info") {
                            strong { "Loading..." }
                            " Your preferences are being loaded. Please wait or "
                            Link(to = "/login", style = "color: inherit; text-decoration: underline;") { "log in" }
                            " if you are not authenticated."
                        }
                    }
                }

                div(class = "card") {
                    div(class = "card-header") {
                        h5 { "Account Information" }
                    }
                    div(class = "card-body") {
                        p(class = "text-muted") { "Sign in to view and manage your account settings, including theme preferences and notification options." }
                        Link(to = "/login", class = "btn btn-primary") { "Login to Your Account" }
                    }
                }

                div(class = "card") {
                    div(class = "card-header") {
                        h5 { "Security Settings" }
                    }
                    div(class = "card-body") {
                        div(class = "row") {
                            div(class = "col-md-6") {
                                div(class = "form-group") {
                                    label { "Two-Factor Authentication" }
                                    p(class = "text-muted", style = "font-size: 0.875rem;") { "Add an extra layer of security to your account" }
                                    Link(to = "/profile", class = "btn btn-secondary btn-sm") { "Configure in Profile" }
                                }
                            }
                            div(class = "col-md-6") {
                                div(class = "form-group") {
                                    label { "Password" }
                                    p(class = "text-muted", style = "font-size: 0.875rem;") { "Change your password regularly to keep your account secure" }
                                    Link(to = "/profile", class = "btn btn-secondary btn-sm") { "Change Password" }
                                }
                            }
                        }
                    }
                }

                div(class = "card", style = "border-color: #dc3545;") {
                    div(class = "card-header", style = "background: #fff5f5; color: #dc3545;") {
                        h5 { "Danger Zone" }
                    }
                    div(class = "card-body") {
                        div(class = "form-group") {
                            label { "Delete Account" }
                            p(class = "text-muted", style = "font-size: 0.875rem;") { "Permanently delete your account and all associated data. This action cannot be undone." }
                            button(class = "btn btn-danger btn-sm") { "Delete Account" }
                        }
                    }
                }
            }
        }

        // Client-side initialization script
        script {
            r#"
            (function() {
                // Simple preferences page initialization
                console.log('Preferences page loaded');
            })();
            "#
        }
    }
}

pub fn get_template() -> Template {
    Template::build("preferences").view(preferences_page).build()
}
