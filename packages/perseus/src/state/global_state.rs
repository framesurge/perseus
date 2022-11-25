use super::rx_state::AnyFreeze;
use super::{Freeze, MakeRx, MakeUnrx};
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

/// A representation of the global state in an app. This will be initialized as
/// a string of whatever was given by the server, until a page requests it and
/// deserializes it properly (since we can't know the type until that happens).
#[derive(Clone)]
pub struct GlobalState(pub Rc<RefCell<GlobalStateType>>);
impl GlobalState {
    /// A convenience function for creating a new global state from an
    /// underlying type of global state.
    pub(crate) fn new(ty: GlobalStateType) -> Self {
        Self(Rc::new(RefCell::new(ty)))
    }
}

/// The backend for the different types of global state.
pub enum GlobalStateType {
    /// The global state has been deserialized and loaded, and is ready for use.
    Loaded(Box<dyn AnyFreeze>),
    /// The global state is in string form from the server.
    Server(String),
    /// There was no global state provided by the server.
    None,
}
impl GlobalStateType {
    /// Parses the global state into the given reactive type if possible. If the
    /// state from the server hasn't been parsed yet, this will return
    /// `None`.
    ///
    /// In other words, this will only return something if the global state has
    /// already been requested and loaded.
    pub fn parse_active<R>(&self) -> Option<<R::Unrx as MakeRx>::Rx>
    where
        R: Clone + AnyFreeze + MakeUnrx,
        // We need this so that the compiler understands that the reactive version of the
        // unreactive version of `R` has the same properties as `R` itself
        <<R as MakeUnrx>::Unrx as MakeRx>::Rx: Clone + AnyFreeze + MakeUnrx,
    {
        match &self {
            // If there's an issue deserializing to this type, we'll fall back to the server
            Self::Loaded(any) => any
                .as_any()
                .downcast_ref::<<R::Unrx as MakeRx>::Rx>()
                .cloned(),
            Self::Server(_) => None,
            Self::None => None,
        }
    }
}
impl Freeze for GlobalStateType {
    fn freeze(&self) -> String {
        match &self {
            Self::Loaded(state) => state.freeze(),
            // There's no point in serializing state that was sent from the server, since we can
            // easily get it again later (it definitionally hasn't changed)
            Self::Server(_) => "Server".to_string(),
            Self::None => "None".to_string(),
        }
    }
}
impl std::fmt::Debug for GlobalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalState").finish()
    }
}

// /// A representation of global state parsed into a specific type.
// pub enum ParsedGlobalState<R> {
//     /// The global state has been deserialized and loaded, and is ready for
// use.     Loaded(R),
//     /// We couldn't parse to the desired reactive type.
//     ParseError,
//     /// The global state is in string form from the server.
//     Server(String),
//     /// There was no global state provided by the server.
//     None,
// }
