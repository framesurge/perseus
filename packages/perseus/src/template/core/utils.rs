/// An internal subset of `RouteInfo` that stores the details needed for
/// preloading.
///
/// This exists on the engine-side for type convenience, but only has fields
/// on the browser-side.
pub(crate) struct PreloadInfo {
    #[cfg(target_arch = "wasm32")]
    pub(crate) locale: String,
    #[cfg(target_arch = "wasm32")]
    pub(crate) was_incremental_match: bool,
}
