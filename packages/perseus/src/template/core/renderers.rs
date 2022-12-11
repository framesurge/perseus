use sycamore::web::Html;
use crate::template::Template;
use std::rc::Rc;
use sycamore::{prelude::Scope, view::View, web::SsrNode};
use crate::path::PathMaybeWithLocale;
use crate::i18n::Translator;
#[cfg(not(target_arch = "wasm32"))]
use crate::reactor::RenderMode;
use crate::utils::provide_context_signal_replace;
use super::utils::PreloadInfo;
use crate::errors::*;
use crate::state::{TemplateState, UnknownStateType, BuildPaths, StateGeneratorInfo};
use crate::Request;
#[cfg(not(target_arch = "wasm32"))]
use http::HeaderMap;
#[cfg(not(target_arch = "wasm32"))]
use crate::reactor::Reactor;
#[cfg(target_arch = "wasm32")]
use sycamore::prelude::ScopeDisposer;

impl<G: Html> Template<G> {
    /// Executes the user-given function that renders the template on the
    /// client-side ONLY. This takes in an existing global state.
    ///
    /// This should NOT be used to render widgets!
    #[cfg(target_arch = "wasm32")]
    #[allow(clippy::too_many_arguments)]
    pub fn render_for_template_client<'a>(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
        cx: Scope<'a>,
    ) -> Result<(View<G>, ScopeDisposer), ClientError> {
        // Only widgets use the preload info
        (self.template)(cx, PreloadInfo { locale: String::new(), was_incremental_match: false }, state, path)
    }
    /// Executes the user-given function that renders the *widget* on the
    /// client-side ONLY. This takes in an existing global state.
    ///
    /// This should NOT be used to render pages!
    #[cfg(target_arch = "wasm32")]
    #[allow(clippy::too_many_arguments)]
    pub fn render_widget_for_template_client<'a>(
        &self,
        path: PathMaybeWithLocale,
        cx: Scope<'a>,
        preload_info: PreloadInfo,
    ) -> Result<(View<G>, ScopeDisposer<'a>), ClientError> {
        // The template state is ignored by widgets, they fetch it themselves asynchronously
        (self.template)(cx, preload_info, TemplateState::empty(), path)
    }
    /// Executes the user-given function that renders the template on the
    /// server-side ONLY. This automatically initializes an isolated global
    /// state.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn render_for_template_server<'a>(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
        global_state: TemplateState,
        mode: RenderMode<SsrNode>,
        cx: Scope<'a>,
        translator: &Translator,
    ) -> Result<View<G>, ClientError> {
        // The context we have here has no context elements set on it, so we set all the
        // defaults (job of the router component on the client-side)
        // We don't need the value, we just want the context instantiations
        let _ = Reactor::engine(global_state, mode, Some(translator)).add_self_to_cx(cx);
        // This is used for widget preloading, which doesn't occur on the engine-side
        let preload_info = PreloadInfo {
            locale: String::new(),
            was_incremental_match: false,
        };
        // We don't care about the scope disposer, since this scope is unique anyway
        let (view, _) = (self.template)(cx, preload_info, state, path)?;
        Ok(view)
    }
    /// Executes the user-given function that renders the capsule on the server-side
    /// ONLY. This takes the scope from a previous call of `.render_for_template_server()`,
    /// assuming the render context and translator have already been fully instantiated.
    pub(crate) fn render_widget_for_template_server<'a>(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
        cx: Scope<'a>,
    ) -> Result<View<G>, ClientError> {
        // This is used for widget preloading, which doesn't occur on the engine-side
        let preload_info = PreloadInfo {
            locale: String::new(),
            was_incremental_match: false,
        };
        // We don't care about the scope disposer, since this scope is unique anyway
        let (view, _) = (self.template)(cx, preload_info, state, path)?;
        Ok(view)
    }
    /// Executes the user-given function that renders the document `<head>`,
    /// returning a string to be interpolated manually. Reactivity in this
    /// function will not take effect due to this string rendering. Note that
    /// this function will provide a translator context.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_head_str(
        &self,
        state: TemplateState,
        global_state: TemplateState,
        translator: &Translator,
    ) -> Result<String, ClientError> {
        use sycamore::{prelude::create_scope_immediate, utils::hydrate::with_no_hydration_context};

        // This is a bit roundabout for error handling
        let mut prerender_view = Ok(View::empty());
        create_scope_immediate(|cx| {
            // The context we have here has no context elements set on it, so we set all the
            // defaults (job of the router component on the client-side)
            // We don't need the value, we just want the context instantiations
            // We don't need any page state store here
            let _ = Reactor::<G>::engine(global_state, RenderMode::Head, Some(translator)).add_self_to_cx(cx);

            prerender_view = with_no_hydration_context(|| {
                (self.head)(cx, state)
            });
        });
        let prerender_view = prerender_view?;
        let prerendered = sycamore::render_to_string(|_| {
            prerender_view
        });

        Ok(prerendered)
    }
    /// Gets the list of templates that should be prerendered for at build-time.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_build_paths(&self) -> Result<BuildPaths, ServerError> {
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
        info: StateGeneratorInfo<UnknownStateType>,
    ) -> Result<TemplateState, ServerError> {
        if let Some(get_build_state) = &self.get_build_state {
            let res = get_build_state.call(info).await;
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
        info: StateGeneratorInfo<UnknownStateType>,
        req: Request,
    ) -> Result<TemplateState, ServerError> {
        if let Some(get_request_state) = &self.get_request_state {
            let res = get_request_state.call(info, req).await;
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
        info: StateGeneratorInfo<UnknownStateType>,
        build_state: TemplateState,
        request_state: TemplateState,
    ) -> Result<TemplateState, ServerError> {
        if let Some(amalgamate_states) = &self.amalgamate_states {
            let res = amalgamate_states
                .call(info, build_state, request_state)
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
                feature_name: "amalgamate_states".to_string(),
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
        info: StateGeneratorInfo<UnknownStateType>,
        req: Request,
    ) -> Result<bool, ServerError> {
        if let Some(should_revalidate) = &self.should_revalidate {
            let res = should_revalidate.call(info, req).await;
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
    pub fn get_headers(&self, state: TemplateState) -> Result<HeaderMap, ServerError> {
        let headers = (self.set_headers)(state)?;
        Ok(headers)
    }
}
