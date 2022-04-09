use crate::plugins::PluginAction;
use crate::server::{get_render_cfg, HtmlShell};
use crate::utils::get_path_prefix_server;
use crate::{
    i18n::{FsTranslationsManager, Locales, TranslationsManager},
    state::GlobalStateCreator,
    stores::{FsMutableStore, ImmutableStore, MutableStore},
    templates::TemplateMap,
    ErrorPages, Html, Plugins, Template,
};
use futures::Future;
use std::pin::Pin;
use std::{collections::HashMap, rc::Rc};
use sycamore::{
    prelude::{component, view},
    view::View,
    SsrNode,
};

/// The default index view, because some simple apps won't need anything fancy here. The user should be able to provide the smallest possible amount of information for their app to work.
/// Note that this doesn't declare a character set or anything else (for maximal cross-language compatibility)
static DFLT_INDEX_VIEW: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset=UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    </head>
    <body>
        <div id="root"></div>
    </body>
</html>"#;

// This is broken out for debug implementation ease
struct TemplateGetters<G: Html>(Vec<Box<dyn Fn() -> Template<G>>>);
impl<G: Html> std::fmt::Debug for TemplateGetters<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateGetters").finish()
    }
}
// This is broken out for debug implementation ease
struct ErrorPagesGetter<G: Html>(Box<dyn Fn() -> ErrorPages<G>>);
impl<G: Html> std::fmt::Debug for ErrorPagesGetter<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorPagesGetters").finish()
    }
}

/// The different types of translations managers that can be stored. This allows us to store dummy translations managers directly, without holding futures. If this stores a full
/// translations manager though, it will store it as a `Future`, which is later evaluated.
enum Tm<T: TranslationsManager> {
    Dummy(T),
    Full(Pin<Box<dyn Future<Output = T>>>),
}
impl<T: TranslationsManager> std::fmt::Debug for Tm<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tm").finish()
    }
}

/// An automatically implemented trait for asynchronous functions that return instances of `TranslationsManager`. This is needed so we can store the 'promise' of getting a translations
/// manager in future by executing a stored asynchronous function (because we don't want to take in the actual value, which would require asynchronous initialization functions, which we
/// can't have in environments like the browser).
#[doc(hidden)]
pub trait TranslationsManagerGetter {
    type Output: TranslationsManager;
    fn call(&self) -> Box<dyn Future<Output = Self::Output>>;
}
impl<T, F, Fut> TranslationsManagerGetter for F
where
    T: TranslationsManager,
    F: Fn() -> Fut,
    Fut: Future<Output = T> + 'static,
{
    type Output = T;
    fn call(&self) -> Box<dyn Future<Output = Self::Output>> {
        Box::new(self())
    }
}

/// The options for constructing a Perseus app. These encompass all the information Perseus needs to know to create your app. Every Perseus app using the engine must export one of these.
///
/// Note that this is an interim storage point, it's not depended on by any part of the core logic, and therefore custom engines can entirely ignore this.
#[derive(Debug)]
pub struct PerseusAppBase<G: Html, M: MutableStore, T: TranslationsManager> {
    /// The HTML ID of the root `<div>` element into which Perseus will be injected.
    root: String,
    /// A list of function that produce templates for the app to use. These are stored as functions so that they can be called an arbitrary number of times.
    // From this, we can construct the necessary kind of template map (we can call the user-given functions an arbitrary number of times)
    template_getters: TemplateGetters<G>,
    /// The app's error pages.
    error_pages: ErrorPagesGetter<G>,
    /// The global state creator for the app.
    global_state_creator: GlobalStateCreator,
    /// The internationalization information for the app.
    locales: Locales,
    /// The static aliases the app serves.
    static_aliases: HashMap<String, String>,
    /// The plugins the app uses.
    plugins: Rc<Plugins<G>>,
    /// The app's immutable store.
    immutable_store: ImmutableStore,
    /// The HTML template that'll be used to render the app into. This must be static, but can be generated or sourced in any way. Note that this MUST
    /// contain a `<div>` with the `id` set to whatever the value of `self.root` is.
    index_view: String,
    /// The app's mutable store.
    mutable_store: M,
    /// The app's translations manager, expressed as a function yielding a `Future`. This is only ever needed on the server-side, and can't be set up properly on the client-side because
    /// we can't use futures in the app initialization in Wasm.
    translations_manager: Tm<T>,
}

// The usual implementation in which the default mutable store is used
// We don't need to have a similar one for the default translations manager because things are completely generic there
impl<G: Html, T: TranslationsManager> PerseusAppBase<G, FsMutableStore, T> {
    /// Creates a new instance of a Perseus app using the default filesystem-based mutable store. For most apps, this will be sufficient. Note that this initializes the translations manager
    /// as a dummy, and adds no templates or error pages.
    ///
    /// In development, you can get away with defining no error pages, but production apps (e.g. those created with `perseus deploy`) MUST set their own custom error pages.
    ///
    /// This is asynchronous because it creates a translations manager in the background.
    // It makes no sense to implement `Default` on this, so we silence Clippy deliberately
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::new_with_mutable_store(FsMutableStore::new("./dist/mutable".to_string()))
    }
}
// If one's using the default translations manager, caching should be handled automatically for them
impl<G: Html, M: MutableStore> PerseusAppBase<G, M, FsTranslationsManager> {
    /// The same as `.locales_and_translations_manager()`, but this accepts a literal `Locales` `struct`, which means this can be used when you're using `FsTranslationsManager` but when you don't
    /// know if your app is using i18n or not (almost always middleware).
    pub fn locales_lit_and_translations_manager(mut self, locales: Locales) -> Self {
        let using_i18n = locales.using_i18n;
        self.locales = locales;
        // If we're using i18n, do caching stuff
        // If not, use a dummy translations manager
        if using_i18n {
            // By default, all translations are cached
            let all_locales: Vec<String> = self
                .locales
                .get_all()
                .iter()
                // We have a `&&String` at this point, hence the double clone
                .cloned()
                .cloned()
                .collect();
            let tm_fut = FsTranslationsManager::new(
                crate::internal::i18n::DFLT_TRANSLATIONS_DIR.to_string(),
                all_locales,
                crate::internal::i18n::TRANSLATOR_FILE_EXT.to_string(),
            );
            self.translations_manager = Tm::Full(Box::pin(tm_fut));
        } else {
            self.translations_manager = Tm::Dummy(FsTranslationsManager::new_dummy());
        }

        self
    }
    /// Sets the internationalization information for an app using the default translations manager (`FsTranslationsManager`). This handles locale caching and the like automatically for you,
    /// though you could alternatively use `.locales()` and `.translations_manager()` independently to customize various behaviors. This takes the same arguments as `.locales()`, so
    /// the first argument is the default locale (used as a fallback for users with no locale preferences set in their browsers), and the second is a list of other locales supported.
    ///
    /// If you're not using i18n, you don't need to call this function. If you for some reason do have to though (e.g. overriding some other preferences in middleware), use `.disable_i18n()`,
    /// not this, as you're very likely to shoot yourself in the foot! (If i18n is disabled, the default locale MUST be set to `xx-XX`, for example.)
    pub fn locales_and_translations_manager(self, default: &str, other: &[&str]) -> Self {
        let locales = Locales {
            default: default.to_string(),
            other: other.iter().map(|s| s.to_string()).collect(),
            using_i18n: true,
        };

        self.locales_lit_and_translations_manager(locales)
    }
}
// The base implementation, generic over the mutable store and translations manager
impl<G: Html, M: MutableStore, T: TranslationsManager> PerseusAppBase<G, M, T> {
    /// Creates a new instance of a Perseus app, with the default options and a custom mutable store.
    pub fn new_with_mutable_store(mutable_store: M) -> Self {
        Self {
            root: "root".to_string(),
            // We do initialize with no templates, because an app without templates is in theory possible (and it's more convenient to call `.template()` for each one)
            template_getters: TemplateGetters(Vec::new()),
            // We do offer default error pages, but they'll panic if they're called for production building
            error_pages: ErrorPagesGetter(Box::new(ErrorPages::default)),
            global_state_creator: GlobalStateCreator::default(),
            // By default, we'll disable i18n (as much as I may want more websites to support more languages...)
            locales: Locales {
                default: "xx-XX".to_string(),
                other: Vec::new(),
                using_i18n: false,
            },
            // By default, we won't serve any static content outside the `static/` directory
            static_aliases: HashMap::new(),
            // By default, we won't use any plugins
            plugins: Rc::new(Plugins::new()),
            // This is relative to `.perseus/`
            immutable_store: ImmutableStore::new("./dist".to_string()),
            mutable_store,
            translations_manager: Tm::Dummy(T::new_dummy()),
            // Many users won't need anything fancy in the index view, so we provide a default
            index_view: DFLT_INDEX_VIEW.to_string(),
        }
    }

    // Setters (these all consume `self`)
    /// Sets the HTML ID of the `<div>` element at which to insert Perseus.
    pub fn root(mut self, val: &str) -> Self {
        self.root = val.to_string();
        self
    }
    /// Sets all the app's templates. This takes a vector of boxed functions that return templates.
    pub fn templates(mut self, val: Vec<Box<dyn Fn() -> Template<G>>>) -> Self {
        self.template_getters.0 = val;
        self
    }
    /// Adds a single new template to the app (convenience function). This takes a function that returns a template.
    pub fn template(mut self, val: impl Fn() -> Template<G> + 'static) -> Self {
        self.template_getters.0.push(Box::new(val));
        self
    }
    /// Sets the app's error pages.
    pub fn error_pages(mut self, val: impl Fn() -> ErrorPages<G> + 'static) -> Self {
        self.error_pages = ErrorPagesGetter(Box::new(val));
        self
    }
    /// Sets the app's global state creator.
    pub fn global_state_creator(mut self, val: GlobalStateCreator) -> Self {
        self.global_state_creator = val;
        self
    }
    /// Sets the locales information for the app. The first argument is the default locale (used as a fallback for users with no locale preferences set in their browsers), and
    /// the second is a list of other locales supported.
    ///
    /// Note that this does not update the translations manager, which must be done separately (if you're using `FsTranslationsManager`, the default, you can use
    /// `.locales_and_translations_manager()` to set both at once).
    ///
    /// If you're not using i18n, you don't need to call this function. If you for some reason do have to though (e.g. overriding some other preferences in middleware), use `.disable_i18n()`,
    /// not this, as you're very likely to shoot yourself in the foot! (If i18n is disabled, the default locale MUST be set to `xx-XX`, for example.)
    pub fn locales(mut self, default: &str, other: &[&str]) -> Self {
        self.locales = Locales {
            default: default.to_string(),
            other: other.iter().map(|s| s.to_string()).collect(),
            using_i18n: true,
        };
        self
    }
    /// Sets the locales information directly based on an instance of `Locales`. Usually, end users will use `.locales()` instead for a friendlier interface.
    pub fn locales_lit(mut self, val: Locales) -> Self {
        self.locales = val;
        self
    }
    /// Sets the translations manager. If you're using the default translations manager (`FsTranslationsManager`), you can use `.locales_and_translations_manager()` to set this automatically
    /// based on the locales information. This takes a `Future<Output = T>`, where `T` is your translations manager's type.
    ///
    /// The reason that this takes a `Future` is to avoid the use of `.await` in your app definition code, which must be synchronous due to constraints of Perseus' client-side systems.
    /// When your code is run on the server, the `Future` will be `.await`ed on, but on Wasm, it will be discarded and ignored, since the translations manager isn't needed in Wasm.
    ///
    /// This is generally intended for use with custom translations manager or specific use-cases with the default (mostly to do with custom caching behavior).
    pub fn translations_manager(mut self, val: impl Future<Output = T> + 'static) -> Self {
        self.translations_manager = Tm::Full(Box::pin(val));
        self
    }
    /// Explicitly disables internationalization. You shouldn't ever need to call this, as it's the default, but you may want to if you're writing middleware that doesn't support i18n.
    pub fn disable_i18n(mut self) -> Self {
        self.locales = Locales {
            default: "xx-XX".to_string(),
            other: Vec::new(),
            using_i18n: false,
        };
        // All translations manager must implement this function, which is designed for this exact purpose
        self.translations_manager = Tm::Dummy(T::new_dummy());
        self
    }
    /// Sets all the app's static aliases. This takes a map of URLs (e.g. `/file`) to resource paths, relative to the project directory (e.g. `style.css`).
    pub fn static_aliases(mut self, val: HashMap<String, String>) -> Self {
        self.static_aliases = val;
        self
    }
    /// Adds a single static alias (convenience function). This takes a URL path (e.g. `/file`) followed by a path to a resource (which must be within the project directory, e.g. `style.css`).
    pub fn static_alias(mut self, url: &str, resource: &str) -> Self {
        // We don't elaborate the alias to an actual filesystem path until the getter
        self.static_aliases
            .insert(url.to_string(), resource.to_string());
        self
    }
    /// Sets the plugins that the app will use.
    pub fn plugins(mut self, val: Plugins<G>) -> Self {
        self.plugins = Rc::new(val);
        self
    }
    /// Sets the mutable store for the app to use, which you would change for some production server environments if you wanted to store build artifacts that can change at runtime in a
    /// place other than on the filesystem (created for serverless functions specifically).
    pub fn mutable_store(mut self, val: M) -> Self {
        self.mutable_store = val;
        self
    }
    /// Sets the immutable store for the app to use. You should almost never need to change this unless you're not working with the CLI.
    pub fn immutable_store(mut self, val: ImmutableStore) -> Self {
        self.immutable_store = val;
        self
    }
    /// Sets the index view as a string. This should be used if you're using an `index.html` file or the like.
    ///
    /// Note: if possible, you should switch to using `.index_view()`, which uses a Sycamore view rather than an HTML string.
    pub fn index_view_str(mut self, val: &str) -> Self {
        self.index_view = val.to_string();
        self
    }
    /// Sets the index view using a Sycamore view, which avoids the need to write any HTML by hand whatsoever. Note that this must contain a `<head>` and `<body>` at a minimum.
    ///
    /// Warning: this view can't be reactive (yet). It will be rendered to a static string, which won't be hydrated.
    // The lifetime of the provided function doesn't need to be static, because we render using it and then we're done with it
    pub fn index_view<'a>(mut self, f: impl Fn() -> View<SsrNode> + 'a) -> Self {
        let html_str = sycamore::render_to_string(f);
        self.index_view = html_str;
        self
    }
    // Setters
    /// Gets the HTML ID of the `<div>` at which to insert Perseus.
    pub fn get_root(&self) -> String {
        self.plugins
            .control_actions
            .settings_actions
            .set_app_root
            .run((), self.plugins.get_plugin_data())
            .unwrap_or_else(|| self.root.to_string())
    }
    /// Gets the index view as a string, without generating an HTML shell (pass this into `::get_html_shell()` to do that).
    ///
    /// Note that this automatically adds `<!DOCTYPE html>` to the start of the HTMl shell produced, which can only be overriden with a control plugin (though you should really never do this
    /// in Perseus, which targets HTML on the web).
    #[cfg(feature = "server-side")]
    pub fn get_index_view_str(&self) -> String {
        // We have to add an HTML document type declaration, otherwise the browser could think it's literally anything! (This shouldn't be a problem, but it could be in 100 years...)
        format!("<!DOCTYPE html>\n{}", self.index_view)
    }
    /// Gets an HTML shell from an index view string. This is broken out so that it can be executed after the app has been built (which requries getting the translations manager, consuming
    /// `self`). As inconvenient as this is, it's necessitated, otherwise exporting would try to access the built app before it had actually been built.
    #[cfg(feature = "server-side")]
    pub async fn get_html_shell(
        index_view_str: String,
        root: &str,
        immutable_store: &ImmutableStore,
        plugins: &Plugins<G>,
    ) -> HtmlShell {
        // Construct an HTML shell
        let mut html_shell = HtmlShell::new(
            index_view_str,
            root,
            // TODO Handle this properly (good enough for now because that's what we weere already doing)
            &get_render_cfg(immutable_store)
                .await
                .expect("Couldn't get render configuration!"),
            &get_path_prefix_server(),
        );

        // Apply the myriad plugin actions to the HTML shell (replacing the whole thing first if need be)
        let shell_str = plugins
            .control_actions
            .settings_actions
            .html_shell_actions
            .set_shell
            .run((), plugins.get_plugin_data())
            .unwrap_or(html_shell.shell);
        html_shell.shell = shell_str;
        // For convenience, we alias the HTML shell functional actions
        let hsf_actions = &plugins
            .functional_actions
            .settings_actions
            .html_shell_actions;

        // These all return `Vec<String>`, so the code is almost identical for all the places for flexible interpolation
        html_shell.head_before_boundary.push(
            hsf_actions
                .add_to_head_before_boundary
                .run((), plugins.get_plugin_data())
                .values()
                .flatten()
                .cloned()
                .collect(),
        );
        html_shell.scripts_before_boundary.push(
            hsf_actions
                .add_to_scripts_before_boundary
                .run((), plugins.get_plugin_data())
                .values()
                .flatten()
                .cloned()
                .collect(),
        );
        html_shell.head_after_boundary.push(
            hsf_actions
                .add_to_head_after_boundary
                .run((), plugins.get_plugin_data())
                .values()
                .flatten()
                .cloned()
                .collect(),
        );
        html_shell.scripts_after_boundary.push(
            hsf_actions
                .add_to_scripts_after_boundary
                .run((), plugins.get_plugin_data())
                .values()
                .flatten()
                .cloned()
                .collect(),
        );
        html_shell.before_content.push(
            hsf_actions
                .add_to_before_content
                .run((), plugins.get_plugin_data())
                .values()
                .flatten()
                .cloned()
                .collect(),
        );
        html_shell.after_content.push(
            hsf_actions
                .add_to_after_content
                .run((), plugins.get_plugin_data())
                .values()
                .flatten()
                .cloned()
                .collect(),
        );

        html_shell
    }
    /// Gets the templates in an `Rc`-based `HashMap` for non-concurrent access.
    pub fn get_templates_map(&self) -> TemplateMap<G> {
        let mut map = HashMap::new();

        // Now add the templates the user provided
        for template_getter in self.template_getters.0.iter() {
            let template = template_getter();
            map.insert(template.get_path(), Rc::new(template));
        }

        // This will return a map of plugin name to a vector of templates to add
        let extra_templates = self
            .plugins
            .functional_actions
            .settings_actions
            .add_templates
            .run((), self.plugins.get_plugin_data());
        for (_plugin_name, plugin_templates) in extra_templates {
            // Turn that vector into a template map by extracting the template root paths as keys
            for template in plugin_templates {
                map.insert(template.get_path(), Rc::new(template));
            }
        }

        map
    }
    /// Gets the templates in an `Arc`-based `HashMap` for concurrent access. This should only be relevant on the server-side.
    #[cfg(feature = "server-side")]
    pub fn get_atomic_templates_map(&self) -> crate::templates::ArcTemplateMap<G> {
        let mut map = HashMap::new();

        // Now add the templates the user provided
        for template_getter in self.template_getters.0.iter() {
            let template = template_getter();
            map.insert(template.get_path(), std::sync::Arc::new(template));
        }

        // This will return a map of plugin name to a vector of templates to add
        let extra_templates = self
            .plugins
            .functional_actions
            .settings_actions
            .add_templates
            .run((), self.plugins.get_plugin_data());
        for (_plugin_name, plugin_templates) in extra_templates {
            // Turn that vector into a template map by extracting the template root paths as keys
            for template in plugin_templates {
                map.insert(template.get_path(), std::sync::Arc::new(template));
            }
        }

        map
    }
    /// Gets the error pages used in the app. This returns an `Rc`.
    pub fn get_error_pages(&self) -> ErrorPages<G> {
        let mut error_pages = (self.error_pages.0)();
        let extra_error_pages = self
            .plugins
            .functional_actions
            .settings_actions
            .add_error_pages
            .run((), self.plugins.get_plugin_data());
        for (_plugin_name, plugin_error_pages) in extra_error_pages {
            for (status, error_page) in plugin_error_pages {
                error_pages.add_page_rc(status, error_page);
            }
        }

        error_pages
    }
    /// Gets the global state creator. This can't be directly modified by plugins because of reactive type complexities.
    pub fn get_global_state_creator(&self) -> GlobalStateCreator {
        self.global_state_creator.clone()
    }
    /// Gets the locales information.
    pub fn get_locales(&self) -> Locales {
        let locales = self.locales.clone();
        self.plugins
            .control_actions
            .settings_actions
            .set_locales
            .run(locales.clone(), self.plugins.get_plugin_data())
            .unwrap_or(locales)
    }
    /// Gets the server-side translations manager. Like the mutable store, this can't be modified by plugins due to trait complexities.
    ///
    /// This involves evaluating the future stored for the translations manager, and so this consumes `self`.
    #[cfg(feature = "server-side")]
    pub async fn get_translations_manager(self) -> T {
        match self.translations_manager {
            Tm::Dummy(tm) => tm,
            Tm::Full(tm) => tm.await,
        }
    }
    /// Gets the immutable store.
    #[cfg(feature = "server-side")]
    pub fn get_immutable_store(&self) -> ImmutableStore {
        let immutable_store = self.immutable_store.clone();
        self.plugins
            .control_actions
            .settings_actions
            .set_immutable_store
            .run(immutable_store.clone(), self.plugins.get_plugin_data())
            .unwrap_or(immutable_store)
    }
    /// Gets the mutable store. This can't be modified by plugins due to trait complexities, so plugins should instead expose a function that the user can use to manually set it.
    #[cfg(feature = "server-side")]
    pub fn get_mutable_store(&self) -> M {
        self.mutable_store.clone()
    }
    /// Gets the plugins registered for the app. These are passed around and used in a way that doesn't require them to be concurrently accessible, and so are provided in an `Rc`.
    pub fn get_plugins(&self) -> Rc<Plugins<G>> {
        self.plugins.clone()
    }
    /// Gets the static aliases. This will check all provided resource paths to ensure they don't reference files outside the project directory, due to potential security risks in production
    /// (we don't want to accidentally serve an arbitrary in a production environment where a path may point to somewhere evil, like an alias to `/etc/passwd`).
    #[cfg(feature = "server-side")]
    pub fn get_static_aliases(&self) -> HashMap<String, String> {
        let mut static_aliases = self.static_aliases.clone();
        // This will return a map of plugin name to another map of static aliases that that plugin produced
        let extra_static_aliases = self
            .plugins
            .functional_actions
            .settings_actions
            .add_static_aliases
            .run((), self.plugins.get_plugin_data());
        for (_plugin_name, aliases) in extra_static_aliases {
            let new_aliases: HashMap<String, String> = aliases
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            static_aliases.extend(new_aliases);
        }

        let mut scoped_static_aliases = HashMap::new();
        for (url, path) in static_aliases {
            // We need to move this from being scoped to the app to being scoped for `.perseus/`
            // TODO Make sure this works properly on Windows (seems to..)
            let new_path = if path.starts_with('/') {
                // Absolute paths are a security risk and are disallowed
                // The reason for this is that they could point somewhere completely different on a production server (like an alias to `/etc/passwd`)
                // Allowing these would also inevitably cause head-scratching in production, it's much easier to disallow these
                panic!(
                    "it's a security risk to include absolute paths in `static_aliases` ('{}'), please make this relative to the project directory",
                    path
                );
            } else if path.starts_with("../") {
                // Anything outside this directory is a security risk as well
                panic!("it's a security risk to include paths outside the current directory in `static_aliases` ('{}')", path);
            } else if path.starts_with("./") {
                // `./` -> `../` (moving to execution from `.perseus/`)
                // But if we're operating standalone, it stays the same
                if cfg!(feature = "standalone") {
                    path.to_string()
                } else {
                    format!(".{}", path)
                }
            } else {
                // Anything else gets a `../` prepended
                // But if we're operating standalone, it stays the same
                if cfg!(feature = "standalone") {
                    path.to_string()
                } else {
                    format!("../{}", path)
                }
            };

            scoped_static_aliases.insert(url, new_path);
        }

        scoped_static_aliases
    }
}

/// The component that represents the entrypoint at which Perseus will inject itself. You can use this with the `.index_view()` method of `PerseusApp` to avoid having to create the entrypoint
/// `<div>` manually.
#[component(PerseusRoot<G>)]
pub fn perseus_root() -> View<G> {
    view! {
        div(dangerously_set_inner_html = "<div id=\"root\"></div>")
    }
}

/// An alias for the usual kind of Perseus app, which uses the filesystem-based mutable store and translations manager.
pub type PerseusApp<G> = PerseusAppBase<G, FsMutableStore, FsTranslationsManager>;
/// An alias for a Perseus app that uses a custom mutable store type.
pub type PerseusAppWithMutableStore<G, M> = PerseusAppBase<G, M, FsTranslationsManager>;
/// An alias for a Perseus app that uses a custom translations manager type.
pub type PerseusAppWithTranslationsManager<G, T> = PerseusAppBase<G, FsMutableStore, T>;
/// An alias for a fully customizable Perseus app that can accept a custom mutable store and a custom translations manager. Alternatively, you could just use `PerseusAppBase` directly.
pub type PerseusAppWithMutableStoreAndTranslationsManager<G, M, T> = PerseusAppBase<G, M, T>;
