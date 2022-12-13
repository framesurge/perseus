use super::core::PreloadInfo;
use crate::{
    errors::*,
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

/// A custom `enum` representation of a `Result`-style type whose error is a
/// `Box` that can accept any thread-safe error type. This is used internally as
/// a conversion type so that your state generation functions (e.g.
/// `get_build_state`) can return *either* some type `T`, or a `Result<T, E>`,
/// where `E` is of your choosing. This type appears as
/// `Into<GeneratorResult<T>>` in several function signatures, and that
/// `From`/`Into` relation is automatically implemented for all the types you'll
/// need in your state generation functions.
///
/// You can think of this as another way of implementing a `MaybeFallible`
/// trait, which would be more elegant, but doesn't work yet due to the overlap
/// between `T` and `Result<T, E>` (which could itself be interpreted as `T`).
/// Until certain nightly features of Rust as stabilized, this is not possible
/// without copious type parameter specification.
///
/// You should never need to use this type yourself, consider it an internal
/// conversion type.
pub enum GeneratorResult<T> {
    /// Equivalent to `Result::Ok`.
    Ok(T),
    /// Equivalent to `Result::Err`.
    Err(Box<dyn std::error::Error + Send + Sync>),
}
impl<T> GeneratorResult<T> {
    /// Converts this `enum` into a `Result` amenable to typical usage within
    /// Perseus' engine-side. Since this type has no blame, any errors will
    /// be implicitly blamed on the server with no special status
    /// code (leading to the return of a *500 Internal Server Error* if the
    /// error propagates at request-time).
    pub(crate) fn to_server_result(
        self,
        fn_name: &str,
        template_name: String,
    ) -> Result<T, ServerError> {
        match self {
            Self::Ok(val) => Ok(val),
            Self::Err(err) => Err(ServerError::RenderFnFailed {
                fn_name: fn_name.to_string(),
                template_name,
                blame: ErrorBlame::Server(None),
                source: err,
            }),
        }
    }
}
/// The same as [`GeneratorResult`], except this uses a [`GenericBlamedError`]
/// as its error type, which is essentially a `Box`ed generic error with an
/// attached [`ErrorBlame`] denoting who is responsible for the error: the
/// client or the server. You'll see this as a convertion type in the signatures
/// of functions that might be run at reuqest-time (e.g. `get_request_state`
/// might have an error caused by a missing file, which would be the server's
/// fault, or a malformed cookie, which would be the client's fault).
///
/// For a function that returns `Into<BlamedGeneratorResult<T>>`, you can return
/// either `T` directly, or `Result<T, BlamedError<E>>`: see [`BlamedError`] for
/// further information. (Note that the `?` operator can automatically turn `E`
/// into `BlamedError<E>`, setting the server as the one to blame.)
pub enum BlamedGeneratorResult<T> {
    /// Equivalent to `Result::Ok`.
    Ok(T),
    /// Equivalent to `Result::Err`.
    Err(GenericBlamedError),
}
impl<T> BlamedGeneratorResult<T> {
    /// Converts this `enum` into a `Result` amenable to typical usage within
    /// Perseus' engine-side. This will use the underlying error blame.
    pub(crate) fn to_server_result(
        self,
        fn_name: &str,
        template_name: String,
    ) -> Result<T, ServerError> {
        match self {
            Self::Ok(val) => Ok(val),
            Self::Err(err) => Err(ServerError::RenderFnFailed {
                fn_name: fn_name.to_string(),
                template_name,
                blame: err.blame,
                source: err.error,
            }),
        }
    }
}

// We manually implement everything we need here (and only what we need). A
// neater approach would be a `MaybeFallible` trait, but that needs an
// implementation for both `T` and `Result<T, E>`, which overlap. With
// rust-lang/rust#31844, that would be possible, but that seems to be quite a
// while away.

// Build paths
impl From<BuildPaths> for GeneratorResult<BuildPaths> {
    fn from(val: BuildPaths) -> Self {
        Self::Ok(val)
    }
}
impl<E: std::error::Error + Send + Sync + 'static> From<Result<BuildPaths, E>>
    for GeneratorResult<BuildPaths>
{
    fn from(val: Result<BuildPaths, E>) -> Self {
        match val {
            Ok(val) => Self::Ok(val),
            Err(err) => Self::Err(err.into()),
        }
    }
}
// Head
impl From<View<SsrNode>> for GeneratorResult<View<SsrNode>> {
    fn from(val: View<SsrNode>) -> Self {
        Self::Ok(val)
    }
}
impl<E: std::error::Error + Send + Sync + 'static> From<Result<View<SsrNode>, E>>
    for GeneratorResult<View<SsrNode>>
{
    fn from(val: Result<View<SsrNode>, E>) -> Self {
        match val {
            Ok(val) => Self::Ok(val),
            Err(err) => Self::Err(err.into()),
        }
    }
}
// Headers
impl From<HeaderMap> for GeneratorResult<HeaderMap> {
    fn from(val: HeaderMap) -> Self {
        Self::Ok(val)
    }
}
impl<E: std::error::Error + Send + Sync + 'static> From<Result<HeaderMap, E>>
    for GeneratorResult<HeaderMap>
{
    fn from(val: Result<HeaderMap, E>) -> Self {
        match val {
            Ok(val) => Self::Ok(val),
            Err(err) => Self::Err(err.into()),
        }
    }
}
// Build/request state and state amalgamation (blamed; they all have the same
// return types)
impl<S: Serialize + DeserializeOwned + MakeRx> From<S> for BlamedGeneratorResult<S> {
    fn from(val: S) -> Self {
        Self::Ok(val)
    }
}
impl<S: Serialize + DeserializeOwned + MakeRx, E: std::error::Error + Send + Sync + 'static>
    From<Result<S, BlamedError<E>>> for BlamedGeneratorResult<S>
{
    fn from(val: Result<S, BlamedError<E>>) -> Self {
        match val {
            Ok(val) => Self::Ok(val),
            Err(err) => Self::Err(err.to_boxed()),
        }
    }
}
// Should revalidate (blamed)
impl From<bool> for BlamedGeneratorResult<bool> {
    fn from(val: bool) -> Self {
        Self::Ok(val)
    }
}
impl<E: std::error::Error + Send + Sync + 'static> From<Result<bool, BlamedError<E>>>
    for BlamedGeneratorResult<bool>
{
    fn from(val: Result<bool, BlamedError<E>>) -> Self {
        match val {
            Ok(val) => Self::Ok(val),
            Err(err) => Self::Err(err.to_boxed()),
        }
    }
}

// A series of asynchronous closure traits that prevent the user from having to
// pin their functions
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(crate) GetBuildPathsFnType,
    Result<BuildPaths, ServerError>
);
// The build state strategy needs an error cause if it's invoked from
// incremental
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(super) GetBuildStateFnType,
    Result<TemplateState, ServerError>,
    info: StateGeneratorInfo<UnknownStateType>
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(super) GetRequestStateFnType,
    Result<TemplateState, ServerError>,
    info: StateGeneratorInfo<UnknownStateType>,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(super) ShouldRevalidateFnType,
    Result<bool, ServerError>,
    info: StateGeneratorInfo<UnknownStateType>,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub(super) AmalgamateStatesFnType,
    Result<TemplateState, ServerError>,
    info: StateGeneratorInfo<UnknownStateType>,
    build_state: TemplateState,
    request_state: TemplateState
);

// These traits are for the functions users provide to us! They are NOT stored
// internally! As `R` denotes reference reactive state elsewhere, `V` is used
// here for return types. Also, for some reason macros don't do `>>`, so we need
// random spaces.
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub GetBuildPathsUserFnType< V: Into< GeneratorResult<BuildPaths> > >,
    V
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub GetBuildStateUserFnType< S: Serialize + DeserializeOwned + MakeRx, B: Serialize + DeserializeOwned + Send + Sync, V: Into< BlamedGeneratorResult<S> > >,
    V,
    info: StateGeneratorInfo<B>
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub GetRequestStateUserFnType< S: Serialize + DeserializeOwned + MakeRx, B: Serialize + DeserializeOwned + Send + Sync, V: Into< BlamedGeneratorResult<S> > >,
    V,
    info: StateGeneratorInfo<B>,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub ShouldRevalidateUserFnType< B: Serialize + DeserializeOwned + Send + Sync, V: Into< BlamedGeneratorResult<bool> >  >,
    V,
    info: StateGeneratorInfo<B>,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    pub AmalgamateStatesUserFnType< S: Serialize + DeserializeOwned + MakeRx, B: Serialize + DeserializeOwned + Send + Sync, V: Into< BlamedGeneratorResult<S> > >,
    V,
    info: StateGeneratorInfo<B>,
    build_state: S,
    request_state: S
);

// A series of closure types that should not be typed out more than once
/// The type of functions that are given a state and render a page.
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
// Note: the head and header functions have render errors constructed inside
// their closures!
/// A type alias for the function that modifies the document head. This is just
/// a template function that will always be server-side rendered in function (it
/// may be rendered on the client, but it will always be used to create an HTML
/// string, rather than a reactive template).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type HeadFn =
    Box<dyn Fn(Scope, TemplateState) -> Result<View<SsrNode>, ServerError> + Send + Sync>;
#[cfg(not(target_arch = "wasm32"))]
/// The type of functions that modify HTTP response headers.
pub(crate) type SetHeadersFn =
    Box<dyn Fn(TemplateState) -> Result<HeaderMap, ServerError> + Send + Sync>;
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
