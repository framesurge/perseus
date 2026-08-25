//! A client-side navigation link component for Perseus.
//!
//! This component renders an anchor tag (`<a>`) that uses client-side navigation
//! via `navigate()` while preserving SEO-friendly `href` attributes.
//!
//! # Why is this needed?
//!
//! The sycamore-router intercepts clicks on anchor tags to enable client-side
//! navigation. However, this interception only works for anchor tags that exist
//! in the initial static view - it does **not** work for anchor tags created
//! inside dynamic views (closures that return views).
//!
//! Perseus renders page content inside a dynamic view to enable reactive updates
//! during navigation. This means anchor tags inside page templates don't get
//! the router's click interception, causing full page reloads instead of
//! client-side navigation.
//!
//! This `Link` component solves the problem by explicitly calling `navigate()`
//! on click while preserving the `href` attribute for:
//! - SEO (search engine crawlers see proper links)
//! - Accessibility (right-click "open in new tab", etc.)
//! - Graceful degradation (works without JavaScript)

use std::fmt;
use sycamore::prelude::*;

#[cfg(client)]
use sycamore_router::navigate;

/// Props for the [`Link`] component.
#[derive(Props)]
pub struct LinkProps {
    /// The destination URL for the link.
    /// This should be an absolute path (e.g., "/about") or use the `link!` macro
    /// for internationalized paths.
    #[prop(setter(into))]
    pub to: String,
    /// Optional ID attribute for the anchor element.
    #[prop(default, setter(into))]
    pub id: Option<String>,
    /// Optional CSS class(es) for the anchor element.
    #[prop(default, setter(into))]
    pub class: Option<String>,
    /// Optional inline CSS style(s) for the anchor element.
    #[prop(default, setter(into))]
    pub style: Option<String>,
    /// The content to display inside the link.
    pub children: Children,
}

impl fmt::Debug for LinkProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LinkProps")
            .field("to", &self.to)
            .field("id", &self.id)
            .field("class", &self.class)
            .field("style", &self.style)
            .field("children", &"<Children>")
            .finish()
    }
}

/// A client-side navigation link component.
///
/// This renders an `<a>` tag that performs client-side navigation using Perseus's
/// router, while maintaining proper `href` attributes for SEO and accessibility.
///
/// # Example
///
/// ```rust,ignore
/// use perseus::prelude::*;
/// use sycamore::prelude::*;
///
/// fn my_page() -> View {
///     view! {
///         // Simple link
///         Link(to = "/about") { "About" }
///
///         // Link with i18n support
///         Link(to = link!("/about")) { "About" }
///
///         // Link with ID and class
///         Link(to = "/contact", id = "contact-link", class = "nav-link") {
///             "Contact Us"
///         }
///     }
/// }
/// ```
#[component]
pub fn Link(props: LinkProps) -> View {
    let LinkProps {
        to,
        id,
        class,
        style,
        children,
    } = props;

    let children = children.call();
    let href = to.clone();

    // On the client, we intercept clicks and use navigate()
    // On the server/engine, we just render a normal anchor tag
    #[cfg(client)]
    let on_click = {
        let to = to.clone();
        move |ev: web_sys::MouseEvent| {
            // Don't intercept if modifier keys are pressed (open in new tab, etc.)
            if ev.meta_key() || ev.ctrl_key() || ev.shift_key() || ev.alt_key() {
                return;
            }
            // Don't intercept middle-click (usually opens in new tab)
            if ev.button() != 0 {
                return;
            }
            ev.prevent_default();
            navigate(&to);
        }
    };

    #[cfg(client)]
    {
        match (id, class, style) {
            (Some(id), Some(class), Some(style)) => view! {
                a(href = href, id = id, class = class, style = style, on:click = on_click) { (children) }
            },
            (Some(id), Some(class), None) => view! {
                a(href = href, id = id, class = class, on:click = on_click) { (children) }
            },
            (Some(id), None, Some(style)) => view! {
                a(href = href, id = id, style = style, on:click = on_click) { (children) }
            },
            (Some(id), None, None) => view! {
                a(href = href, id = id, on:click = on_click) { (children) }
            },
            (None, Some(class), Some(style)) => view! {
                a(href = href, class = class, style = style, on:click = on_click) { (children) }
            },
            (None, Some(class), None) => view! {
                a(href = href, class = class, on:click = on_click) { (children) }
            },
            (None, None, Some(style)) => view! {
                a(href = href, style = style, on:click = on_click) { (children) }
            },
            (None, None, None) => view! {
                a(href = href, on:click = on_click) { (children) }
            },
        }
    }

    #[cfg(not(client))]
    {
        match (id, class, style) {
            (Some(id), Some(class), Some(style)) => view! {
                a(href = href, id = id, class = class, style = style) { (children) }
            },
            (Some(id), Some(class), None) => view! {
                a(href = href, id = id, class = class) { (children) }
            },
            (Some(id), None, Some(style)) => view! {
                a(href = href, id = id, style = style) { (children) }
            },
            (Some(id), None, None) => view! {
                a(href = href, id = id) { (children) }
            },
            (None, Some(class), Some(style)) => view! {
                a(href = href, class = class, style = style) { (children) }
            },
            (None, Some(class), None) => view! {
                a(href = href, class = class) { (children) }
            },
            (None, None, Some(style)) => view! {
                a(href = href, style = style) { (children) }
            },
            (None, None, None) => view! {
                a(href = href) { (children) }
            },
        }
    }
}
