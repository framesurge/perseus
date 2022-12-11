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
    init::PerseusAppBase,
    plugins::Plugins,
    server::HtmlShell,
    state::{GlobalStateCreator, TemplateState},
    stores::{ImmutableStore, MutableStore},
    template::{ArcCapsuleMap, ArcTemplateMap},
};
use futures::executor::block_on;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use sycamore::web::SsrNode;

/// The Perseus state generator.
pub struct Turbine<M: MutableStore, T: TranslationsManager> {
    /// All the templates in the app.
    templates: ArcTemplateMap<SsrNode>,
    /// The app's error views.
    error_views: Arc<ErrorViews<SsrNode>>,
    /// All the capsule fallbacks in the app.
    capsule_fallbacks: ArcCapsuleMap<SsrNode>,
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
    render_cfg: HashMap<String, String>,
    /// A map of locale to global state. This is kept cached throughout the
    /// build process, since every template we build will require it to be
    /// provided through context.
    global_states_by_locale: HashMap<String, TemplateState>,
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
        let templates = app.get_atomic_templates_map();
        let capsule_fallbacks = app.get_atomic_capsules_map();
        let locales = app.get_locales()?;
        let immutable_store = app.get_immutable_store()?;
        let mutable_store = app.get_mutable_store();
        let global_state_creator = app.get_global_state_creator();
        let plugins = app.get_plugins();
        let index_view_str = app.get_index_view_str();
        let root_id = app.get_root()?;
        let static_dir = app.get_static_dir();
        let static_aliases = app.get_static_aliases()?;
        let error_views = app.get_atomic_error_views();
        // This consumes the app
        // Note that we can't do anything in parallel with this anyway
        let translations_manager = block_on(app.get_translations_manager());

        Ok(Self {
            templates,
            capsule_fallbacks,
            locales,
            immutable_store,
            mutable_store,
            global_state_creator,
            plugins,
            index_view_str,
            root_id,
            static_dir: PathBuf::from(&static_dir),
            static_aliases,
            error_views,
            translations_manager,

            // If we're going from a `PerseusApp`, these will be filled in later
            render_cfg: HashMap::new(),
            global_states_by_locale: HashMap::new(),
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

        // Get all the global states
        let mut global_states_by_locale = HashMap::new();
        for locale in self.locales.get_all() {
            // IMPORTANT: A global state that doesn't generate at build-time won't have a
            // corresponding file!
            let res = self
                .immutable_store
                .read(&format!("static/global_state_{}.json", &locale))
                .await;
            let global_state = match res {
                Ok(state) => {
                    let state = TemplateState::from_str(&state)
                        .map_err(|err| ServerError::InvalidPageState { source: err })?;
                    state
                }
                Err(StoreError::NotFound { .. }) => TemplateState::empty(),
                Err(err) => return Err(err.into()),
            };

            global_states_by_locale.insert(locale.to_string(), global_state);
        }
        self.global_states_by_locale = global_states_by_locale;

        let html_shell = PerseusAppBase::<SsrNode, M, T>::get_html_shell(
            self.index_view_str.to_string(),
            &self.root_id,
            &self.render_cfg,
            &self.immutable_store,
            &self.plugins,
        )
        .await?;
        self.html_shell = Some(html_shell);

        Ok(())
    }
}
