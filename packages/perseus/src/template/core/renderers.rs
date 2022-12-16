use super::utils::PreloadInfo;
use crate::errors::*;
#[cfg(not(target_arch = "wasm32"))]
use crate::i18n::Translator;
use crate::path::PathMaybeWithLocale;
#[cfg(not(target_arch = "wasm32"))]
use crate::reactor::Reactor;
#[cfg(not(target_arch = "wasm32"))]
use crate::reactor::RenderMode;
use crate::state::TemplateState;
#[cfg(not(target_arch = "wasm32"))]
use crate::state::{BuildPaths, StateGeneratorInfo, UnknownStateType};
use crate::template::Template;
#[cfg(not(target_arch = "wasm32"))]
use crate::Request;
#[cfg(not(target_arch = "wasm32"))]
use http::HeaderMap;
#[cfg(target_arch = "wasm32")]
use sycamore::prelude::ScopeDisposer;
use sycamore::web::Html;
#[cfg(not(target_arch = "wasm32"))]
use sycamore::web::SsrNode;
use sycamore::{prelude::Scope, view::View};

impl<G: Html> Template<G> {
    /// Executes the user-given function that renders the template on the
    /// client-side ONLY. This takes in an existing global state.
    ///
    /// This should NOT be used to render widgets!
    #[cfg(target_arch = "wasm32")]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn render_for_template_client<'a>(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
        cx: Scope<'a>,
    ) -> Result<(View<G>, ScopeDisposer<'a>), ClientError> {
        // Only widgets use the preload info
        (self.template)(
            cx,
            PreloadInfo {
                locale: String::new(),
                was_incremental_match: false,
            },
            state,
            path,
        )
    }
    /// Executes the user-given function that renders the *widget* on the
    /// client-side ONLY. This takes in an existing global state. This will
    /// ignore its internal scope disposer, since the given scope **must**
    /// be a page-level scope, which will be disposed from the root when the
    /// page changes, thereby disposing of all the child scopes, like those
    /// used for widgets.
    ///
    /// This should NOT be used to render pages!
    #[cfg(target_arch = "wasm32")]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn render_widget_for_template_client(
        &self,
        path: PathMaybeWithLocale,
        cx: Scope,
        preload_info: PreloadInfo,
    ) -> Result<View<G>, ClientError> {
        // The template state is ignored by widgets, they fetch it themselves
        // asynchronously
        let (view, _disposer) = (self.template)(cx, preload_info, TemplateState::empty(), path)?;
        Ok(view)
    }
    /// Executes the user-given function that renders the template on the
    /// server-side ONLY. This automatically initializes an isolated global
    /// state.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn render_for_template_server(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
        global_state: TemplateState,
        mode: RenderMode<SsrNode>,
        cx: Scope,
        translator: &Translator,
    ) -> Result<View<G>, ClientError> {
        // The context we have here has no context elements set on it, so we set all the
        // defaults (job of the router component on the client-side)
        // We don't need the value, we just want the context instantiations
        Reactor::engine(global_state, mode, Some(translator)).add_self_to_cx(cx);
        // This is used for widget preloading, which doesn't occur on the engine-side
        let preload_info = PreloadInfo {};
        // We don't care about the scope disposer, since this scope is unique anyway
        let (view, _) = (self.template)(cx, preload_info, state, path)?;
        Ok(view)
    }
    /// Executes the user-given function that renders the capsule on the
    /// server-side ONLY. This takes the scope from a previous call of
    /// `.render_for_template_server()`, assuming the reactor has already
    /// been fully instantiated.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn render_widget_for_template_server(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
        cx: Scope,
    ) -> Result<View<G>, ClientError> {
        // This is used for widget preloading, which doesn't occur on the engine-side
        let preload_info = PreloadInfo {};
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
    ) -> Result<String, ServerError> {
        use sycamore::{
            prelude::create_scope_immediate, utils::hydrate::with_no_hydration_context,
        };

        // This is a bit roundabout for error handling
        let mut prerender_view = Ok(View::empty());
        create_scope_immediate(|cx| {
            // The context we have here has no context elements set on it, so we set all the
            // defaults (job of the router component on the client-side)
            // We don't need the value, we just want the context instantiations
            // We don't need any page state store here
            Reactor::<G>::engine(global_state, RenderMode::Head, Some(translator))
                .add_self_to_cx(cx);

            prerender_view = with_no_hydration_context(|| (self.head)(cx, state));
        });
        let prerender_view = prerender_view?;
        let prerendered = sycamore::render_to_string(|_| prerender_view);

        Ok(prerendered)
    }
    /// Gets the list of templates that should be prerendered for at build-time.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_build_paths(&self) -> Result<BuildPaths, ServerError> {
        if let Some(get_build_paths) = &self.get_build_paths {
            get_build_paths.call().await
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
            get_build_state.call(info).await
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
    /// client, so the user must specify an [`ErrorBlame`]. This is also passed
    /// the locale being rendered to.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_request_state(
        &self,
        info: StateGeneratorInfo<UnknownStateType>,
        req: Request,
    ) -> Result<TemplateState, ServerError> {
        if let Some(get_request_state) = &self.get_request_state {
            get_request_state.call(info, req).await
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
    /// an [`ErrorBlame`].
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
            amalgamate_states
                .call(info, build_state, request_state)
                .await
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
    /// user must specify an [`ErrorBlame`].
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn should_revalidate(
        &self,
        info: StateGeneratorInfo<UnknownStateType>,
        req: Request,
    ) -> Result<bool, ServerError> {
        if let Some(should_revalidate) = &self.should_revalidate {
            should_revalidate.call(info, req).await
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
        (self.set_headers)(state)
    }
}