use super::utils::PreloadInfo;
use crate::errors::*;
#[cfg(engine)]
use crate::i18n::Translator;
use crate::path::PathMaybeWithLocale;
#[cfg(engine)]
use crate::reactor::Reactor;
#[cfg(engine)]
use crate::reactor::RenderMode;
use crate::state::TemplateState;
#[cfg(engine)]
use crate::state::{BuildPaths, StateGeneratorInfo, UnknownStateType};
#[cfg(engine)]
use crate::template::default_headers;
use crate::template::TemplateInner;
#[cfg(engine)]
use crate::Request;
#[cfg(engine)]
use http::HeaderMap;
use sycamore::prelude::*;
#[cfg(engine)]
use sycamore::web::SsrNode;

impl TemplateInner {
    /// Executes the user-given function that renders the template on the
    /// client-side ONLY. This takes in an existing global state.
    ///
    /// This should NOT be used to render widgets!
    #[cfg(any(client, doc))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn render_for_template_client(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
    ) -> Result<(View, sycamore::reactive::NodeHandle), ClientError> {
        assert!(
            !self.is_capsule,
            "tried to render capsule with template logic"
        );

        // Only widgets use the preload info
        (self.view)(
            PreloadInfo {
                locale: String::new(),
                was_incremental_match: false,
            },
            state,
            path,
        )
    }
    /// Executes the user-given function that renders the template on the
    /// server-side ONLY. This automatically initializes an isolated global
    /// state.
    #[cfg(engine)]
    pub(crate) fn render_for_template_server(
        &self,
        path: PathMaybeWithLocale,
        state: TemplateState,
        global_state: TemplateState,
        mode: RenderMode,
        translator: &Translator,
    ) -> Result<View, ClientError> {
        assert!(
            !self.is_capsule,
            "tried to render capsule with template logic"
        );

        // The context we have here has no context elements set on it, so we set all the
        // defaults (job of the router component on the client-side)
        use sycamore::reactive::{create_child_scope, create_root};

        // IMPORTANT: When called during SSR (from render_to_string), we must NOT create
        // a new root because that would isolate us from the HydrationRegistry context.
        // Instead, we create a child scope which inherits parent contexts.
        //
        // We detect SSR context by checking if we're in an existing scope. During SSR,
        // render_to_string creates a scope, so we'll be in one. When called directly
        // (e.g., during static generation), we need to create a new root.

        // Check if we can access an existing scope by trying to use create_child_scope
        // If we're not in a scope, it will panic, so we catch that and create a root
        let in_existing_scope = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // This will succeed if we're in a scope, fail otherwise
            create_child_scope(|| {}).dispose();
        }))
        .is_ok();

        if in_existing_scope {
            // We're in an SSR context with existing scope - use child scope to inherit contexts
            let handle = create_child_scope(|| {});
            let view = handle.run_in(|| {
                Reactor::engine(global_state, mode, Some(translator)).add_self_to_cx();
                let preload_info = PreloadInfo {};
                let (view, _) = (self.view)(preload_info, state, path)?;
                Ok::<_, ClientError>(view)
            })?;
            handle.dispose();
            Ok(view)
        } else {
            // No existing scope - create a new root (e.g., static generation)
            let root_handle = create_root(|| {});
            let view = root_handle.run_in(|| {
                Reactor::engine(global_state, mode, Some(translator)).add_self_to_cx();
                let preload_info = PreloadInfo {};
                let (view, _) = (self.view)(preload_info, state, path)?;
                Ok::<_, ClientError>(view)
            })?;
            std::mem::forget(root_handle);
            Ok(view)
        }
    }
    /// Executes the user-given function that renders the document `<head>`,
    /// returning a string to be interpolated manually. Reactivity in this
    /// function will not take effect due to this string rendering. Note that
    /// this function will provide a translator context.
    #[cfg(engine)]
    pub(crate) fn render_head_str(
        &self,
        state: TemplateState,
        global_state: TemplateState,
        translator: &Translator,
    ) -> Result<String, ServerError> {
        // Set up the reactor in a root scope and render within it
        use sycamore::reactive::create_root;

        let root_handle = create_root(|| {});
        let prerendered = root_handle.run_in(|| {
            // The context we have here has no context elements set on it, so we set all the
            // defaults (job of the router component on the client-side)
            // We don't need the value, we just want the context instantiations
            // We don't need any page state store here
            Reactor::engine(global_state, RenderMode::Head, Some(translator)).add_self_to_cx();

            // Head rendering doesn't need hydration context
            let prerender_view = if let Some(head_fn) = &self.head {
                (head_fn)(state)?
            } else {
                View::new()
            };
            let prerendered = sycamore::render_to_string(|| prerender_view);
            Ok::<_, ServerError>(prerendered)
        })?;

        std::mem::forget(root_handle);
        Ok(prerendered)
    }
    /// Gets the list of templates that should be prerendered for at build-time.
    #[cfg(engine)]
    pub(crate) async fn get_build_paths(&self) -> Result<BuildPaths, ServerError> {
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
    #[cfg(engine)]
    pub(crate) async fn get_build_state(
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
    #[cfg(engine)]
    pub(crate) async fn get_request_state(
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
    #[cfg(engine)]
    pub(crate) async fn amalgamate_states(
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
    #[cfg(engine)]
    pub(crate) async fn should_revalidate(
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
    /// the power to override existing headers, including `Content-Type`.
    ///
    /// This will automatically instantiate a scope and set up an engine-side
    /// reactor so that the user's function can access global state and
    /// translations, as localized headers are very much real. Locale
    /// detection pages are considered internal to Perseus, and therefore do
    /// not have support for user headers (at this time).
    #[cfg(engine)]
    pub(crate) fn get_headers(
        &self,
        state: TemplateState,
        global_state: TemplateState,
        translator: Option<&Translator>,
    ) -> Result<HeaderMap, ServerError> {
        // Set up the reactor in a root scope and call header function within it
        use sycamore::reactive::create_root;

        let root_handle = create_root(|| {});
        let headers = root_handle.run_in(|| {
            let reactor = Reactor::engine(global_state, RenderMode::Headers, translator);
            reactor.add_self_to_cx();

            if let Some(header_fn) = &self.set_headers {
                (header_fn)(state)
            } else {
                Ok(default_headers())
            }
        })?;

        std::mem::forget(root_handle);
        Ok(headers)
    }
}
