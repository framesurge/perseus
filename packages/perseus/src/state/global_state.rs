use super::rx_state::AnyFreeze;
use super::TemplateState;
use super::{MakeRx, MakeUnrx};
#[cfg(engine)] // To suppress warnings
use crate::errors::*;
use crate::errors::{ClientError, ClientInvariantError};
#[cfg(engine)]
use crate::make_async_trait;
#[cfg(engine)]
use crate::template::{BlamedGeneratorResult, GeneratorResult};
#[cfg(engine)]
use crate::utils::AsyncFnReturn;
#[cfg(engine)]
use crate::Request;
#[cfg(engine)]
use futures::Future;
#[cfg(engine)]
use serde::de::DeserializeOwned;
#[cfg(any(client, doc))]
use serde::Deserialize;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(engine)]
make_async_trait!(
    GlobalStateBuildFnType,
    Result<TemplateState, ServerError>
);
#[cfg(engine)]
make_async_trait!(
    GlobalStateRequestFnType,
    Result<TemplateState, ServerError>,
    req: Request
);
#[cfg(engine)]
make_async_trait!(
    GlobalStateAmalgamationFnType,
    Result<TemplateState, ServerError>,
    build_state: TemplateState,
    request_state: TemplateState
);

#[cfg(engine)]
make_async_trait!(
    GlobalStateBuildUserFnType<
        S: Serialize + DeserializeOwned + MakeRx,
        V: Into<
        GeneratorResult<S>
        >,
    >,
    V
);
#[cfg(engine)]
make_async_trait!(
    GlobalStateRequestUserFnType< S: Serialize + DeserializeOwned + MakeRx, V: Into< BlamedGeneratorResult<S> > >,
    V,
    req: Request
);
#[cfg(engine)]
make_async_trait!(
    GlobalStateAmalgamationUserFnType< S: Serialize + DeserializeOwned + MakeRx, V: Into< BlamedGeneratorResult<S> > >,
    V,
    build_state: S,
    request_state: S
);

/// The type of functions that generate global state at build-time.
#[cfg(engine)]
pub type GlobalStateBuildFn = Box<dyn GlobalStateBuildFnType + Send + Sync>;
/// The type of functions that generate global state at build-time.
#[cfg(engine)]
pub type GlobalStateRequestFn = Box<dyn GlobalStateRequestFnType + Send + Sync>;
/// The type of functions that generate global state at build-time.
#[cfg(engine)]
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
    #[cfg(engine)]
    build: Option<GlobalStateBuildFn>,
    /// The function that creates state at request-time. This is roughly
    /// equivalent to the *request state* strategy for templates.
    #[cfg(engine)]
    request: Option<GlobalStateRequestFn>,
    /// The function that amalgamates state from build-time and request-time.
    /// This is roughly equivalent to the *state amalgamation* strategy for
    /// templates.
    #[cfg(engine)]
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
    #[cfg(engine)]
    pub fn build_state_fn<S, V>(
        mut self,
        val: impl GlobalStateBuildUserFnType<S, V> + Clone + Send + Sync + 'static,
    ) -> Self
    where
        S: Serialize + DeserializeOwned + MakeRx,
        V: Into<GeneratorResult<S>>,
    {
        self.build = Some(Box::new(move || {
            let val = val.clone();
            async move {
                let user_state = val
                    .call()
                    .await
                    .into()
                    .into_server_result("global_build_state", "GLOBAL_STATE".to_string())?;
                let template_state: TemplateState = user_state.into();
                Ok(template_state)
            }
        }));
        self
    }
    /// Adds a function to generate global state at build-time.
    #[cfg(any(client, doc))]
    pub fn build_state_fn(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Adds a function to generate global state at request-time.
    #[cfg(engine)]
    pub fn request_state_fn<S, V>(
        mut self,
        val: impl GlobalStateRequestUserFnType<S, V> + Clone + Send + Sync + 'static,
    ) -> Self
    where
        S: Serialize + DeserializeOwned + MakeRx,
        V: Into<BlamedGeneratorResult<S>>,
    {
        self.request = Some(Box::new(move |req| {
            let val = val.clone();
            async move {
                let user_state = val
                    .call(req)
                    .await
                    .into()
                    .into_server_result("global_request_state", "GLOBAL_STATE".to_string())?;
                let template_state: TemplateState = user_state.into();
                Ok(template_state)
            }
        }));
        self
    }
    /// Adds a function to generate global state at request-time.
    #[cfg(any(client, doc))]
    pub fn request_state_fn(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Adds a function to amalgamate build-time and request-time global state.
    #[cfg(engine)]
    pub fn amalgamate_states_fn<S, V>(
        mut self,
        val: impl GlobalStateAmalgamationUserFnType<S, V> + Clone + Send + Sync + 'static,
    ) -> Self
    where
        S: Serialize + DeserializeOwned + MakeRx + Send + Sync + 'static,
        V: Into<BlamedGeneratorResult<S>>,
    {
        self.amalgamation = Some(Box::new(
            move |build_state: TemplateState, request_state: TemplateState| {
                let val = val.clone();
                async move {
                    // Amalgamation logic will only be called if both states are indeed defined
                    let typed_build_state = build_state.change_type::<S>();
                    let user_build_state = match typed_build_state.into_concrete() {
                        Ok(state) => state,
                        Err(err) => panic!(
                            "unrecoverable error in state amalgamation parameter derivation: {:#?}",
                            err
                        ),
                    };
                    let typed_request_state = request_state.change_type::<S>();
                    let user_request_state = match typed_request_state.into_concrete() {
                        Ok(state) => state,
                        Err(err) => panic!(
                            "unrecoverable error in state amalgamation parameter derivation: {:#?}",
                            err
                        ),
                    };
                    let user_state = val
                        .call(user_build_state, user_request_state)
                        .await
                        .into()
                        .into_server_result(
                            "global_amalgamate_states",
                            "GLOBAL_STATE".to_string(),
                        )?;
                    let template_state: TemplateState = user_state.into();
                    Ok(template_state)
                }
            },
        ));
        self
    }
    /// Adds a function to amalgamate build-time and request-time global state.
    #[cfg(any(client, doc))]
    pub fn amalgamate_states_fn(self, _val: impl Fn() + 'static) -> Self {
        self
    }

    /// Gets the global state at build-time.
    #[cfg(engine)]
    pub async fn get_build_state(&self) -> Result<TemplateState, ServerError> {
        if let Some(get_build_state) = &self.build {
            get_build_state.call().await
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: "GLOBAL_STATE".to_string(),
                feature_name: "build_state".to_string(),
            }
            .into())
        }
    }
    /// Gets the global state at request-time.
    #[cfg(engine)]
    pub async fn get_request_state(&self, req: Request) -> Result<TemplateState, ServerError> {
        if let Some(get_request_state) = &self.request {
            get_request_state.call(req).await
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
    #[cfg(engine)]
    pub async fn amalgamate_states(
        &self,
        build_state: TemplateState,
        request_state: TemplateState,
    ) -> Result<TemplateState, ServerError> {
        if let Some(amalgamate_states) = &self.amalgamation {
            amalgamate_states.call(build_state, request_state).await
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: "GLOBAL_STATE".to_string(),
                feature_name: "amalgamate_states".to_string(),
            }
            .into())
        }
    }

    /// Checks if this state needs to do anything on requests for it.
    #[cfg(engine)]
    pub fn uses_request_state(&self) -> bool {
        self.request.is_some()
    }
    /// Checks if this state needs to do anything at build time.
    #[cfg(engine)]
    pub fn uses_build_state(&self) -> bool {
        self.build.is_some()
    }
    /// Checks if this state has custom logic to amalgamate build and
    /// request states if both are generated.
    #[cfg(engine)]
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
#[derive(Debug)]
pub enum GlobalStateType {
    /// The global state has been deserialized and loaded, and is ready for use.
    Loaded(Box<dyn AnyFreeze>),
    /// The global state is in string form from the server.
    Server(TemplateState),
    /// The global state provided by the server was empty, indicating that this
    /// app does not use global state.
    None,
}
impl GlobalStateType {
    /// Parses the global state into the given reactive type if possible. If the
    /// state from the server hasn't been parsed yet, this will return
    /// `None`. This will return an error if a type mismatch occurred.
    ///
    /// In other words, this will only return something if the global state has
    /// already been requested and loaded.
    pub fn parse_active<S>(&self) -> Result<Option<S::Rx>, ClientError>
    where
        S: MakeRx,
        S::Rx: MakeUnrx<Unrx = S> + AnyFreeze + Clone,
    {
        match &self {
            // If there's an issue deserializing to this type, we'll fall back to the server
            Self::Loaded(any) => {
                let rx = any
                    .as_any()
                    .downcast_ref::<S::Rx>()
                    .ok_or(ClientInvariantError::GlobalStateDowncast)?
                    .clone();
                Ok(Some(rx))
            }
            Self::Server(_) | Self::None => Ok(None),
        }
    }
}
impl std::fmt::Debug for GlobalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalState").finish()
    }
}

/// Frozen global state.
#[cfg(any(client, doc))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FrozenGlobalState {
    /// There is state that should be instantiated.
    Some(String),
    /// The global state had not been modified from the engine-side. In this
    /// case, we don't bother storing the frozen state, since it can be
    /// trivially re-instantiated.
    Server,
    /// There was no global state.
    None,
    /// The frozen global state has already been used. This could be used to
    /// ignore a global state in the frozen version of an app that does use
    /// global state (as opposed to using `None` in such an app, which would
    /// cause an invariant error), however thaw preferences exsit for exactly
    /// this purpose.
    Used,
}
#[cfg(any(client, doc))]
impl From<&GlobalStateType> for FrozenGlobalState {
    fn from(val: &GlobalStateType) -> Self {
        match val {
            GlobalStateType::Loaded(state) => Self::Some(state.freeze()),
            GlobalStateType::None => Self::None,
            GlobalStateType::Server(_) => Self::Server,
        }
    }
}
