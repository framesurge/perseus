use crate::template::Entity;
use crate::{path::PathWithoutLocale, Html};

/// Information about a route, which, combined with error pages and a
/// client-side translations manager, allows the initialization of the app shell
/// and the rendering of a page.
#[derive(Clone)]
pub struct RouteInfo<G: Html> {
    /// The actual path of the route. This does *not* include the locale!
    pub path: PathWithoutLocale,
    /// The template that will be used. The app shell will derive props and a
    /// translator to pass to the template function.
    pub entity: Entity<G>,
    /// Whether or not the matched page was incrementally-generated at runtime
    /// (if it has been yet). If this is `true`, the server will
    /// use a mutable store rather than an immutable one. See the book for more
    /// details.
    pub was_incremental_match: bool,
    /// The locale for the template to be rendered in.
    pub locale: String,
}

/// The possible outcomes of matching a route in an app.
#[derive(Clone)]
pub enum RouteVerdict<G: Html> {
    /// The given route was found, and route information is attached.
    Found(RouteInfo<G>),
    /// The given route was not found, and a `404 Not Found` page should be
    /// shown. In apps using i18n, an invalid page without a locale will
    /// first be redirected, before being later resolved as 404. Hence,
    /// we can always provide a locale here, allowing the error view to be
    /// appropriately translated. (I.e. there will never be a non-localized
    /// 404 page in Perseus.)
    NotFound {
        /// The active locale.
        locale: String,
    },
    /// The given route maps to the locale detector, which will redirect the
    /// user to the attached path (in the appropriate locale).
    ///
    /// The attached path will have the appropriate locale prepended during the
    /// detection process.
    LocaleDetection(PathWithoutLocale),
}
