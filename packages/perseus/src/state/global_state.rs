use super::rx_state::AnyFreeze;
use crate::errors::*;
use crate::make_async_trait;
use crate::template::RenderFnResult;
use crate::utils::AsyncFnReturn;
use futures::Future;
use std::cell::RefCell;
use std::rc::Rc;

make_async_trait!(GlobalStateCreatorFnType, RenderFnResult<String>);
/// The type of functions that generate global state. These will generate a `String` for their custom global state type.
pub type GlobalStateCreatorFn = Rc<dyn GlobalStateCreatorFnType + Send + Sync>;

/// A creator for global state. This stores user-provided functions that will be invoked to generate global state on the client
/// and the server.
///
/// The primary purpose of this is to allow the generation of top-level app state on the server and the client. Notably,
/// this can also be interacted with by plugins.
#[derive(Default, Clone)]
pub struct GlobalStateCreator {
    /// The function that creates state at build-time. This is roughly equivalent to the *build state* strategy for templates.
    build: Option<GlobalStateCreatorFn>,
}
impl std::fmt::Debug for GlobalStateCreator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalStateCreator")
            .field(
                "build",
                &self.build.as_ref().map(|_| "GlobalStateCreatorFn"),
            )
            .finish()
    }
}
impl GlobalStateCreator {
    /// Creates a new instance of this `struct`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds a function to generate global state at build-time.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn build_state_fn(
        mut self,
        val: impl GlobalStateCreatorFnType + Send + Sync + 'static,
    ) -> Self {
        #[cfg(feature = "server-side")]
        {
            self.build = Some(Rc::new(val));
        }
        self
    }
    /// Gets the global state at build-time. If no function was registered to this, we'll return `None`.
    pub async fn get_build_state(&self) -> Result<Option<String>, GlobalStateError> {
        if let Some(get_server_state) = &self.build {
            let res = get_server_state.call().await;
            match res {
                Ok(res) => Ok(Some(res)),
                Err(err) => Err(GlobalStateError::BuildGenerationFailed { source: err }),
            }
        } else {
            Ok(None)
        }
    }
}

/// A representation of the global state in an app.
#[derive(Clone)]
pub struct GlobalState(pub Rc<RefCell<Box<dyn AnyFreeze>>>);
impl Default for GlobalState {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Box::new(Option::<()>::None))))
    }
}
impl std::fmt::Debug for GlobalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalState").finish()
    }
}
