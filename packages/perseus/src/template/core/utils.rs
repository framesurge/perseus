/// An internal subset of `RouteInfo` that stores the details needed for
/// preloading.
///
/// This exists on the engine-side for type convenience, but only has fields
/// on the browser-side.
pub(crate) struct PreloadInfo {
    #[cfg(any(client, doc))]
    pub(crate) locale: String,
    #[cfg(any(client, doc))]
    pub(crate) was_incremental_match: bool,
}
