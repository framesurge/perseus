use std::collections::HashMap;

use crate::template::Entity;
use crate::{path::PathWithoutLocale, Html};

/// Information about a route, which, combined with error pages and a
/// client-side translations manager, allows the initialization of the app shell
/// and the rendering of a page.
#[derive(Clone, Debug)]
pub struct FullRouteInfo<'a, G: Html> {
    /// The actual path of the route. This does *not* include the locale!
    pub path: PathWithoutLocale,
    /// The template that will be used. The app shell will derive props and a
    /// translator to pass to the template function.
    pub entity: &'a Entity<G>,
    /// Whether or not the matched page was incrementally-generated at runtime
    /// (if it has been yet). If this is `true`, the server will
    /// use a mutable store rather than an immutable one. See the book for more
    /// details.
    pub was_incremental_match: bool,
    /// The locale for the template to be rendered in.
    pub locale: String,
}

/// The possible outcomes of matching a route in an app.
#[derive(Clone, Debug)]
pub enum FullRouteVerdict<'a, G: Html> {
    /// The given route was found, and route information is attached.
    Found(FullRouteInfo<'a, G>),
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

/// Information about a route, which, combined with error pages and a
/// client-side translations manager, allows the initialization of the app shell
/// and the rendering of a page.
///
/// Unlike [`FullRouteInfo`], this does not store the actual template being
/// used, instead it only stores its name, making it much easier to store.
#[derive(Clone, Debug)]
pub struct RouteInfo {
    /// The actual path of the route. This does *not* include the locale!
    pub path: PathWithoutLocale,
    /// The name of the template that should be used.
    pub entity_name: String,
    /// Whether or not the matched page was incrementally-generated at runtime
    /// (if it has been yet). If this is `true`, the server will
    /// use a mutable store rather than an immutable one. See the book for more
    /// details.
    pub was_incremental_match: bool,
    /// The locale for the template to be rendered in.
    pub locale: String,
}
impl RouteInfo {
    /// Converts this [`RouteInfo`] into a [`FullRouteInfo`].
    ///
    /// # Panics
    /// This will panic if the entity name held by `Self` is not in the given
    /// map, which is only a concern if you `Self` didn't come from
    /// `match_route`.
    pub(crate) fn into_full<'a, G: Html>(
        self,
        entities: &'a HashMap<String, Entity<G>>,
    ) -> FullRouteInfo<'a, G> {
        let entity = entities.get(&self.entity_name).expect("conversion to full route info failed, given entities did not contain given entity name");
        FullRouteInfo {
            path: self.path,
            entity,
            was_incremental_match: self.was_incremental_match,
            locale: self.locale,
        }
    }
}

/// The possible outcomes of matching a route in an app.
///
/// Unlike [`FullRouteVerdict`], this does not store the actual template being
/// used, instead it only stores its name, making it much easier to store.
#[derive(Clone, Debug)]
pub enum RouteVerdict {
    /// The given route was found, and route information is attached.
    Found(RouteInfo),
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
impl RouteVerdict {
    /// Converts this [`RouteVerdict`] into a [`FullRouteVerdict`].
    ///
    /// # Panics
    /// This will panic if the entity name held by `Self` is not in the given
    /// map, which is only a concern if you `Self` didn't come from
    /// `match_route` (this only applies when `Self` is `Self::Found(..)`).
    pub(crate) fn into_full<'a, G: Html>(
        self,
        entities: &'a HashMap<String, Entity<G>>,
    ) -> FullRouteVerdict<'a, G> {
        match self {
            Self::Found(info) => FullRouteVerdict::Found(info.into_full(entities)),
            Self::NotFound { locale } => FullRouteVerdict::NotFound { locale },
            Self::LocaleDetection(dest) => FullRouteVerdict::LocaleDetection(dest),
        }
    }
}
