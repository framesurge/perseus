/// A widget dependency that has been resolved to the details needed to actually render it.
/// These are held through interior mutability by templates.
#[derive(Clone, Debug)]
pub(crate) struct ResolvedWidgetDependency {
    /// The full path of the widget.
    pub(crate) path: String,
    /// The name of the capsule that created the widget.
    pub(crate) capsule_name: String,
    /// The locale for which this resolution was performed. (Hypothetically, in some
    /// custom builds, a page may only exist for some locales; Perseus doesn't actually
    /// prohibit this behavior.)
    pub(crate) locale: String,
    /// Whether or not the widget was an incremental match from the capsule, which the
    /// server will need to know when rendering.
    pub(crate) was_incremental_match: bool,
}

/// An internal subset of `RouteInfo` that stores the details needed for preloading.
pub(crate) struct PreloadInfo {
    pub(crate) locale: String,
    pub(crate) was_incremental_match: bool,
}
