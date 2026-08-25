use perseus::prelude::*;
use sycamore::prelude::*;

/// Navigation component props
#[derive(Clone)]
pub struct NavProps {
    pub current_page: &'static str,
}

/// Simple navbar view function (works on both engine and client)
pub fn navbar(props: NavProps) -> View {
    view! {
        nav(class = "navbar", style = "background: white; box-shadow: 0 2px 10px rgba(0,0,0,0.08); padding: 0.75rem 2rem; display: flex; justify-content: space-between; align-items: center;") {
            div(class = "navbar-brand", style = "display: flex; align-items: center; gap: 10px;") {
                Link(to = "/", style = "font-size: 1.25rem; font-weight: 700; color: #333; text-decoration: none;") {
                    span(style = "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent;") { "Perseus" }
                    span(style = "color: #666; font-weight: 400;") { "Membership" }
                }
            }
            div(class = "navbar-nav", style = "display: flex; gap: 0.5rem; align-items: center;") {
                Link(to = "/dashboard", class = if props.current_page == "dashboard" { "nav-link active" } else { "nav-link" }) { "Dashboard" }
                Link(to = "/posts", class = if props.current_page == "posts" { "nav-link active" } else { "nav-link" }) { "Posts" }
                Link(to = "/profile", class = if props.current_page == "profile" { "nav-link active" } else { "nav-link" }) { "Profile" }
                Link(to = "/preferences", class = if props.current_page == "preferences" { "nav-link active" } else { "nav-link" }) { "Preferences" }
                Link(to = "/contact", class = if props.current_page == "contact" { "nav-link active" } else { "nav-link" }) { "Contact" }
                Link(to = "/login", class = if props.current_page == "login" { "nav-link active" } else { "nav-link" }) { "Login" }
                Link(to = "/register", class = if props.current_page == "register" { "nav-link active" } else { "nav-link" }) { "Register" }
            }
        }
    }
}

/* Add this to your CSS:
.navbar {
    position: sticky;
    top: 0;
    z-index: 1000;
}
.nav-link {
    display: inline-block;
    padding: 0.5rem 1rem;
    color: #495057 !important;
    text-decoration: none !important;
    border-radius: 4px;
    font-size: 0.875rem;
    font-weight: 500;
    transition: all 0.15s ease;
}
.nav-link:hover {
    color: #667eea !important;
    background: rgba(102, 126, 234, 0.1);
}
.nav-link.active {
    color: #667eea !important;
    background: rgba(102, 126, 234, 0.1);
}
.badge {
    display: inline-block;
    padding: 0.25em 0.5em;
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
    text-align: center;
    white-space: nowrap;
    vertical-align: baseline;
    border-radius: 0.25rem;
}
.badge-info {
    color: #fff;
    background-color: #17a2b8;
}
*/
