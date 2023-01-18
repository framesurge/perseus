use super::Turbine;
use crate::{
    error_views::ServerErrorData,
    errors::*,
    i18n::TranslationsManager,
    internal::{PageData, PageDataPartial},
    path::PathMaybeWithLocale,
    plugins::PluginAction,
    state::TemplateState,
    stores::MutableStore,
    utils::get_path_prefix_server,
};
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use futures::future::{try_join, try_join_all};
use serde_json::Value;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// Exports your app to a series of static files. If any templates/capsules
    /// in your app use request-time-only functionality, this will fail.
    pub async fn export(&mut self) -> Result<(), Arc<Error>> {
        // Note that this function uses different plugin actions from a pure build
        self.plugins
            .functional_actions
            .export_actions
            .before_export
            .run((), self.plugins.get_plugin_data())
            .map_err(|err| Arc::new(err.into()))?;
        let res = self.build_internal(true).await; // We mark that we will be exporting
        if let Err(err) = res {
            let err: Arc<Error> = Arc::new(err.into());
            self.plugins
                .functional_actions
                .export_actions
                .after_failed_build
                .run(err.clone(), self.plugins.get_plugin_data())
                .map_err(|err| Arc::new(err.into()))?;

            return Err(err);
        } else {
            self.plugins
                .functional_actions
                .export_actions
                .after_successful_build
                .run((), self.plugins.get_plugin_data())
                .map_err(|err| Arc::new(err.into()))?;
        }

        // By now, the global states have been written for each locale, along with the
        // render configuration (that's all in memory and in the immutable store)

        // This won't have any trailing slashes (they're stripped by the immutable store
        // initializer)
        let dest = format!("{}/exported", self.immutable_store.get_path());
        // Turn the build artifacts into self-contained static files
        let export_res = self.export_internal().await;
        if let Err(err) = export_res {
            let err: Arc<Error> = Arc::new(err.into());
            self.plugins
                .functional_actions
                .export_actions
                .after_failed_export
                .run(err.clone(), self.plugins.get_plugin_data())
                .map_err(|err| Arc::new(err.into()))?;

            Err(err)
        } else {
            self.copy_static_aliases(&dest)?;
            self.copy_static_dir(&dest)?;

            self.plugins
                .functional_actions
                .export_actions
                .after_successful_export
                .run((), self.plugins.get_plugin_data())
                .map_err(|err| Arc::new(err.into()))?;

            Ok(())
        }
    }

    // TODO Warnings for render cancellations in exported apps
    async fn export_internal(&self) -> Result<(), ServerError> {
        // Loop over every pair in the render config
        let mut export_futs = Vec::new();
        for (path, template_path) in self.render_cfg.iter() {
            export_futs.push(self.export_path(path, template_path));
        }
        // If we're using i18n, loop through the locales to create translations files
        let mut translations_futs = Vec::new();
        if self.locales.using_i18n {
            for locale in self.locales.get_all() {
                translations_futs.push(self.create_translation_file(locale));
            }
        }

        // Do *everything* in parallel
        try_join(try_join_all(export_futs), try_join_all(translations_futs)).await?;

        // Copying in bundles from the filesystem is done externally to this function

        Ok(())
    }
    /// This exports for all locales, or for none if the app doesn't use i18n.
    async fn export_path(&self, path: &str, template_path: &str) -> Result<(), ServerError> {
        // We assume we've already built the app, which would have populated this
        let html_shell = self.html_shell.as_ref().unwrap();

        let path_prefix = get_path_prefix_server();
        // We need the encoded path to reference flattened build artifacts
        // But we don't create a flattened system with exporting, everything is properly
        // created in a directory structure
        let path_encoded = urlencoding::encode(path).to_string();
        // All initial load pages should be written into their own folders, which
        // prevents a situation of a template root page outside the directory for the
        // rest of that template's pages (see #73). The `.html` file extension is
        // added when this variable is used (for contrast to the `.json`s)
        let initial_load_path = if path.ends_with("index") {
            // However, if it's already an index page, we don't want `index/index.html`
            path.to_string()
        } else {
            format!("{}/index", &path)
        };

        // Get the template itself
        let template = self.entities.get(template_path);
        let template = match template {
            Some(template) => template,
            None => {
                return Err(ServeError::PageNotFound {
                    path: template_path.to_string(),
                }
                .into())
            }
        };

        // Create a locale detection file for it if we're using i18n
        // These just send the app shell, which will perform a redirect as necessary
        // Notably, these also include fallback redirectors if either Wasm or JS is
        // disabled (or both)
        if self.locales.using_i18n && !template.is_capsule {
            self.immutable_store
                .write(
                    &format!("exported/{}.html", &initial_load_path),
                    &html_shell
                        .clone()
                        .locale_redirection_fallback(&format!(
                            "{}/{}/{}",
                            path_prefix, self.locales.default, &path
                        ))
                        .to_string(),
                )
                .await?;
        }
        // Check if that template uses build state (in which case it should have a JSON
        // file)
        let has_state = template.uses_build_state();
        if self.locales.using_i18n {
            // Loop through all the app's locales
            for locale in self.locales.get_all() {
                // This map was constructed from the locales, so each one must be in here
                let global_state = self.global_states_by_locale.get(locale).unwrap();

                let page_data = self
                    .get_static_page_data(
                        &format!("{}-{}", locale, &path_encoded),
                        has_state,
                        template.is_capsule,
                    )
                    .await?;

                // Don't create initial load pages for widgets
                if !template.is_capsule {
                    // Get the translations string for this locale
                    let translations = self
                        .translations_manager
                        .get_translations_str_for_locale(locale.to_string())
                        .await?;
                    // Create a full HTML file from those that can be served for initial loads
                    // The build process writes these with a dummy default locale even though we're
                    // not using i18n
                    let full_html = html_shell
                        .clone()
                        .page_data(&page_data, global_state, &translations)
                        .to_string();
                    self.immutable_store
                        .write(
                            &format!("exported/{}/{}.html", locale, initial_load_path),
                            &full_html,
                        )
                        .await?;
                }

                // Serialize the page data to JSON and write it as a partial (fetched by the app
                // shell for subsequent loads)
                let partial_page_data = PageDataPartial {
                    state: page_data.state,
                    head: page_data.head,
                };
                let partial = serde_json::to_string(&partial_page_data).unwrap();
                self.immutable_store
                    .write(
                        &format!("exported/.perseus/page/{}/{}.json", locale, &path),
                        &partial,
                    )
                    .await?;
            }
        } else {
            // For apps without i18n, the global state will still be built for the dummy
            // locale
            let global_state = self.global_states_by_locale.get("xx-XX").unwrap();

            let page_data = self
                .get_static_page_data(
                    &format!("{}-{}", self.locales.default, &path_encoded),
                    has_state,
                    template.is_capsule,
                )
                .await?;

            // Don't create initial load pages for widgets
            if !template.is_capsule {
                // Create a full HTML file from those that can be served for initial loads
                // The build process writes these with a dummy default locale even though we're
                // not using i18n
                let full_html = html_shell
                    .clone()
                    .page_data(&page_data, global_state, "")
                    .to_string();
                // We don't add an extension because this will be queried directly by the
                // browser
                self.immutable_store
                    .write(&format!("exported/{}.html", initial_load_path), &full_html)
                    .await?;
            }

            // Serialize the page data to JSON and write it as a partial (fetched by the app
            // shell for subsequent loads)
            let partial_page_data = PageDataPartial {
                state: page_data.state,
                head: page_data.head,
            };
            let partial = serde_json::to_string(&partial_page_data).unwrap();
            self.immutable_store
                .write(
                    &format!(
                        "exported/.perseus/page/{}/{}.json",
                        self.locales.default, &path
                    ),
                    &partial,
                )
                .await?;
        }

        Ok(())
    }
    async fn create_translation_file(&self, locale: &str) -> Result<(), ServerError> {
        // Get the translations string for that
        let translations_str = self
            .translations_manager
            .get_translations_str_for_locale(locale.to_string())
            .await?;
        // Write it to an asset so that it can be served directly
        self.immutable_store
            .write(
                &format!("exported/.perseus/translations/{}", locale),
                &translations_str,
            )
            .await?;

        Ok(())
    }
    /// This will work for capsules by just returning empty values
    /// for the parts of `PageData` that they can't fulfill. Importantly,
    /// capsules will be immediately converted into `PageDataPartial`s by
    /// the caller (since initial load pages don't need to be constructed).
    async fn get_static_page_data(
        &self,
        full_path_encoded: &str,
        has_state: bool,
        is_capsule: bool,
    ) -> Result<PageData, ServerError> {
        // Get the partial HTML content and a state to go with it (if applicable)
        let content = if !is_capsule {
            self.immutable_store
                .read(&format!("static/{}.html", full_path_encoded))
                .await?
        } else {
            String::new()
        };
        // This maps all the dependencies for any page that has a prerendered fragment
        let widget_states = if !is_capsule {
            self.immutable_store
                .read(&format!("static/{}.widgets.json", full_path_encoded))
                .await?
        } else {
            "{}".to_string()
        };
        // These are *not* fallible!
        let widget_states = match serde_json::from_str::<
            HashMap<PathMaybeWithLocale, (String, Value)>,
        >(&widget_states)
        {
            // Same processing as the server does
            Ok(widget_states) => widget_states
                .into_iter()
                .map(|(k, (_, v))| (k, Ok(v)))
                .collect::<_>(),
            Err(err) => return Err(ServerError::InvalidPageState { source: err }),
        };
        let head = if !is_capsule {
            self.immutable_store
                .read(&format!("static/{}.head.html", full_path_encoded))
                .await?
        } else {
            String::new()
        };
        let mut state = match has_state {
            true => serde_json::from_str(
                &self
                    .immutable_store
                    .read(&format!("static/{}.json", full_path_encoded))
                    .await?,
            )
            .map_err(|err| ServerError::InvalidPageState { source: err })?,
            false => TemplateState::empty().state,
        };
        // Widget states are always parsed as fallible on the browser-side
        // because initially loaded widgets actually can be. This is a server-side
        // workaround that we have to replicate here.
        if is_capsule {
            state = serde_json::to_value(Ok::<_, ()>(state)).unwrap();
        }
        // Create an instance of `PageData`
        Ok(PageData {
            content,
            state,
            head,
            widget_states,
        })
    }
    /// Copies the static aliases into a distribution directory at `dest` (no
    /// trailing `/`). This should be the root of the destination directory for
    /// the exported files. Because this provides a customizable
    /// destination, it is fully engine-agnostic.
    ///
    /// The error type here is a tuple of the location the asset was copied
    /// from, the location it was copied to, and the error in that process
    /// (which could be from `io` or `fs_extra`).
    fn copy_static_aliases(&self, dest: &str) -> Result<(), Arc<Error>> {
        // Loop through any static aliases and copy them in too
        // Unlike with the server, these could override pages!
        // We'll copy from the alias to the path (it could be a directory or a file)
        // Remember: `alias` has a leading `/`!
        for (alias, path) in &self.static_aliases {
            let from = PathBuf::from(path);
            let to = format!("{}{}", dest, alias);

            if from.is_dir() {
                if let Err(err) = copy_dir(&from, &to, &CopyOptions::new()) {
                    let err = EngineError::CopyStaticAliasDirErr {
                        source: err,
                        to,
                        from: path.to_string(),
                    };
                    let err: Arc<Error> = Arc::new(err.into());
                    self.plugins
                        .functional_actions
                        .export_actions
                        .after_failed_static_alias_dir_copy
                        .run(err.clone(), self.plugins.get_plugin_data())
                        .map_err(|err| Arc::new(err.into()))?;
                    return Err(err);
                }
            } else if let Err(err) = fs::copy(&from, &to) {
                let err = EngineError::CopyStaticAliasFileError {
                    source: err,
                    to,
                    from: path.to_string(),
                };
                let err: Arc<Error> = Arc::new(err.into());
                self.plugins
                    .functional_actions
                    .export_actions
                    .after_failed_static_alias_file_copy
                    .run(err.clone(), self.plugins.get_plugin_data())
                    .map_err(|err| Arc::new(err.into()))?;
                return Err(err);
            }
        }

        Ok(())
    }
    /// Copies the directory containing static data to be put in
    /// `/.perseus/static/` (URL). This takes in both the location of the
    /// static directory and the destination directory for exported files.
    fn copy_static_dir(&self, dest: &str) -> Result<(), Arc<Error>> {
        // Copy the `static` directory into the export package if it exists
        // If the user wants extra, they can use static aliases, plugins are unnecessary
        // here
        if self.static_dir.exists() {
            if let Err(err) = copy_dir(
                &self.static_dir,
                format!("{}/.perseus/", dest),
                &CopyOptions::new(),
            ) {
                let err = EngineError::CopyStaticDirError {
                    source: err,
                    path: self.static_dir.to_string_lossy().to_string(),
                    dest: dest.to_string(),
                };
                let err: Arc<Error> = Arc::new(err.into());
                self.plugins
                    .functional_actions
                    .export_actions
                    .after_failed_static_copy
                    .run(err.clone(), self.plugins.get_plugin_data())
                    .map_err(|err| Arc::new(err.into()))?;
                return Err(err);
            }
        }

        Ok(())
    }
}
