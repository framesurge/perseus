use super::rx_state::AnyFreeze;
use super::{Freeze, MakeRx, MakeUnrx};
#[cfg(not(target_arch = "wasm32"))] // To suppress warnings
use crate::errors::*;
use crate::stores::ImmutableStore;
use crate::template::{RenderFnResult, TemplateState};
use crate::utils::AsyncFnReturn;
use crate::{make_async_trait, RenderFnResultWithCause, Request};
use futures::Future;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    GlobalStateBuildFnType,
    RenderFnResult<TemplateState>,
    locale: String
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    GlobalStateRequestFnType,
    RenderFnResultWithCause<TemplateState>,
    locale: String,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    GlobalStateAmalgamationFnType,
    RenderFnResultWithCause<TemplateState>,
    locale: String,
    build_state: TemplateState,
    request_state: TemplateState
);

#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    GlobalStateBuildUserFnType<S: Serialize + DeserializeOwned + MakeRx>,
    RenderFnResult<S>,
    locale: String
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    GlobalStateRequestUserFnType<S: Serialize + DeserializeOwned + MakeRx>,
    RenderFnResultWithCause<S>,
    locale: String,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    GlobalStateAmalgamationUserFnType<S: Serialize + DeserializeOwned + MakeRx>,
    RenderFnResultWithCause<S>,
    locale: String,
    build_state: S,
    request_state: S
);

/// The type of functions that generate global state at build-time.
#[cfg(not(target_arch = "wasm32"))]
pub type GlobalStateBuildFn = Box<dyn GlobalStateBuildFnType + Send + Sync>;
/// The type of functions that generate global state at build-time.
#[cfg(not(target_arch = "wasm32"))]
pub type GlobalStateRequestFn = Box<dyn GlobalStateRequestFnType + Send + Sync>;
/// The type of functions that generate global state at build-time.
#[cfg(not(target_arch = "wasm32"))]
pub type GlobalStateAmalgamationFn = Box<dyn GlobalStateAmalgamationFnType + Send + Sync>;

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
    build: Option<GlobalStateBuildFn>,
    /// The function that creates state at request-time. This is roughly
    /// equivalent to the *request state* strategy for templates.
    #[cfg(not(target_arch = "wasm32"))]
    request: Option<GlobalStateRequestFn>,
    /// The function that amalgamates state from build-time and request-time.
    /// This is roughly equivalent to the *state amalgamation* strategy for
    /// templates.
    #[cfg(not(target_arch = "wasm32"))]
    amalgamation: Option<GlobalStateAmalgamationFn>,
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
    pub fn build_state_fn<S>(
        mut self,
        val: impl GlobalStateBuildUserFnType<S> + Clone + Send + Sync + 'static,
    ) -> Self
    where
        S: Serialize + DeserializeOwned + MakeRx,
    {
        self.build = Some(Box::new(move |locale| {
            let val = val.clone();
            async move {
                let user_state = val.call(locale).await?;
                let template_state: TemplateState = user_state.into();
                Ok(template_state)
            }
        }));
        self
    }
    /// Adds a function to generate global state at build-time.
    #[cfg(target_arch = "wasm32")]
    pub fn build_state_fn(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Adds a function to generate global state at request-time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn request_state_fn<S>(
        mut self,
        val: impl GlobalStateRequestUserFnType<S> + Clone + Send + Sync + 'static,
    ) -> Self
    where
        S: Serialize + DeserializeOwned + MakeRx,
    {
        self.request = Some(Box::new(move |locale, req| {
            let val = val.clone();
            async move {
                let user_state = val.call(locale, req).await?;
                let template_state: TemplateState = user_state.into();
                Ok(template_state)
            }
        }));
        self
    }
    /// Adds a function to generate global state at request-time.
    #[cfg(target_arch = "wasm32")]
    pub fn request_state_fn(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Adds a function to amalgamate build-time and request-time global state.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn amalgamate_states_fn<S>(
        mut self,
        val: impl GlobalStateAmalgamationUserFnType<S> + Clone + Send + Sync + 'static,
    ) -> Self
    where
        S: Serialize + DeserializeOwned + MakeRx + Send + Sync + 'static,
    {
        self.amalgamation = Some(Box::new(
            move |locale, build_state: TemplateState, request_state: TemplateState| {
                let val = val.clone();
                async move {
                    // Amalgamation logic will only be called if both states are indeed defined
                    let typed_build_state = build_state.change_type::<S>();
                    let user_build_state = match typed_build_state.to_concrete() {
                        Ok(state) => state,
                        Err(err) => panic!(
                            "unrecoverable error in state amalgamation parameter derivation: {:#?}",
                            err
                        ),
                    };
                    let typed_request_state = request_state.change_type::<S>();
                    let user_request_state = match typed_request_state.to_concrete() {
                        Ok(state) => state,
                        Err(err) => panic!(
                            "unrecoverable error in state amalgamation parameter derivation: {:#?}",
                            err
                        ),
                    };
                    let user_state = val
                        .call(locale, user_build_state, user_request_state)
                        .await?;
                    let template_state: TemplateState = user_state.into();
                    Ok(template_state)
                }
            },
        ));
        self
    }
    /// Adds a function to amalgamate build-time and request-time global state.
    #[cfg(target_arch = "wasm32")]
    pub fn amalgamate_states_fn(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Gets the global state at build-time.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_build_state(&self, locale: String) -> Result<TemplateState, ServerError> {
        if let Some(get_build_state) = &self.build {
            let res = get_build_state.call(locale).await;
            match res {
                Ok(res) => Ok(res),
                // Unlike template build state, there's no incremental generation here, so the
                // client can't have caused an error
                Err(err) => Err(ServerError::RenderFnFailed {
                    fn_name: "get_build_state".to_string(),
                    template_name: "GLOBAL_STATE".to_string(),
                    cause: ErrorCause::Server(None),
                    source: err,
                }),
            }
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: "GLOBAL_STATE".to_string(),
                feature_name: "build_state".to_string(),
            }
            .into())
        }
    }
    /// Gets the global state at request-time.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_request_state(
        &self,
        locale: String,
        req: Request,
    ) -> Result<TemplateState, ServerError> {
        if let Some(get_request_state) = &self.request {
            let res = get_request_state.call(locale, req).await;
            match res {
                Ok(res) => Ok(res),
                Err(GenericErrorWithCause { error, cause }) => Err(ServerError::RenderFnFailed {
                    fn_name: "get_request_state".to_string(),
                    template_name: "GLOBAL_STATE".to_string(),
                    cause,
                    source: error,
                }),
            }
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: "GLOBAL_STATE".to_string(),
                feature_name: "request_state".to_string(),
            }
            .into())
        }
    }
    /// Amalgamates global state that was generated at build-time with that
    /// generated at request-time, according to custom user-provided logic.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn amalgamate_states(
        &self,
        locale: String,
        build_state: TemplateState,
        request_state: TemplateState,
    ) -> Result<TemplateState, ServerError> {
        if let Some(amalgamate_states) = &self.amalgamation {
            let res = amalgamate_states
                .call(locale, build_state, request_state)
                .await;
            match res {
                Ok(res) => Ok(res),
                Err(GenericErrorWithCause { error, cause }) => Err(ServerError::RenderFnFailed {
                    fn_name: "amalgamate_states".to_string(),
                    template_name: "GLOBAL_STATE".to_string(),
                    cause,
                    source: error,
                }),
            }
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: "GLOBAL_STATE".to_string(),
                feature_name: "amalgamate_states".to_string(),
            }
            .into())
        }
    }

    /// Checks if this state needs to do anything on requests for it.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_request_state(&self) -> bool {
        self.request.is_some()
    }
    /// Checks if this state needs to do anything at build time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_build_state(&self) -> bool {
        self.build.is_some()
    }
    /// Checks if this state has custom logic to amalgamate build and
    /// request states if both are generated.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn can_amalgamate_states(&self) -> bool {
        self.amalgamation.is_some()
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
    Server(TemplateState),
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
            // easily get it again later (it can't possibly have been changed on the browser-side)
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

/// A utility function for getting the global state that has already been built
/// at build-time. If there was none built, then this will return an empty
/// [`TemplateState`] (hence, a `StoreError::NotFound` is impossible from this
/// function).
#[cfg(not(target_arch = "wasm32"))]
pub async fn get_built_global_state(
    immutable_store: &ImmutableStore,
) -> Result<TemplateState, ServerError> {
    let res = immutable_store.read("static/global_state.json").await;
    match res {
        Ok(state) => {
            let state = TemplateState::from_str(&state)
                .map_err(|err| ServerError::InvalidPageState { source: err })?;
            Ok(state)
        }
        Err(StoreError::NotFound { .. }) => Ok(TemplateState::empty()),
        Err(err) => Err(err.into()),
    }
}
