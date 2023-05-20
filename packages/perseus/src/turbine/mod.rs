//! The internals of Perseus' state generation platform. This is not responsible
//! for the reactivity of state, or any other browser-side work. This is
//! responsible for the actual *generation* of state on the engine-side, at both
//! build-time and request-time.
//!
//! If you wanted to isolate the core of engine-side Perseus, it would be this
//! module.

mod build;
mod build_error_page;
mod export;
mod export_error_page;
mod serve;
/// This has the actual API endpoints.
mod server;
mod tinker;

pub use server::{ApiResponse, SubsequentLoadQueryParams};

use crate::{
    error_views::ErrorViews,
    errors::*,
    i18n::{Locales, TranslationsManager},
    init::{PerseusAppBase, Tm},
    plugins::Plugins,
    server::HtmlShell,
    state::{GlobalStateCreator, TemplateState},
    stores::{ImmutableStore, MutableStore},
    template::EntityMap,
};
use futures::executor::block_on;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use sycamore::web::SsrNode;

/// The Perseus state generator.
#[derive(Debug)]
pub struct Turbine<M: MutableStore, T: TranslationsManager> {
    /// All the templates and capsules in the app.
    entities: EntityMap<SsrNode>,
    /// The app's error views.
    error_views: Arc<ErrorViews<SsrNode>>,
    /// The app's locales data.
    locales: Locales,
    /// An immutable store.
    immutable_store: ImmutableStore,
    /// A mutable store.
    mutable_store: M,
    /// A translations manager.
    translations_manager: T,
    /// The global state creator.
    global_state_creator: Arc<GlobalStateCreator>,
    plugins: Arc<Plugins>,
    index_view_str: String,
    root_id: String,
    /// This is stored as a `PathBuf` so we can easily check whether or not it
    /// exists.
    pub static_dir: PathBuf,
    /// The app's static aliases.
    pub static_aliases: HashMap<String, String>,
    // --- These may not be populated at creation ---
    /// The app's render configuration, a map of paths in the app to the names
    /// of the templates that generated them. (Since templates can have
    /// multiple `/` delimiters in their names.)
    ///
    /// Since the paths are not actually valid paths, we leave them typed as
    /// `String`s, but these keys are in effect `PathWithoutLocale` instances.
    render_cfg: HashMap<String, String>,
    /// The app's global state, kept cached throughout the build process because
    /// every template we build will need access to it through context.
    global_state: TemplateState,
    /// Custom URL path prefix for perseus.
    pub path_prefix_server: Option<String>,
    /// The HTML shell that can be used for constructing the full pages this app
    /// returns.
    html_shell: Option<HtmlShell>,
}

// We want to be able to create a turbine straight from an app base
impl<M: MutableStore, T: TranslationsManager> TryFrom<PerseusAppBase<SsrNode, M, T>>
    for Turbine<M, T>
{
    type Error = PluginError;

    fn try_from(app: PerseusAppBase<SsrNode, M, T>) -> Result<Self, Self::Error> {
        let locales = app.get_locales()?;
        let immutable_store = app.get_immutable_store()?;
        let index_view_str = app.get_index_view_str();
        let root_id = app.get_root()?;
        let static_aliases = app.get_static_aliases()?;

        Ok(Self {
            entities: app.entities,
            locales,
            immutable_store,
            mutable_store: app.mutable_store,
            global_state_creator: app.global_state_creator,
            plugins: app.plugins,
            index_view_str,
            root_id,
            static_dir: PathBuf::from(&app.static_dir),
            static_aliases,
            #[cfg(debug_assertions)]
            error_views: app.error_views.unwrap_or_default(),
            #[cfg(not(debug_assertions))]
            error_views: app
                .error_views
                .expect("you must provide your own error pages in production"),
            // This consumes the app
            // Note that we can't do anything in parallel with this anyway
            translations_manager: match app.translations_manager {
                Tm::Dummy(tm) => tm,
                Tm::Full(tm) => block_on(tm),
            },

            // If we're going from a `PerseusApp`, these will be filled in later
            render_cfg: HashMap::new(),
            // This will be immediately overriden
            global_state: TemplateState::empty(),
            path_prefix_server: None,
            html_shell: None,
        })
    }
}

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Updates some internal fields of the turbine by assuming the app has been
    /// built in the past. This expects a number of things to exist in the
    /// filesystem. Note that calling `.build()` will automatically perform
    /// this population.
    pub async fn populate_after_build(&mut self) -> Result<(), ServerError> {
        // Get the render config
        let render_cfg_str = self.immutable_store.read("render_conf.json").await?;
        let render_cfg = serde_json::from_str::<HashMap<String, String>>(&render_cfg_str)
            .map_err(|err| ServerError::BuildError(BuildError::RenderCfgInvalid { source: err }))?;
        self.render_cfg = render_cfg;

        // Get the global state
        let global_state = self.immutable_store.read("static/global_state.json").await;
        self.global_state = match global_state {
            Ok(state) => TemplateState::from_str(&state)
                .map_err(|err| ServerError::InvalidPageState { source: err })?,
            Err(StoreError::NotFound { .. }) => TemplateState::empty(),
            Err(err) => return Err(err.into()),
        };

        let html_shell = PerseusAppBase::<SsrNode, M, T>::get_html_shell(
            self.index_view_str.to_string(),
            &self.root_id,
            &self.render_cfg,
            &self.plugins,
            self.get_path_prefix_server().as_ref(),
        )
        .await?;
        self.html_shell = Some(html_shell);

        Ok(())
    }

    /// Gets the path prefix to apply on the server. If `path_prefix_server` is not set
    /// on `Turbine`, `PERSEUS_BASE_PATH` environment variable is used to resolve,
    /// which avoids hardcoding something as changeable as this into the final binary.
    /// Hence however, that variable must be the same as what's set in `<base>`
    /// (done automatically).
    /// Trailing forward slashes will be trimmed automatically.
    #[cfg(engine)]
    pub fn get_path_prefix_server(&self) -> String {
        self.path_prefix_server.clone().unwrap_or_else(|| {
            use std::env;
            let base_path = env::var("PERSEUS_BASE_PATH").unwrap_or_else(|_| "".to_string());
            base_path
                .strip_suffix('/')
                .unwrap_or(&base_path)
                .to_string()
        })
    }
}
