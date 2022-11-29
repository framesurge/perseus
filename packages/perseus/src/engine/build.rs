use crate::build::{build_app, BuildProps};
use crate::errors::Error;
use crate::{
    i18n::TranslationsManager, plugins::PluginAction, stores::MutableStore, PerseusAppBase, SsrNode,
};
use std::rc::Rc;

/// Builds the app, calling all necessary plugin opportunities. This works
/// solely with the properties provided in the given
/// [`PerseusApp`](crate::PerseusApp), so this is entirely engine-agnostic.
///
/// Note that this expects to be run in the root of the project.
pub async fn build<M: MutableStore, T: TranslationsManager>(
    app: PerseusAppBase<SsrNode, M, T>,
) -> Result<(), Rc<Error>> {
    let plugins = app.get_plugins();

    plugins
        .functional_actions
        .build_actions
        .before_build
        .run((), plugins.get_plugin_data())
        .map_err(|err| Rc::new(err.into()))?;

    let immutable_store = app
        .get_immutable_store()
        .map_err(|err| Rc::new(err.into()))?;
    let mutable_store = app.get_mutable_store();
    let locales = app.get_locales().map_err(|err| Rc::new(err.into()))?;
    let gsc = app.get_global_state_creator();

    // Build the site for all the common locales (done in parallel)
    // All these parameters can be modified by `PerseusApp` and plugins, so there's
    // no point in having a plugin opportunity here
    let templates_map = app
        .get_atomic_templates_map()
        .map_err(|err| Rc::new(err.into()))?;

    // We have to get the translations manager last, because it consumes everything
    let translations_manager = app.get_translations_manager().await;

    let res = build_app(BuildProps {
        templates: &templates_map,
        locales: &locales,
        immutable_store: &immutable_store,
        mutable_store: &mutable_store,
        translations_manager: &translations_manager,
        global_state_creator: &gsc,
        exporting: false,
    })
    .await;
    if let Err(err) = res {
        let err: Rc<Error> = Rc::new(err.into());
        plugins
            .functional_actions
            .build_actions
            .after_failed_build
            .run(err.clone(), plugins.get_plugin_data())
            .map_err(|err| Rc::new(err.into()))?;

        Err(err)
    } else {
        plugins
            .functional_actions
            .build_actions
            .after_successful_build
            .run((), plugins.get_plugin_data())
            .map_err(|err| Rc::new(err.into()))?;

        Ok(())
    }
}
