use super::rx_state::AnyFreeze;
#[cfg(not(target_arch = "wasm32"))] // To suppress warnings
use crate::errors::*;
use crate::make_async_trait;
use crate::template::RenderFnResult;
use crate::utils::AsyncFnReturn;
use futures::Future;
use std::cell::RefCell;
use std::rc::Rc;

make_async_trait!(GlobalStateCreatorFnType, RenderFnResult<String>);
/// The type of functions that generate global state. These will generate a
/// `String` for their custom global state type.
#[cfg(not(target_arch = "wasm32"))]
pub type GlobalStateCreatorFn = Box<dyn GlobalStateCreatorFnType + Send + Sync>;

/// A creator for global state. This stores user-provided functions that will be
/// invoked to generate global state on the client and the server.
///
/// The primary purpose of this is to allow the generation of top-level app
/// state on the server and the client. Notably, this can also be interacted
/// with by plugins.
#[derive(Default)]
pub struct GlobalStateCreator {
    /// The function that creates state at build-time. This is roughly
    /// equivalent to the *build state* strategy for templates.
    #[cfg(not(target_arch = "wasm32"))]
    build: Option<GlobalStateCreatorFn>,
}
impl std::fmt::Debug for GlobalStateCreator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalStateCreator").finish()
    }
}
impl GlobalStateCreator {
    /// Creates a new instance of this `struct`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds a function to generate global state at build-time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_state_fn(
        mut self,
        val: impl GlobalStateCreatorFnType + Send + Sync + 'static,
    ) -> Self {
        self.build = Some(Box::new(val));
        self
    }
    /// Adds a function to generate global state at build-time.
    #[cfg(target_arch = "wasm32")]
    pub fn build_state_fn(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Gets the global state at build-time. If no function was registered to
    /// this, we'll return `None`.
    #[cfg(not(target_arch = "wasm32"))]
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
