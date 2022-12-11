use super::core::PreloadInfo;
use crate::{
    errors::{ClientError, GenericErrorWithCause},
    make_async_trait,
    path::PathMaybeWithLocale,
    state::{BuildPaths, MakeRx, StateGeneratorInfo, TemplateState, UnknownStateType},
    utils::AsyncFnReturn,
    Request,
};
use futures::Future;
#[cfg(not(target_arch = "wasm32"))]
use http::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
use sycamore::{
    prelude::{Scope, ScopeDisposer},
    view::View,
    web::SsrNode,
};

/// A generic error type that can be adapted for any errors the user may want to
/// return from a render function. `.into()` can be used to convert most error
/// types into this without further hassle. Otherwise, use `Box::new()` on the
/// type.
pub type RenderFnResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
/// A generic error type that can be adapted for any errors the user may want to
/// return from a render function, as with [`RenderFnResult<T>`]. However, this
/// also includes a mandatory statement of causation for any errors, which
/// assigns blame for them to either the client or the server. In cases where
/// this is ambiguous, this allows returning accurate HTTP status codes.
///
/// Note that you can automatically convert from your error type into this with
/// `.into()` or `?`, which will blame the server for the error by default and
/// return a *500 Internal Server Error* HTTP status code. Otherwise, you'll
/// need to manually instantiate [`GenericErrorWithCause`] and return that as
/// the error type. Alternatively, you could use
/// [`blame_err!`](crate::blame_err).
pub type RenderFnResultWithCause<T> = std::result::Result<T, GenericErrorWithCause>;

// A series of asynchronous closure traits that prevent the user from having to
// pin their functions
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(pub GetBuildPathsFnType, RenderFnResult<BuildPaths>); // This doubles as the user type
                                                                        // The build state strategy needs an error cause if it's invoked from
                                                                        // incremental
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(super) GetBuildStateFnType,
    RenderFnResultWithCause<TemplateState>,
    info: StateGeneratorInfo<UnknownStateType>
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(super) GetRequestStateFnType,
    RenderFnResultWithCause<TemplateState>,
    info: StateGeneratorInfo<UnknownStateType>,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(super) ShouldRevalidateFnType,
    RenderFnResultWithCause<bool>,
    info: StateGeneratorInfo<UnknownStateType>,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(super) AmalgamateStatesFnType,
    RenderFnResultWithCause<TemplateState>,
    info: StateGeneratorInfo<UnknownStateType>,
    build_state: TemplateState,
    request_state: TemplateState
);

// These traits are for the functions users provide to us! They are NOT stored
// internally!
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub GetBuildStateUserFnType<S: Serialize + DeserializeOwned + MakeRx, B: Serialize + DeserializeOwned + Send + Sync>,
    RenderFnResultWithCause<S>,
    info: StateGeneratorInfo<B>
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub GetRequestStateUserFnType<S: Serialize + DeserializeOwned + MakeRx, B: Serialize + DeserializeOwned + Send + Sync>,
    RenderFnResultWithCause<S>,
    info: StateGeneratorInfo<B>,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub ShouldRevalidateUserFnType<B: Serialize + DeserializeOwned + Send + Sync>,
    RenderFnResultWithCause<bool>,
    info: StateGeneratorInfo<B>,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub AmalgamateStatesUserFnType<S: Serialize + DeserializeOwned + MakeRx, B: Serialize + DeserializeOwned + Send + Sync>,
    RenderFnResultWithCause<S>,
    info: StateGeneratorInfo<B>,
    build_state: S,
    request_state: S
);

// A series of closure types that should not be typed out more than once
/// The type of functions that are given a state and render a page. If you've
/// defined state for your page, it's safe to `.unwrap()` the given `Option`
/// inside `PageProps`. If you're using i18n, an `Rc<Translator>` will also be
/// made available through Sycamore's [context system](https://sycamore-rs.netlify.app/docs/advanced/advanced_reactivity).
pub(crate) type TemplateFn<G> = Box<
    dyn for<'a> Fn(
            Scope<'a>,
            PreloadInfo,
            TemplateState,
            PathMaybeWithLocale,
        ) -> Result<(View<G>, ScopeDisposer<'a>), ClientError>
        + Send
        + Sync,
>;
/// A type alias for the function that modifies the document head. This is just
/// a template function that will always be server-side rendered in function (it
/// may be rendered on the client, but it will always be used to create an HTML
/// string, rather than a reactive template).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type HeadFn =
    Box<dyn Fn(Scope, TemplateState) -> Result<View<SsrNode>, ClientError> + Send + Sync>;
#[cfg(not(target_arch = "wasm32"))]
/// The type of functions that modify HTTP response headers.
pub(crate) type SetHeadersFn =
    Box<dyn Fn(TemplateState) -> Result<HeaderMap, ClientError> + Send + Sync>;
/// The type of functions that get build paths.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type GetBuildPathsFn = Box<dyn GetBuildPathsFnType + Send + Sync>;
/// The type of functions that get build state.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type GetBuildStateFn = Box<dyn GetBuildStateFnType + Send + Sync>;
/// The type of functions that get request state.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type GetRequestStateFn = Box<dyn GetRequestStateFnType + Send + Sync>;
/// The type of functions that check if a template should revalidate.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type ShouldRevalidateFn = Box<dyn ShouldRevalidateFnType + Send + Sync>;
/// The type of functions that amalgamate build and request states.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type AmalgamateStatesFn = Box<dyn AmalgamateStatesFnType + Send + Sync>;
