use super::{TemplateState, TemplateStateWithType};
use serde::{de::DeserializeOwned, Serialize};

/// The output of the build seed system, which should be generated by a user
/// function for each template.
#[derive(Debug)]
pub struct BuildPaths {
    /// The paths to render underneath this template, without the template name
    /// or leading forward slashes.
    pub paths: Vec<String>,
    /// Any additional state, of an arbitrary type, to be passed to all future
    /// state generation. This can be used to avoid unnecessary duplicate
    /// filesystem reads, or the like.
    ///
    /// The exact type information from this is deliberately discarded.
    pub extra: TemplateState,
}

/// The information any function that generates state will be provided.
///
/// This must be able to be shared safely between threads.
#[derive(Clone, Debug)]
pub struct StateGeneratorInfo<B: Serialize + DeserializeOwned + Send + Sync> {
    /// The path it is generating for, not including the template name or
    /// locale.
    ///
    /// **Warning:** previous versions of Perseus used to prefix this with the
    /// template name, and this is no longer done, for convenience of
    /// handling.
    pub path: String,
    /// The locale it is generating for.
    pub locale: String,
    /// Any extra data from the template's build seed.
    pub(crate) extra: TemplateStateWithType<B>,
}
impl<B: Serialize + DeserializeOwned + Send + Sync + 'static> StateGeneratorInfo<B> {
    /// Transform the underlying [`TemplateStateWithType`] into one with a
    /// different type. Once this is done, `.to_concrete()` can be used to
    /// get this type out of the container.
    #[cfg(engine)] // Just to silence clippy (if you need to remove this, do)
    pub(crate) fn change_type<U: Serialize + DeserializeOwned + Send + Sync>(
        self,
    ) -> StateGeneratorInfo<U> {
        StateGeneratorInfo {
            path: self.path,
            locale: self.locale,
            extra: self.extra.change_type(),
        }
    }
    /// Get the extra build state as an owned type.
    ///
    /// # Panics
    /// Hypothetically, if there were a failure in the Perseus core such that
    /// your extra build state ended up being malformed, this would panic.
    /// However, this should never happen, as there are multiplr layers of
    /// checks before this that should catch such an event. If this panics,
    /// and if keeps panicking after `perseus clean`, please report it as a
    /// bug (assuming all your types are correct).
    pub fn get_extra(&self) -> B {
        match B::deserialize(&self.extra.state) {
            Ok(extra) => extra,
            // This should never happen...
            Err(err) => panic!(
                "unrecoverable extra build state extraction failure: {:#?}",
                err
            ),
        }
    }
}
