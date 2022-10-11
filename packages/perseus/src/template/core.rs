// This file contains logic to define how templates are rendered

#[cfg(not(target_arch = "wasm32"))]
use super::default_headers;
use super::PageProps;
#[cfg(not(target_arch = "wasm32"))]
use super::RenderCtx;
use crate::errors::*;
#[cfg(not(target_arch = "wasm32"))]
use crate::make_async_trait;
use crate::translator::Translator;
use crate::utils::provide_context_signal_replace;
#[cfg(not(target_arch = "wasm32"))]
use crate::utils::AsyncFnReturn;
#[cfg(not(target_arch = "wasm32"))]
use crate::utils::ComputedDuration;
use crate::utils::PerseusDuration; /* We do actually want this in both the engine and the
                                    * browser */
use crate::Html;
#[cfg(not(target_arch = "wasm32"))]
use crate::Request;
#[cfg(not(target_arch = "wasm32"))]
use crate::SsrNode;
#[cfg(not(target_arch = "wasm32"))]
use futures::Future;
#[cfg(not(target_arch = "wasm32"))]
use http::header::HeaderMap;
use sycamore::prelude::{Scope, View};
#[cfg(not(target_arch = "wasm32"))]
use sycamore::utils::hydrate::with_no_hydration_context;

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
make_async_trait!(GetBuildPathsFnType, RenderFnResult<Vec<String>>);
// The build state strategy needs an error cause if it's invoked from
// incremental
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    GetBuildStateFnType,
    RenderFnResultWithCause<String>,
    path: String,
    locale: String
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    GetRequestStateFnType,
    RenderFnResultWithCause<String>,
    path: String,
    locale: String,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    ShouldRevalidateFnType,
    RenderFnResultWithCause<bool>,
    path: String,
    locale: String,
    req: Request
);
#[cfg(not(target_arch = "wasm32"))]
make_async_trait!(
    AmalgamateStatesFnType,
    RenderFnResultWithCause<String>,
    path: String,
    locale: String,
    build_state: String,
    request_state: String
);

// A series of closure types that should not be typed out more than once
/// The type of functions that are given a state and render a page. If you've
/// defined state for your page, it's safe to `.unwrap()` the given `Option`
/// inside `PageProps`. If you're using i18n, an `Rc<Translator>` will also be
/// made available through Sycamore's [context system](https://sycamore-rs.netlify.app/docs/advanced/advanced_reactivity).
pub type TemplateFn<G> = Box<dyn Fn(Scope, PageProps) -> View<G> + Send + Sync>;
/// A type alias for the function that modifies the document head. This is just
/// a template function that will always be server-side rendered in function (it
/// may be rendered on the client, but it will always be used to create an HTML
/// string, rather than a reactive template).
#[cfg(not(target_arch = "wasm32"))]
pub type HeadFn = TemplateFn<SsrNode>;
#[cfg(not(target_arch = "wasm32"))]
/// The type of functions that modify HTTP response headers.
pub type SetHeadersFn = Box<dyn Fn(Option<String>) -> HeaderMap + Send + Sync>;
/// The type of functions that get build paths.
#[cfg(not(target_arch = "wasm32"))]
pub type GetBuildPathsFn = Box<dyn GetBuildPathsFnType + Send + Sync>;
/// The type of functions that get build state.
#[cfg(not(target_arch = "wasm32"))]
pub type GetBuildStateFn = Box<dyn GetBuildStateFnType + Send + Sync>;
/// The type of functions that get request state.
#[cfg(not(target_arch = "wasm32"))]
pub type GetRequestStateFn = Box<dyn GetRequestStateFnType + Send + Sync>;
/// The type of functions that check if a template should revalidate.
#[cfg(not(target_arch = "wasm32"))]
pub type ShouldRevalidateFn = Box<dyn ShouldRevalidateFnType + Send + Sync>;
/// The type of functions that amalgamate build and request states.
#[cfg(not(target_arch = "wasm32"))]
pub type AmalgamateStatesFn = Box<dyn AmalgamateStatesFnType + Send + Sync>;

/// A single template in an app. Each template is comprised of a Sycamore view,
/// a state type, and some functions involved with generating that state. Pages
/// can then be generated from particular states. For instance, a single `docs`
/// template could have a state `struct` that stores a title and some content,
/// which could then render as many pages as desired.
///
/// You can read more about the templates system [here](https://arctic-hen7.github.io/perseus/en-US/docs/next/core-principles).
///
/// Note that all template states are passed around as `String`s to avoid
/// type maps and other inefficiencies, since they need to be transmitted over
/// the network anyway. As such, this `struct` is entirely state-agnostic,
/// since all the state-relevant functions merely return `String`s. The
/// various proc macros used to annotate such functions (e.g.
/// `#[perseus::build_state]`) perform serialization/deserialization
/// automatically for convenience.
pub struct Template<G: Html> {
    /// The path to the root of the template. Any build paths will be inserted
    /// under this.
    path: String,
    /// A function that will render your template. This will be provided the
    /// rendered properties, and will be used whenever your template needs
    /// to be prerendered in some way. This should be very similar to the
    /// function that hydrates your template on the client side.
    /// This will be executed inside `sycamore::render_to_string`, and should
    /// return a `Template<SsrNode>`. This takes an `Option<Props>`
    /// because otherwise efficient typing is almost impossible for templates
    /// without any properties (solutions welcome in PRs!).
    template: TemplateFn<G>,
    /// A function that will be used to populate the document's `<head>` with
    /// metadata such as the title. This will be passed state in
    /// the same way as `template`, but will always be rendered to a string,
    /// which will then be interpolated directly into the `<head>`,
    /// so reactivity here will not work!
    #[cfg(not(target_arch = "wasm32"))]
    head: TemplateFn<SsrNode>,
    /// A function to be run when the server returns an HTTP response. This
    /// should return headers for said response, given the template's state.
    /// The most common use-case of this is to add cache control that respects
    /// revalidation. This will only be run on successful responses, and
    /// does have the power to override existing headers. By default, this will
    /// create sensible cache control headers.
    #[cfg(not(target_arch = "wasm32"))]
    set_headers: SetHeadersFn,
    /// A function that gets the paths to render for at built-time. If
    /// `incremental_generation` is `true`, more paths can be rendered at
    /// request time on top of these.
    #[cfg(not(target_arch = "wasm32"))]
    get_build_paths: Option<GetBuildPathsFn>,
    /// Defines whether or not any new paths that match this template will be
    /// prerendered and cached in production. This allows you to
    /// have potentially billions of templates and retain a super-fast build
    /// process. The first user will have an ever-so-slightly slower
    /// experience, and everyone else gets the benefits afterwards. This
    /// requires `get_build_paths`. Note that the template root will NOT
    /// be rendered on demand, and must be explicitly defined if it's wanted. It
    /// can use a different template.
    #[cfg(not(target_arch = "wasm32"))]
    incremental_generation: bool,
    /// A function that gets the initial state to use to prerender the template
    /// at build time. This will be passed the path of the template, and
    /// will be run for any sub-paths.
    #[cfg(not(target_arch = "wasm32"))]
    get_build_state: Option<GetBuildStateFn>,
    /// A function that will run on every request to generate a state for that
    /// request. This allows server-side-rendering. This can be used with
    /// `get_build_state`, though custom amalgamation logic must be provided.
    #[cfg(not(target_arch = "wasm32"))]
    get_request_state: Option<GetRequestStateFn>,
    /// A function to be run on every request to check if a template prerendered
    /// at build-time should be prerendered again. If used with
    /// `revalidate_after`, this function will only be run after that time
    /// period. This function will not be parsed anything specific to the
    /// request that invoked it.
    #[cfg(not(target_arch = "wasm32"))]
    should_revalidate: Option<ShouldRevalidateFn>,
    /// A length of time after which to prerender the template again. The given
    /// duration will be waited for, and the next request after it will lead
    /// to a revalidation. Note that, if this is used with incremental
    /// generation, the counter will only start after the first render
    /// (meaning if you expect a weekly re-rendering cycle for all pages,
    /// they'd likely all be out of sync, you'd need to manually implement
    /// that with `should_revalidate`).
    #[cfg(not(target_arch = "wasm32"))]
    revalidate_after: Option<ComputedDuration>,
    /// Custom logic to amalgamate potentially different states generated at
    /// build and request time. This is only necessary if your template uses
    /// both `build_state` and `request_state`. If not specified and both are
    /// generated, request state will be prioritized.
    #[cfg(not(target_arch = "wasm32"))]
    amalgamate_states: Option<AmalgamateStatesFn>,
}
impl<G: Html> std::fmt::Debug for Template<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Template")
            .field("path", &self.path)
            .field("template", &"TemplateFn")
            .field("head", &"HeadFn")
            .field("set_headers", &"SetHeadersFn")
            // TODO Server-specific stuff
            .finish()
    }
}
impl<G: Html> Template<G> {
    /// Creates a new [`Template`]. By default, this has absolutely no
    /// associated data. If rendered, it would result in a blank screen.
    pub fn new(path: impl Into<String> + std::fmt::Display) -> Self {
        Self {
            path: path.to_string(),
            template: Box::new(|cx, _| sycamore::view! { cx, }),
            // Unlike `template`, this may not be set at all (especially in very simple apps)
            #[cfg(not(target_arch = "wasm32"))]
            head: Box::new(|cx, _| sycamore::view! { cx, }),
            // We create sensible header defaults here
            #[cfg(not(target_arch = "wasm32"))]
            set_headers: Box::new(|_| default_headers()),
            #[cfg(not(target_arch = "wasm32"))]
            get_build_paths: None,
            #[cfg(not(target_arch = "wasm32"))]
            incremental_generation: false,
            #[cfg(not(target_arch = "wasm32"))]
            get_build_state: None,
            #[cfg(not(target_arch = "wasm32"))]
            get_request_state: None,
            #[cfg(not(target_arch = "wasm32"))]
            should_revalidate: None,
            #[cfg(not(target_arch = "wasm32"))]
            revalidate_after: None,
            #[cfg(not(target_arch = "wasm32"))]
            amalgamate_states: None,
        }
    }

    // Render executors
    /// Executes the user-given function that renders the template on the
    /// client-side ONLY. This takes in an existing global state.
    #[cfg(target_arch = "wasm32")]
    #[allow(clippy::too_many_arguments)]
    pub fn render_for_template_client<'a>(
        &self,
        props: PageProps,
        cx: Scope<'a>,
        // Taking a reference here involves a serious risk of runtime panics, unfortunately (it's
        // simpler to own it at this point, and we clone it anyway internally)
        translator: Translator,
    ) -> View<G> {
        // The router component has already set up all the elements of context needed by
        // the rest of the system, we can get on with rendering the template All
        // we have to do is provide the translator, replacing whatever is present
        provide_context_signal_replace(cx, translator);

        (self.template)(cx, props)
    }
    /// Executes the user-given function that renders the template on the
    /// server-side ONLY. This automatically initializes an isolated global
    /// state.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_for_template_server<'a>(
        &self,
        props: PageProps,
        cx: Scope<'a>,
        translator: &Translator,
    ) -> View<G> {
        // The context we have here has no context elements set on it, so we set all the
        // defaults (job of the router component on the client-side)
        // We don't need the value, we just want the context instantiations
        let _ = RenderCtx::default().set_ctx(cx);
        // And now provide a translator separately
        provide_context_signal_replace(cx, translator.clone());

        (self.template)(cx, props)
    }
    /// Executes the user-given function that renders the document `<head>`,
    /// returning a string to be interpolated manually. Reactivity in this
    /// function will not take effect due to this string rendering. Note that
    /// this function will provide a translator context.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_head_str(&self, props: PageProps, translator: &Translator) -> String {
        sycamore::render_to_string(|cx| {
            // The context we have here has no context elements set on it, so we set all the
            // defaults (job of the router component on the client-side)
            // We don't need the value, we just want the context instantiations
            let _ = RenderCtx::default().set_ctx(cx);
            // And now provide a translator separately
            provide_context_signal_replace(cx, translator.clone());
            // We don't want to generate hydration keys for the head because it is static.
            with_no_hydration_context(|| (self.head)(cx, props))
        })
    }
    /// Gets the list of templates that should be prerendered for at build-time.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_build_paths(&self) -> Result<Vec<String>, ServerError> {
        if let Some(get_build_paths) = &self.get_build_paths {
            let res = get_build_paths.call().await;
            match res {
                Ok(res) => Ok(res),
                Err(err) => Err(ServerError::RenderFnFailed {
                    fn_name: "get_build_paths".to_string(),
                    template_name: self.get_path(),
                    cause: ErrorCause::Server(None),
                    source: err,
                }),
            }
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: self.path.clone(),
                feature_name: "build_paths".to_string(),
            }
            .into())
        }
    }
    /// Gets the initial state for a template. This needs to be passed the full
    /// path of the template, which may be one of those generated by
    /// `.get_build_paths()`. This also needs the locale being rendered to so
    /// that more complex applications like custom documentation systems can
    /// be enabled.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_build_state(
        &self,
        path: String,
        locale: String,
    ) -> Result<String, ServerError> {
        if let Some(get_build_state) = &self.get_build_state {
            let res = get_build_state.call(path, locale).await;
            match res {
                Ok(res) => Ok(res),
                Err(GenericErrorWithCause { error, cause }) => Err(ServerError::RenderFnFailed {
                    fn_name: "get_build_state".to_string(),
                    template_name: self.get_path(),
                    cause,
                    source: error,
                }),
            }
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: self.path.clone(),
                feature_name: "build_state".to_string(),
            }
            .into())
        }
    }
    /// Gets the request-time state for a template. This is equivalent to SSR,
    /// and will not be performed at build-time. Unlike `.get_build_paths()`
    /// though, this will be passed information about the request that triggered
    /// the render. Errors here can be caused by either the server or the
    /// client, so the user must specify an [`ErrorCause`]. This is also passed
    /// the locale being rendered to.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_request_state(
        &self,
        path: String,
        locale: String,
        req: Request,
    ) -> Result<String, ServerError> {
        if let Some(get_request_state) = &self.get_request_state {
            let res = get_request_state.call(path, locale, req).await;
            match res {
                Ok(res) => Ok(res),
                Err(GenericErrorWithCause { error, cause }) => Err(ServerError::RenderFnFailed {
                    fn_name: "get_request_state".to_string(),
                    template_name: self.get_path(),
                    cause,
                    source: error,
                }),
            }
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: self.path.clone(),
                feature_name: "request_state".to_string(),
            }
            .into())
        }
    }
    /// Amalgamates given request and build states. Errors here can be caused by
    /// either the server or the client, so the user must specify
    /// an [`ErrorCause`].
    ///
    /// This takes a separate build state and request state to ensure there are
    /// no `None`s for either of the states. This will only be called if both
    /// states are generated.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn amalgamate_states(
        &self,
        path: String,
        locale: String,
        build_state: String,
        request_state: String,
    ) -> Result<String, ServerError> {
        if let Some(amalgamate_states) = &self.amalgamate_states {
            let res = amalgamate_states
                .call(path, locale, build_state, request_state)
                .await;
            match res {
                Ok(res) => Ok(res),
                Err(GenericErrorWithCause { error, cause }) => Err(ServerError::RenderFnFailed {
                    fn_name: "amalgamate_states".to_string(),
                    template_name: self.get_path(),
                    cause,
                    source: error,
                }),
            }
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: self.path.clone(),
                feature_name: "request_state".to_string(),
            }
            .into())
        }
    }
    /// Checks, by the user's custom logic, if this template should revalidate.
    /// This function isn't presently parsed anything, but has
    /// network access etc., and can really do whatever it likes. Errors here
    /// can be caused by either the server or the client, so the
    /// user must specify an [`ErrorCause`].
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn should_revalidate(
        &self,
        path: String,
        locale: String,
        req: Request,
    ) -> Result<bool, ServerError> {
        if let Some(should_revalidate) = &self.should_revalidate {
            let res = should_revalidate.call(path, locale, req).await;
            match res {
                Ok(res) => Ok(res),
                Err(GenericErrorWithCause { error, cause }) => Err(ServerError::RenderFnFailed {
                    fn_name: "should_revalidate".to_string(),
                    template_name: self.get_path(),
                    cause,
                    source: error,
                }),
            }
        } else {
            Err(BuildError::TemplateFeatureNotEnabled {
                template_name: self.path.clone(),
                feature_name: "should_revalidate".to_string(),
            }
            .into())
        }
    }
    /// Gets the template's headers for the given state. These will be inserted
    /// into any successful HTTP responses for this template, and they have
    /// the power to override.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_headers(&self, state: Option<String>) -> HeaderMap {
        (self.set_headers)(state)
    }

    // Value getters
    /// Gets the path of the template. This is the root path under which any
    /// generated pages will be served. In the simplest case, there will
    /// only be one page rendered, and it will occupy that root position.
    pub fn get_path(&self) -> String {
        self.path.clone()
    }
    /// Gets the interval after which the template will next revalidate.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_revalidate_interval(&self) -> Option<ComputedDuration> {
        self.revalidate_after.clone()
    }

    // Render characteristic checkers
    /// Checks if this template can revalidate existing prerendered templates.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn revalidates(&self) -> bool {
        self.should_revalidate.is_some() || self.revalidate_after.is_some()
    }
    /// Checks if this template can revalidate existing prerendered templates
    /// after a given time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn revalidates_with_time(&self) -> bool {
        self.revalidate_after.is_some()
    }
    /// Checks if this template can revalidate existing prerendered templates
    /// based on some given logic.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn revalidates_with_logic(&self) -> bool {
        self.should_revalidate.is_some()
    }
    /// Checks if this template can render more templates beyond those paths it
    /// explicitly defines.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_incremental(&self) -> bool {
        self.incremental_generation
    }
    /// Checks if this template is a template to generate paths beneath it.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_build_paths(&self) -> bool {
        self.get_build_paths.is_some()
    }
    /// Checks if this template needs to do anything on requests for it.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_request_state(&self) -> bool {
        self.get_request_state.is_some()
    }
    /// Checks if this template needs to do anything at build time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_build_state(&self) -> bool {
        self.get_build_state.is_some()
    }
    /// Checks if this template has custom logic to amalgamate build and
    /// request states if both are generated.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn can_amalgamate_states(&self) -> bool {
        self.amalgamate_states.is_some()
    }
    /// Checks if this template defines no rendering logic whatsoever. Such
    /// templates will be rendered using SSG. Basic templates can
    /// still modify headers.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn is_basic(&self) -> bool {
        !self.uses_build_paths()
            && !self.uses_build_state()
            && !self.uses_request_state()
            && !self.revalidates()
            && !self.uses_incremental()
    }

    // Builder setters
    // The server-only ones have a different version for Wasm that takes in an empty
    // function (this means we don't have to bring in function types, and therefore
    // we can avoid bringing in the whole `http` module --- a very significant
    // saving!) The macros handle the creation of empty functions to make user's
    // lives easier
    /// Sets the template rendering function to use. This function might take in
    /// some state (use the `#[perseus::template_rx]` macro for serialization
    /// convenience) and/or some global state, and then it must return a
    /// Sycamore [`View`].
    pub fn template(
        mut self,
        val: impl Fn(Scope, PageProps) -> View<G> + Send + Sync + 'static,
    ) -> Template<G> {
        self.template = Box::new(val);
        self
    }

    /// Sets the document `<head>` rendering function to use. The [`View`]
    /// produced by this will only be rendered on the engine-side, and will
    /// *not* be reactive (since it only contains metadata).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn head(
        mut self,
        val: impl Fn(Scope, PageProps) -> View<SsrNode> + Send + Sync + 'static,
    ) -> Template<G> {
        // Headers are always prerendered on the server-side
        self.head = Box::new(val);
        self
    }
    /// Sets the document `<head>` rendering function to use. The [`View`]
    /// produced by this will only be rendered on the engine-side, and will
    /// *not* be reactive (since it only contains metadata).
    #[cfg(target_arch = "wasm32")]
    pub fn head(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Sets the function to set headers. This will override Perseus' inbuilt
    /// header defaults.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_headers_fn(
        mut self,
        val: impl Fn(Option<String>) -> HeaderMap + Send + Sync + 'static,
    ) -> Template<G> {
        self.set_headers = Box::new(val);
        self
    }
    /// Sets the function to set headers. This will override Perseus' inbuilt
    /// header defaults.
    #[cfg(target_arch = "wasm32")]
    pub fn set_headers_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *build paths* strategy with the given function.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_paths_fn(
        mut self,
        val: impl GetBuildPathsFnType + Send + Sync + 'static,
    ) -> Template<G> {
        self.get_build_paths = Some(Box::new(val));
        self
    }
    /// Enables the *build paths* strategy with the given function.
    #[cfg(target_arch = "wasm32")]
    pub fn build_paths_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *incremental generation* strategy.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn incremental_generation(mut self) -> Template<G> {
        self.incremental_generation = true;
        self
    }
    /// Enables the *incremental generation* strategy.
    #[cfg(target_arch = "wasm32")]
    pub fn incremental_generation(self) -> Template<G> {
        self
    }

    /// Enables the *build state* strategy with the given function.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_state_fn(
        mut self,
        val: impl GetBuildStateFnType + Send + Sync + 'static,
    ) -> Template<G> {
        self.get_build_state = Some(Box::new(val));
        self
    }
    /// Enables the *build state* strategy with the given function.
    #[cfg(target_arch = "wasm32")]
    pub fn build_state_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *request state* strategy with the given function.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn request_state_fn(
        mut self,
        val: impl GetRequestStateFnType + Send + Sync + 'static,
    ) -> Template<G> {
        self.get_request_state = Some(Box::new(val));
        self
    }
    /// Enables the *request state* strategy with the given function.
    #[cfg(target_arch = "wasm32")]
    pub fn request_state_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *revalidation* strategy (logic variant) with the given
    /// function.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn should_revalidate_fn(
        mut self,
        val: impl ShouldRevalidateFnType + Send + Sync + 'static,
    ) -> Template<G> {
        self.should_revalidate = Some(Box::new(val));
        self
    }
    /// Enables the *revalidation* strategy (logic variant) with the given
    /// function.
    #[cfg(target_arch = "wasm32")]
    pub fn should_revalidate_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *revalidation* strategy (time variant). This takes a time
    /// string of a form like `1w` for one week.
    ///
    ///    - s: second,
    ///    - m: minute,
    ///    - h: hour,
    ///    - d: day,
    ///    - w: week,
    ///    - M: month (30 days used here, 12M ≠ 1y!),
    ///    - y: year (365 days always, leap years ignored, if you want them add
    ///      them as days)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn revalidate_after<I: PerseusDuration>(mut self, val: I) -> Template<G> {
        let computed_duration = match val.into_computed() {
            Ok(val) => val,
            // This is fine, because this will be checked when we try to build the app (i.e. it'll
            // show up before runtime)
            Err(_) => panic!("invalid revalidation interval"),
        };
        self.revalidate_after = Some(computed_duration);
        self
    }
    /// Enables the *revalidation* strategy (time variant). This takes a time
    /// string of a form like `1w` for one week.
    ///
    ///    - s: second,
    ///    - m: minute,
    ///    - h: hour,
    ///    - d: day,
    ///    - w: week,
    ///    - M: month (30 days used here, 12M ≠ 1y!),
    ///    - y: year (365 days always, leap years ignored, if you want them add
    ///      them as days)
    #[cfg(target_arch = "wasm32")]
    pub fn revalidate_after<I: PerseusDuration>(self, _val: I) -> Template<G> {
        self
    }

    /// Enables state amalgamation with the given function. State amalgamation
    /// allows you to have one template generate state at both build time
    /// and request time. The function you provide here is responsible for
    /// rationalizing the two into one single state to be sent to the client,
    /// and this will be run just after the request state function
    /// completes. See [`States`] for further details.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn amalgamate_states_fn(
        mut self,
        val: impl AmalgamateStatesFnType + Send + Sync + 'static,
    ) -> Template<G> {
        self.amalgamate_states = Some(Box::new(val));
        self
    }
    /// Enables state amalgamation with the given function. State amalgamation
    /// allows you to have one template generate state at both build time
    /// and request time. The function you provide here is responsible for
    /// rationalizing the two into one single state to be sent to the client,
    /// and this will be run just after the request state function
    /// completes. See [`States`] for further details.
    #[cfg(target_arch = "wasm32")]
    pub fn amalgamate_states_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }
}

// The engine needs to know whether or not to use hydration, this is how we pass
// those feature settings through
#[cfg(not(feature = "hydrate"))]
#[doc(hidden)]
pub(crate) type TemplateNodeType = sycamore::prelude::DomNode;
#[cfg(feature = "hydrate")]
#[doc(hidden)]
pub(crate) type TemplateNodeType = sycamore::prelude::HydrateNode;

/// Checks if we're on the server or the client. This must be run inside a
/// reactive scope (e.g. a `view!` or `create_effect`), because it uses
/// Sycamore context.
// TODO (0.4.0) Remove this altogether
#[macro_export]
#[deprecated(since = "0.3.1", note = "use `G::IS_BROWSER` instead")]
macro_rules! is_server {
    () => {{
        let render_ctx = ::sycamore::context::use_context::<::perseus::templates::RenderCtx>();
        render_ctx.is_server
    }};
}
