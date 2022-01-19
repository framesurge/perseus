// This file contains logic to define how templates are rendered

use crate::default_headers::default_headers;
use crate::errors::*;
use crate::global_state::GlobalState;
use crate::router::RouterState;
use crate::translator::Translator;
use crate::Html;
use crate::Request;
use crate::SsrNode;
use futures::Future;
use http::header::HeaderMap;
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use sycamore::context::{ContextProvider, ContextProviderProps};
use sycamore::prelude::{view, View};

/// This encapsulates all elements of context currently provided to Perseus templates. While this can be used manually, there are macros
/// to make this easier for each thing in here.
#[derive(Clone)]
pub struct RenderCtx {
    /// Whether or not we're being executed on the server-side. This can be used to gate `web_sys` functions and the like that expect
    /// to be run in the browser.
    pub is_server: bool,
    /// A translator for templates to use. This will still be present in non-i18n apps, but it will have no message IDs and support for
    /// the non-existent locale `xx-XX`. This uses an `Arc<T>` for thread-safety.
    pub translator: Translator,
    /// The router's state.
    pub router: RouterState,
    /// The global state for the app. This is a type map to which templates can add state that they need to access at a later time. Typically, interfacing with this will be done
    /// through the automation of the `#[perseus::template_with_rx_state(...)]` macro, but it can be used manually as well.
    pub global_state: GlobalState,
}

/// Represents all the different states that can be generated for a single template, allowing amalgamation logic to be run with the knowledge
/// of what did what (rather than blindly working on a vector).
#[derive(Default)]
pub struct States {
    /// Any state generated by the *build state* strategy.
    pub build_state: Option<String>,
    /// Any state generated by the *request state* strategy.
    pub request_state: Option<String>,
}
impl States {
    /// Creates a new instance of the states, setting both to `None`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Checks if both request state and build state are defined.
    pub fn both_defined(&self) -> bool {
        self.build_state.is_some() && self.request_state.is_some()
    }
    /// Gets the only defined state if only one is defined. If no states are defined, this will just return `None`. If both are defined,
    /// this will return an error.
    pub fn get_defined(&self) -> Result<Option<String>, ServeError> {
        if self.both_defined() {
            return Err(ServeError::BothStatesDefined);
        }

        if self.build_state.is_some() {
            Ok(self.build_state.clone())
        } else if self.request_state.is_some() {
            Ok(self.request_state.clone())
        } else {
            Ok(None)
        }
    }
}
/// A generic error type that can be adapted for any errors the user may want to return from a render function. `.into()` can be used
/// to convert most error types into this without further hassle. Otherwise, use `Box::new()` on the type.
pub type RenderFnResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
/// A generic error type that can be adapted for any errors the user may want to return from a render function, as with `RenderFnResult<T>`.
/// However, this also includes a mandatory statement of causation for any errors, which assigns blame for them to either the client
/// or the server. In cases where this is ambiguous, this allows returning accurate HTTP status codes.
///
/// Note that you can automatically convert from your error type into this with `.into()` or `?`, which will blame the server for the
/// error by default and return a *500 Internal Server Error* HTTP status code. Otherwise, you'll need to manually instantiate `ErrorWithCause`
/// and return that as the error type.
pub type RenderFnResultWithCause<T> = std::result::Result<T, GenericErrorWithCause>;

/// A generic return type for asynchronous functions that we need to store in a struct.
type AsyncFnReturn<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

/// Creates traits that prevent users from having to pin their functions' return types. We can't make a generic one until desugared function
/// types are stabilized (https://github.com/rust-lang/rust/issues/29625).
macro_rules! make_async_trait {
    ($name:ident, $return_ty:ty$(, $arg_name:ident: $arg:ty)*) => {
        // These traits should be purely internal, the user is likely to shoot themselves in the foot
        #[doc(hidden)]
        pub trait $name {
            fn call(
                &self,
                // Each given argument is repeated
                $(
                    $arg_name: $arg,
                )*
            ) -> AsyncFnReturn<$return_ty>;
        }
        impl<T, F> $name for T
        where
            T: Fn(
                $(
                    $arg,
                )*
            ) -> F,
            F: Future<Output = $return_ty> + Send + Sync + 'static,
        {
            fn call(
                &self,
                $(
                    $arg_name: $arg,
                )*
            ) -> AsyncFnReturn<$return_ty> {
                Box::pin(self(
                    $(
                        $arg_name,
                    )*
                ))
            }
        }
    };
}

// A series of asynchronous closure traits that prevent the user from having to pin their functions
make_async_trait!(GetBuildPathsFnType, RenderFnResult<Vec<String>>);
// The build state strategy needs an error cause if it's invoked from incremental
make_async_trait!(
    GetBuildStateFnType,
    RenderFnResultWithCause<String>,
    path: String,
    locale: String
);
make_async_trait!(
    GetRequestStateFnType,
    RenderFnResultWithCause<String>,
    path: String,
    locale: String,
    req: Request
);
make_async_trait!(ShouldRevalidateFnType, RenderFnResultWithCause<bool>);

// A series of closure types that should not be typed out more than once
/// The type of functions that are given a state and render a page. If you've defined state for your page, it's safe to `.unwrap()` the
/// given `Option`. If you're using i18n, an `Rc<Translator>` will also be made available through Sycamore's [context system](https://sycamore-rs.netlify.app/docs/advanced/advanced_reactivity).
pub type TemplateFn<G> = Box<dyn Fn(Option<String>) -> View<G> + Send + Sync>;
/// A type alias for the function that modifies the document head. This is just a template function that will always be server-side
/// rendered in function (it may be rendered on the client, but it will always be used to create an HTML string, rather than a reactive
/// template).
pub type HeadFn = TemplateFn<SsrNode>;
/// The type of functions that modify HTTP response headers.
pub type SetHeadersFn = Box<dyn Fn(Option<String>) -> HeaderMap + Send + Sync>;
/// The type of functions that get build paths.
pub type GetBuildPathsFn = Box<dyn GetBuildPathsFnType + Send + Sync>;
/// The type of functions that get build state.
pub type GetBuildStateFn = Box<dyn GetBuildStateFnType + Send + Sync>;
/// The type of functions that get request state.
pub type GetRequestStateFn = Box<dyn GetRequestStateFnType + Send + Sync>;
/// The type of functions that check if a template sghould revalidate.
pub type ShouldRevalidateFn = Box<dyn ShouldRevalidateFnType + Send + Sync>;
/// The type of functions that amalgamate build and request states.
pub type AmalgamateStatesFn =
    Box<dyn Fn(States) -> RenderFnResultWithCause<Option<String>> + Send + Sync>;

/// This allows the specification of all the template templates in an app and how to render them. If no rendering logic is provided at all,
/// the template will be prerendered at build-time with no state. All closures are stored on the heap to avoid hellish lifetime specification.
/// All properties for templates are passed around as strings to avoid type maps and other horrible things, this only adds one extra
/// deserialization call at build time. This only actually owns a two `String`s and a `bool`.
pub struct Template<G: Html> {
    /// The path to the root of the template. Any build paths will be inserted under this.
    path: String,
    /// A function that will render your template. This will be provided the rendered properties, and will be used whenever your template needs
    /// to be prerendered in some way. This should be very similar to the function that hydrates your template on the client side.
    /// This will be executed inside `sycamore::render_to_string`, and should return a `Template<SsrNode>`. This takes an `Option<Props>`
    /// because otherwise efficient typing is almost impossible for templates without any properties (solutions welcome in PRs!).
    template: TemplateFn<G>,
    /// A function that will be used to populate the document's `<head>` with metadata such as the title. This will be passed state in
    /// the same way as `template`, but will always be rendered to a string, whcih will then be interpolated directly into the `<head>`,
    /// so reactivity here will not work!
    head: TemplateFn<SsrNode>,
    /// A function to be run when the server returns an HTTP response. This should return headers for said response, given the template's
    /// state. The most common use-case of this is to add cache control that respects revalidation. This will only be run on successful
    /// responses, and does have the power to override existing headers. By default, this will create sensible cache control headers.
    set_headers: SetHeadersFn,
    /// A function that gets the paths to render for at built-time. This is equivalent to `get_static_paths` in NextJS. If
    /// `incremental_generation` is `true`, more paths can be rendered at request time on top of these.
    get_build_paths: Option<GetBuildPathsFn>,
    /// Defines whether or not any new paths that match this template will be prerendered and cached in production. This allows you to
    /// have potentially billions of templates and retain a super-fast build process. The first user will have an ever-so-slightly slower
    /// experience, and everyone else gets the beneftis afterwards. This requires `get_build_paths`. Note that the template root will NOT
    /// be rendered on demand, and must be explicitly defined if it's wanted. It can uuse a different template.
    incremental_generation: bool,
    /// A function that gets the initial state to use to prerender the template at build time. This will be passed the path of the template, and
    /// will be run for any sub-paths. This is equivalent to `get_static_props` in NextJS.
    get_build_state: Option<GetBuildStateFn>,
    /// A function that will run on every request to generate a state for that request. This allows server-side-rendering. This is equivalent
    /// to `get_server_side_props` in NextJS. This can be used with `get_build_state`, though custom amalgamation logic must be provided.
    get_request_state: Option<GetRequestStateFn>,
    /// A function to be run on every request to check if a template prerendered at build-time should be prerendered again. This is equivalent
    /// to revalidation after a time in NextJS, with the improvement of custom logic. If used with `revalidate_after`, this function will
    /// only be run after that time period. This function will not be parsed anything specific to the request that invoked it.
    should_revalidate: Option<ShouldRevalidateFn>,
    /// A length of time after which to prerender the template again. This is equivalent to revalidating in NextJS. This should specify a
    /// string interval to revalidate after. That will be converted into a datetime to wait for, which will be updated after every revalidation.
    /// Note that, if this is used with incremental generation, the counter will only start after the first render (meaning if you expect
    /// a weekly re-rendering cycle for all pages, they'd likely all be out of sync, you'd need to manually implement that with
    /// `should_revalidate`).
    revalidate_after: Option<String>,
    /// Custom logic to amalgamate potentially different states generated at build and request time. This is only necessary if your template
    /// uses both `build_state` and `request_state`. If not specified and both are generated, request state will be prioritized.
    amalgamate_states: Option<AmalgamateStatesFn>,
}
impl<G: Html> Template<G> {
    /// Creates a new template definition.
    pub fn new(path: impl Into<String> + std::fmt::Display) -> Self {
        Self {
            path: path.to_string(),
            template: Box::new(|_| sycamore::view! {}),
            // Unlike `template`, this may not be set at all (especially in very simple apps)
            head: Box::new(|_| sycamore::view! {}),
            // We create sensible header defaults here
            set_headers: Box::new(|_| default_headers()),
            get_build_paths: None,
            incremental_generation: false,
            get_build_state: None,
            get_request_state: None,
            should_revalidate: None,
            revalidate_after: None,
            amalgamate_states: None,
        }
    }

    // Render executors
    /// Executes the user-given function that renders the template on the server-side (build or request time).
    pub fn render_for_template(
        &self,
        props: Option<String>,
        translator: &Translator,
        is_server: bool,
        router_state: RouterState,
        global_state: GlobalState,
    ) -> View<G> {
        view! {
            // We provide the translator through context, which avoids having to define a separate variable for every translation due to Sycamore's `template!` macro taking ownership with `move` closures
            ContextProvider(ContextProviderProps {
                value: RenderCtx {
                    is_server,
                    translator: translator.clone(),
                    router: router_state,
                    global_state
                },
                children: || (self.template)(props)
            })
        }
    }
    /// Executes the user-given function that renders the document `<head>`, returning a string to be interpolated manually. Reactivity
    /// in this function will not take effect due to this string rendering. Note that this function will provide a translator context.
    pub fn render_head_str(&self, props: Option<String>, translator: &Translator) -> String {
        sycamore::render_to_string(|| {
            view! {
                // We provide the translator through context, which avoids having to define a separate variable for every translation due to Sycamore's `template!` macro taking ownership with `move` closures
                ContextProvider(ContextProviderProps {
                    value: RenderCtx {
                        // This function renders to a string, so we're effectively always on the server
                        // It's also only ever run on the server
                        is_server: true,
                        translator: translator.clone(),
                        // The head string is rendered to a string, and so never has information about router or global state
                        router: RouterState::default(),
                        global_state: GlobalState::default()
                    },
                    children: || (self.head)(props)
                })
            }
        })
    }
    /// Gets the list of templates that should be prerendered for at build-time.
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
    /// Gets the initial state for a template. This needs to be passed the full path of the template, which may be one of those generated by
    /// `.get_build_paths()`. This also needs the locale being rendered to so that more compelx applications like custom documentation
    /// systems can be enabled.
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
    /// Gets the request-time state for a template. This is equivalent to SSR, and will not be performed at build-time. Unlike
    /// `.get_build_paths()` though, this will be passed information about the request that triggered the render. Errors here can be caused
    /// by either the server or the client, so the user must specify an [`ErrorCause`]. This is also passed the locale being rendered to.
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
    /// Amalagmates given request and build states. Errors here can be caused by either the server or the client, so the user must specify
    /// an [`ErrorCause`].
    pub fn amalgamate_states(&self, states: States) -> Result<Option<String>, ServerError> {
        if let Some(amalgamate_states) = &self.amalgamate_states {
            let res = amalgamate_states(states);
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
    /// Checks, by the user's custom logic, if this template should revalidate. This function isn't presently parsed anything, but has
    /// network access etc., and can really do whatever it likes. Errors here can be caused by either the server or the client, so the
    /// user must specify an [`ErrorCause`].
    pub async fn should_revalidate(&self) -> Result<bool, ServerError> {
        if let Some(should_revalidate) = &self.should_revalidate {
            let res = should_revalidate.call().await;
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
    /// Gets the template's headers for the given state. These will be inserted into any successful HTTP responses for this template,
    /// and they have the power to override.
    pub fn get_headers(&self, state: Option<String>) -> HeaderMap {
        (self.set_headers)(state)
    }

    // Value getters
    /// Gets the path of the template. This is the root path under which any generated pages will be served. In the simplest case, there will
    /// only be one page rendered, and it will occupy that root position.
    pub fn get_path(&self) -> String {
        self.path.clone()
    }
    /// Gets the interval after which the template will next revalidate.
    pub fn get_revalidate_interval(&self) -> Option<String> {
        self.revalidate_after.clone()
    }

    // Render characteristic checkers
    /// Checks if this template can revalidate existing prerendered templates.
    pub fn revalidates(&self) -> bool {
        self.should_revalidate.is_some() || self.revalidate_after.is_some()
    }
    /// Checks if this template can revalidate existing prerendered templates after a given time.
    pub fn revalidates_with_time(&self) -> bool {
        self.revalidate_after.is_some()
    }
    /// Checks if this template can revalidate existing prerendered templates based on some given logic.
    pub fn revalidates_with_logic(&self) -> bool {
        self.should_revalidate.is_some()
    }
    /// Checks if this template can render more templates beyond those paths it explicitly defines.
    pub fn uses_incremental(&self) -> bool {
        self.incremental_generation
    }
    /// Checks if this template is a template to generate paths beneath it.
    pub fn uses_build_paths(&self) -> bool {
        self.get_build_paths.is_some()
    }
    /// Checks if this template needs to do anything on requests for it.
    pub fn uses_request_state(&self) -> bool {
        self.get_request_state.is_some()
    }
    /// Checks if this template needs to do anything at build time.
    pub fn uses_build_state(&self) -> bool {
        self.get_build_state.is_some()
    }
    /// Checks if this template has custom logic to amalgamate build and reqquest states if both are generated.
    pub fn can_amalgamate_states(&self) -> bool {
        self.amalgamate_states.is_some()
    }
    /// Checks if this template defines no rendering logic whatsoever. Such templates will be rendered using SSG. Basic templates can
    /// still modify headers.
    pub fn is_basic(&self) -> bool {
        !self.uses_build_paths()
            && !self.uses_build_state()
            && !self.uses_request_state()
            && !self.revalidates()
            && !self.uses_incremental()
    }

    // Builder setters
    // These will only be enabled under the `server-side` feature to prevent server-side code leaking into the Wasm binary (only the template setter is needed)
    /// Sets the template rendering function to use.
    pub fn template(
        mut self,
        val: impl Fn(Option<String>) -> View<G> + Send + Sync + 'static,
    ) -> Template<G> {
        self.template = Box::new(val);
        self
    }
    /// Sets the document head rendering function to use.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn head(
        mut self,
        val: impl Fn(Option<String>) -> View<SsrNode> + Send + Sync + 'static,
    ) -> Template<G> {
        // Headers are always prerendered on the server-side
        #[cfg(feature = "server-side")]
        {
            self.head = Box::new(val);
        }
        self
    }
    /// Sets the function to set headers. This will override Perseus' inbuilt header defaults.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn set_headers_fn(
        mut self,
        val: impl Fn(Option<String>) -> HeaderMap + Send + Sync + 'static,
    ) -> Template<G> {
        #[cfg(feature = "server-side")]
        {
            self.set_headers = Box::new(val);
        }
        self
    }
    /// Enables the *build paths* strategy with the given function.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn build_paths_fn(
        mut self,
        val: impl GetBuildPathsFnType + Send + Sync + 'static,
    ) -> Template<G> {
        #[cfg(feature = "server-side")]
        {
            self.get_build_paths = Some(Box::new(val));
        }
        self
    }
    /// Enables the *incremental generation* strategy.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn incremental_generation(mut self) -> Template<G> {
        #[cfg(feature = "server-side")]
        {
            self.incremental_generation = true;
        }
        self
    }
    /// Enables the *build state* strategy with the given function.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn build_state_fn(
        mut self,
        val: impl GetBuildStateFnType + Send + Sync + 'static,
    ) -> Template<G> {
        #[cfg(feature = "server-side")]
        {
            self.get_build_state = Some(Box::new(val));
        }
        self
    }
    /// Enables the *request state* strategy with the given function.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn request_state_fn(
        mut self,
        val: impl GetRequestStateFnType + Send + Sync + 'static,
    ) -> Template<G> {
        #[cfg(feature = "server-side")]
        {
            self.get_request_state = Some(Box::new(val));
        }
        self
    }
    /// Enables the *revalidation* strategy (logic variant) with the given function.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn should_revalidate_fn(
        mut self,
        val: impl ShouldRevalidateFnType + Send + Sync + 'static,
    ) -> Template<G> {
        #[cfg(feature = "server-side")]
        {
            self.should_revalidate = Some(Box::new(val));
        }
        self
    }
    /// Enables the *revalidation* strategy (time variant). This takes a time string of a form like `1w` for one week. More details are available
    /// [in the book](https://arctic-hen7.github.io/perseus/strategies/revalidation.html#time-syntax).
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn revalidate_after(mut self, val: String) -> Template<G> {
        #[cfg(feature = "server-side")]
        {
            self.revalidate_after = Some(val);
        }
        self
    }
    /// Enables state amalgamation with the given function.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn amalgamate_states_fn(
        mut self,
        val: impl Fn(States) -> RenderFnResultWithCause<Option<String>> + Send + Sync + 'static,
    ) -> Template<G> {
        #[cfg(feature = "server-side")]
        {
            self.amalgamate_states = Some(Box::new(val));
        }
        self
    }
}

/// Gets a `HashMap` of the given templates by their paths for serving. This should be manually wrapped for the pages your app provides
/// for convenience.
#[macro_export]
macro_rules! get_templates_map {
    [
        $($template:expr),+
    ] => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert(
                    $template.get_path(),
                    ::std::rc::Rc::new($template)
                );
            )+

            map
        }
    };
}

/// Gets a `HashMap` of the given templates by their paths for serving. This should be manually wrapped for the pages your app provides
/// for convenience.
///
/// This is the thread-safe version, which should only be used on the server.
#[macro_export]
macro_rules! get_templates_map_atomic {
    [
        $($template:expr),+
    ] => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert(
                    $template.get_path(),
                    ::std::sync::Arc::new($template)
                );
            )+

            map
        }
    };
}

/// A type alias for a `HashMap` of `Template`s. This uses `Rc`s to make the `Template`s cloneable. In server-side multithreading, `ArcTemplateMap` should be used instead.
pub type TemplateMap<G> = HashMap<String, Rc<Template<G>>>;
/// A type alias for a `HashMap` of `Template`s that uses `Arc`s for thread-safety. If you don't need to share templates between threads, use `TemplateMap` instead.
pub type ArcTemplateMap<G> = HashMap<String, Arc<Template<G>>>;

/// Checks if we're on the server or the client. This must be run inside a reactive scope (e.g. a `template!` or `create_effect`),
/// because it uses Sycamore context.
// TODO (0.4.0) Remove this altogether
#[macro_export]
#[deprecated(since = "0.3.1", note = "use `G::IS_BROWSER` instead")]
macro_rules! is_server {
    () => {{
        let render_ctx = ::sycamore::context::use_context::<::perseus::templates::RenderCtx>();
        render_ctx.is_server
    }};
}

/// Gets the `RenderCtx` efficiently.
#[macro_export]
macro_rules! get_render_ctx {
    () => {
        ::sycamore::context::use_context::<::perseus::templates::RenderCtx>()
    };
}
