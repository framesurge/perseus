use crate::errors::{ClientError, ClientInvariantError};
use crate::page_data::PageDataPartial;
use crate::path::*;
use crate::state::AnyFreeze;
#[cfg(any(client, doc))]
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A container for page state in Perseus. This is designed as a context store,
/// in which one of each type can be stored. Therefore, it acts very similarly
/// to Sycamore's context system, though it's specifically designed for each
/// page to store one reactive properties object. In theory, you could interact
/// with this entirely independently of Perseus' state interface, though this
/// isn't recommended.
///
/// Paths in this store will have their locales prepended if the app uses i18n.
// WARNING: Never allow users to manually modify the internal maps/orderings of this,
// or the eviction protocols will become very confused!
#[derive(Clone)]
pub struct PageStateStore {
    /// A map of type IDs to anything, allowing one storage of each type (each
    /// type is intended to a properties `struct` for a template). Entries must
    /// be `Clone`able because we assume them to be `Signal`s or `struct`s
    /// composed of `Signal`s.
    ///
    /// This also stores the head string for each page, which means we don't
    /// need to re-request old pages from the server whatsoever, minimizing
    /// requests.
    ///
    /// Note that this stores both pages and capsules.
    // Technically, this should be `Any + Clone`, but that's not possible without something like
    // `dyn_clone`, and we don't need it because we can restrict on the methods instead!
    map: Rc<RefCell<HashMap<PathMaybeWithLocale, PssEntry>>>,
    /// The order in which pages were submitted to the store. This is used to
    /// evict the state of old pages to prevent Perseus sites from becoming
    /// massive in the browser's memory and slowing the user's browser down.
    ///
    /// This will *not* store capsules.
    order: Rc<RefCell<Vec<PathMaybeWithLocale>>>,
    /// The maximum size of the store before pages are evicted, specified in
    /// terms of a number of pages. Note that this pays no attention to the
    /// size in memory of individual pages (which should be dropped manually
    /// if this is a concern).
    ///
    /// This will only apply to the number of pages stored! If one page depends
    /// on 300 capsules, they will be completely ignored!
    ///
    /// Note: whatever you set here will impact HSR.
    max_size: usize,
    /// A list of pages that will be kept in the store no matter what. This can
    /// be used to maintain the states of essential pages regardless of how
    /// much the user has traveled through the site. The *vast* majority of
    /// use-cases for this would be better fulfilled by using global state, and
    /// this API is *highly* likely to be misused! If at all possible, use
    /// global state!
    // TODO Can widgets be specified here?
    keep_list: Rc<RefCell<Vec<PathMaybeWithLocale>>>,
    /// A list of pages/widgets whose data have been manually preloaded to
    /// minimize future network requests. This list is intended for pages
    /// that are to be globally preloaded; any pages that should only be
    /// preloaded for a specific route should be placed in `route_preloaded`
    /// instead.
    ///
    /// Note that this is used to store the 'preloaded' widgets from the server
    /// in initial loads, before the actual `Widget` components take them up
    /// for rendering.
    preloaded: Rc<RefCell<HashMap<PathMaybeWithLocale, PageDataPartial>>>,
    /// Pages/widgets that have been preloaded for the current route, which
    /// should be cleared on a route change. This is broken out to allow
    /// future preloading based on heuristics for a given page, which should
    /// be dumped if none of the pages are actually used.
    route_preloaded: Rc<RefCell<HashMap<PathMaybeWithLocale, PageDataPartial>>>,
}
impl PageStateStore {
    /// Creates a new, empty page state store with the given maximum size. After
    /// this number of pages have been entered, the oldest ones will have
    /// their states eliminated. Note that individual pages can be
    /// marked for keeping or can be manually removed to circumvent these
    /// mechanisms.
    pub fn new(max_size: usize) -> Self {
        Self {
            map: Rc::default(),
            order: Rc::default(),
            max_size,
            keep_list: Rc::default(),
            preloaded: Rc::default(),
            route_preloaded: Rc::default(),
        }
    }
    /// Gets an element out of the state by its type and URL. If the element
    /// stored for the given URL doesn't match the provided type, `None` will be
    /// returned.
    ///
    /// This will NOT return any document metadata, if any exists.
    pub fn get_state<T: AnyFreeze + Clone>(&self, url: &PathMaybeWithLocale) -> Option<T> {
        let map = self.map.borrow();
        match map.get(url) {
            Some(entry) => {
                let state = match &entry.state {
                    PssState::Some(state) => state,
                    // We don't care whether there could be state in future, there isn't any right
                    // now
                    _ => return None,
                };
                state.as_any().downcast_ref::<T>().map(|val| (*val).clone())
            }
            None => None,
        }
    }
    /// Gets the document metadata registered for a URL, if it exists.
    pub fn get_head(&self, url: &PathMaybeWithLocale) -> Option<String> {
        let map = self.map.borrow();
        match map.get(url) {
            Some(entry) => entry.head.as_ref().map(|v| v.to_string()),
            None => None,
        }
    }
    /// Adds page state to the entry in the store with the given URL, creating
    /// it if it doesn't exist. Any state previously set for the item will
    /// be overridden, but any document metadata will be preserved.
    ///
    /// This will be added to the end of the `order` property, and any previous
    /// entries of it in that list will be removed, unless it is specified to
    /// be a widget.
    ///
    /// If there's already an entry for the given URL that has been marked as
    /// not accepting state, this will return an error, and the entry will
    /// not be added. When this is called for HSR purposes, this should be taken
    /// with a grain of salt, as documented on `.set_state()` for [`PssEntry`].
    pub fn add_state<T: AnyFreeze + Clone>(
        &self,
        url: &PathMaybeWithLocale,
        val: T,
        is_widget: bool,
    ) -> Result<(), ClientError> {
        let mut map = self.map.borrow_mut();
        // We want to modify any existing entries to avoid wiping out document metadata
        if let Some(entry) = map.get_mut(url) {
            entry.set_state(Box::new(val))?
        } else {
            let mut new_entry = PssEntry::default();
            new_entry.set_state(Box::new(val))?;
            map.insert(url.clone(), new_entry);
        }
        let mut order = self.order.borrow_mut();
        // If we haven't been told to keep this page, enter it in the order list so it
        // can be evicted later (unless it's a widget)
        if !self.keep_list.borrow().iter().any(|x| x == url) && !is_widget {
            // Get rid of any previous mentions of this page in the order list
            order.retain(|stored_url| stored_url != url);
            order.push(url.clone());
            // If we've used up the maximum size yet, we should get rid of the oldest pages
            drop(order);
            drop(map);
            self.evict_page_if_needed();
        }

        Ok(())
    }
    /// Adds document metadata to the entry in the store for the given URL,
    /// creating it if it doesn't exist.
    ///
    /// This will be added to the end of the `order` property, and any previous
    /// entries of it in that list will be removed.
    ///
    /// This will accept widgets adding empty heads, since they do still need
    /// to be registered.
    pub fn add_head(&self, url: &PathMaybeWithLocale, head: String, is_widget: bool) {
        let mut map = self.map.borrow_mut();
        // We want to modify any existing entries to avoid wiping out state
        if let Some(entry) = map.get_mut(url) {
            entry.set_head(head);
        } else {
            let mut new_entry = PssEntry::default();
            new_entry.set_head(head);
            map.insert(url.clone(), new_entry);
        }
        let mut order = self.order.borrow_mut();
        // If we haven't been told to keep this page, enter it in the order list so it
        // can be evicted later (unless it's a widget)
        if !self.keep_list.borrow().iter().any(|x| x == url) && !is_widget {
            // Get rid of any previous mentions of this page in the order list
            order.retain(|stored_url| stored_url != url);
            order.push(url.clone());
            // If we've used up the maximum size yet, we should get rid of the oldest pages
            drop(order);
            drop(map);
            self.evict_page_if_needed();
        }
    }
    /// Sets the given entry as not being able to take any state. Any future
    /// attempt to register state for it will lead to silent failures and/or
    /// panics.
    pub fn set_state_never(&self, url: &PathMaybeWithLocale, is_widget: bool) {
        let mut map = self.map.borrow_mut();
        // If there's no entry for this URl yet, we'll create it
        if let Some(entry) = map.get_mut(url) {
            entry.set_state_never();
        } else {
            let mut new_entry = PssEntry::default();
            new_entry.set_state_never();
            map.insert(url.clone(), new_entry);
        }
        let mut order = self.order.borrow_mut();
        // If we haven't been told to keep this page, enter it in the order list so it
        // can be evicted later (unless it's a widget)
        if !self.keep_list.borrow().iter().any(|x| x == url) && !is_widget {
            // Get rid of any previous mentions of this page in the order list
            order.retain(|stored_url| stored_url != url);
            order.push(url.clone());
            // If we've used up the maximum size yet, we should get rid of the oldest pages
            drop(order);
            drop(map);
            self.evict_page_if_needed();
        }
    }
    /// Checks if the state contains an entry for the given URL.
    pub fn contains(&self, url: &PathMaybeWithLocale) -> PssContains {
        let map = self.map.borrow();
        let contains = match map.get(url) {
            Some(entry) => match entry.state {
                PssState::Some(_) => match entry.head {
                    Some(_) => PssContains::All,
                    None => PssContains::State,
                },
                PssState::None => match entry.head {
                    Some(_) => PssContains::Head,
                    None => PssContains::None,
                },
                PssState::Never => match entry.head {
                    Some(_) => PssContains::HeadNoState,
                    None => PssContains::None,
                },
            },
            None => PssContains::None,
        };
        // Now do a final check to make sure it hasn't been preloaded (which would show
        // up as `PssContains::None` after that)
        match contains {
            PssContains::None | PssContains::Head | PssContains::State => {
                let preloaded = self.preloaded.borrow();
                let route_preloaded = self.route_preloaded.borrow();
                // We don't currently distinguish between *how* the page has been preloaded
                if route_preloaded.contains_key(url) || preloaded.contains_key(url) {
                    PssContains::Preloaded
                } else {
                    contains
                }
            }
            _ => contains,
        }
    }
    /// Declares that a certain page/widget depends on a certain widget. This
    /// will added as a bidirectional relation that can be used to control
    /// when the widget will be evicted from the state store (which should
    /// only happen after all the pages using it have also been evicted).
    /// Failure to declare a widget here is not a *critical* error, but it
    /// will lead to a seriously suboptimal user experience.
    ///
    /// # Panics
    /// This function will panic if the given page and widget paths are not
    /// already registered in the state store.
    #[cfg(any(client, doc))]
    pub(crate) fn declare_dependency(
        &self,
        widget_path: &PathMaybeWithLocale,
        caller_path: &PathMaybeWithLocale,
    ) {
        let mut map = self.map.borrow_mut();
        {
            let caller = map.get_mut(caller_path).expect("page/widget that was part of dependency declaration was not present in the state store");
            caller.add_dependency(widget_path.clone());
        }

        {
            let widget = map.get_mut(widget_path).expect(
                "widget that was part of dependency declaration was not present in the state store",
            );
            widget.add_dependent(caller_path.clone());
        }
    }
    /// Preloads the given URL from the server and adds it to the PSS. This
    /// expects a path that does *not* contain the present locale, as the
    /// locale is provided separately. The two are concatenated
    /// appropriately for locale-specific preloading in apps that use it.
    ///
    /// This function has no effect on the server-side.
    ///
    /// Note that this should generally be called through `Reactor`, to avoid
    /// having to manually collect the required arguments.
    // Note that this function bears striking resemblance to the subsequent load system!
    #[cfg(any(client, doc))]
    pub(crate) async fn preload(
        &self,
        path: &PathWithoutLocale,
        locale: &str,
        template_path: &str,
        was_incremental_match: bool,
        is_route_preload: bool,
        // This changes a few flag types, like `AssetType`
        is_widget: bool,
    ) -> Result<(), crate::errors::ClientError> {
        use crate::{
            errors::{AssetType, ClientPreloadError, FetchError},
            utils::{fetch, get_path_prefix_client},
        };

        let asset_ty = if is_widget {
            AssetType::Widget
        } else {
            AssetType::Preload
        };

        let full_path = PathMaybeWithLocale::new(path, locale);

        // If we already have the page loaded fully in the PSS, abort immediately
        if let PssContains::All | PssContains::HeadNoState | PssContains::Preloaded =
            self.contains(&full_path)
        {
            return Ok(());
        }

        // If we're getting data about the index page, explicitly set it to that
        // This can be handled by the Perseus server (and is), but not by static
        // exporting
        // let path_norm = match path.is_empty() {
        //     true => "index".to_string(),
        //     false => path.to_string(),
        // };
        // Get the static page data (head and state)
        let asset_url = format!(
            "{}/.perseus/page/{}/{}.json?entity_name={}&was_incremental_match={}",
            get_path_prefix_client(),
            locale,
            **path,
            template_path,
            was_incremental_match
        );
        // If this doesn't exist, then it's a 404 (we went here by explicit instruction,
        // but it may be an unservable ISR page or the like)
        let page_data_str = fetch(&asset_url, asset_ty).await?;
        match page_data_str {
            Some(page_data_str) => {
                // All good, deserialize the page data
                let page_data =
                    serde_json::from_str::<PageDataPartial>(&page_data_str).map_err(|err| {
                        FetchError::SerFailed {
                            url: path.to_string(),
                            source: err.into(),
                            ty: asset_ty,
                        }
                    })?;
                let mut preloaded = if is_route_preload {
                    self.preloaded.borrow_mut()
                } else {
                    self.route_preloaded.borrow_mut()
                };
                preloaded.insert(full_path, page_data);
                Ok(())
            }
            None => Err(ClientPreloadError::PreloadNotFound {
                path: path.to_string(),
            }
            .into()),
        }
    }
    /// Adds the given widget to the preload list so it can be later accessed
    /// during the initial load render. This is not used for widgets in
    /// subsequently loaded pages, which are fetched separately.
    #[cfg(any(client, doc))]
    pub(crate) fn add_initial_widget(&self, url: PathMaybeWithLocale, state: Value) {
        let mut preloaded = self.preloaded.borrow_mut();
        // Widgets never have heads
        preloaded.insert(
            url,
            PageDataPartial {
                state,
                head: String::new(),
            },
        );
    }
    /// Gets a preloaded page. This will search both the globally and
    /// route-specifically preloaded pages.
    ///
    /// Note that this will delete the preloaded page from the preload cache,
    /// since it's expected to be parsed and rendered immediately. It should
    /// also have its head entered in the PSS.
    pub fn get_preloaded(&self, url: &PathMaybeWithLocale) -> Option<PageDataPartial> {
        let mut preloaded = self.preloaded.borrow_mut();
        let mut route_preloaded = self.route_preloaded.borrow_mut();
        if let Some(page_data) = preloaded.remove(url) {
            Some(page_data)
        } else {
            route_preloaded.remove(url)
        }
    }
    /// Clears all the routes that were preloaded for the last route, keeping
    /// only those listed (this should be used to make sure we don't have to
    /// double-preload things).
    pub fn cycle_route_preloaded(&self, keep_urls: &[&PathMaybeWithLocale]) {
        let mut preloaded = self.route_preloaded.borrow_mut();
        preloaded.retain(|url, _| keep_urls.iter().any(|keep_url| *keep_url == url));
    }
    /// Forces the store to keep a certain page. This will prevent it from being
    /// evicted from the store, regardless of how many other pages are
    /// entered after it.
    ///
    /// Warning: in the *vast* majority of cases, your use-case for this will be
    /// far better served by the global state system! (If you use this with
    /// mutable state, you are quite likely to shoot yourself in the foot.)
    pub fn force_keep(&self, url: &PathMaybeWithLocale) {
        let mut order = self.order.borrow_mut();
        // Get rid of any previous mentions of this page in the order list (which will
        // prevent this page from ever being evicted)
        order.retain(|stored_url| stored_url != url);
        let mut keep_list = self.keep_list.borrow_mut();
        keep_list.push(url.clone());
    }
    /// Forcibly removes a page from the store. Generally, you should never need
    /// to use this function, but it's provided for completeness. This could
    /// be used for preventing a certain page from being frozen,
    /// if necessary. Note that calling this in development will cause HSR to
    /// not work (since it relies on the state freezing system).
    ///
    /// This returns the page's state, if it was found.
    ///
    /// Note: this will safely preserve the invariants of the store (as opposed
    /// to manual removal).
    pub fn force_remove(&self, url: &PathMaybeWithLocale) -> Option<PssEntry> {
        let mut order = self.order.borrow_mut();
        order.retain(|stored_url| stored_url != url);
        let mut map = self.map.borrow_mut();
        map.remove(url)
    }
    /// Evicts the oldest page in the store if we've reached the order limit.
    /// This will also traverse the rest of the store to evict any widgets
    /// that were only used by that page.
    ///
    /// This assumes that any references to parts of the store have been
    /// dropped, as this will mutably interact with a number of them.
    ///
    /// Note that this will never affect paths in the keep list, since they
    /// don't actually appear in `self.order`.
    fn evict_page_if_needed(&self) {
        let mut order = self.order.borrow_mut();
        let mut map = self.map.borrow_mut();
        let keep_list = self.keep_list.borrow();
        if order.len() > self.max_size {
            // Because this is called on every addition, we can safely assume that it's only
            // one over
            let old_url = order.remove(0);
            // Assuming there's been no tampering, this will be fine
            let PssEntry { dependencies, .. } = map.remove(&old_url).unwrap();
            // We want to remove any widgets that this page used that no other page is using
            for dep in dependencies.into_iter() {
                // First, make sure this isn't in the keep list
                if !keep_list.contains(&dep) {
                    // Invariants say this will be present in both
                    let entry = map.get_mut(&dep).unwrap();
                    // We've evicted this page, so it's no longer a dependent
                    entry.dependents.retain(|v| v != &old_url);
                    if entry.dependents.is_empty() {
                        // Evict this widget
                        map.remove(&dep).unwrap();
                    }
                }
            }
        }
    }
}
impl PageStateStore {
    /// Freezes the component entries into a new `HashMap` of `String`s to avoid
    /// extra layers of deserialization. This does NOT include document
    /// metadata, which will be re-requested from the server. (There is no
    /// point in freezing that, since it can't be unique for the user's page
    /// interactions, as it's added directly as the server sends it.)
    ///
    /// Note that the typed path system uses transparent serialization, and has
    /// no additional storage cost.
    // TODO Avoid literally cloning all the page states here if possible
    pub fn freeze_to_hash_map(&self) -> HashMap<PathMaybeWithLocale, String> {
        let map = self.map.borrow();
        let mut str_map = HashMap::new();
        for (k, entry) in map.iter() {
            // Only freeze the underlying state if there is any (we want to minimize space
            // usage)
            if let PssState::Some(state) = &entry.state {
                let v_str = state.freeze();
                str_map.insert(k.clone(), v_str);
            }
        }

        str_map
    }
}
impl std::fmt::Debug for PageStateStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageStateStore").finish()
    }
}

/// An entry for a single page in the PSS. This type has no concern for the
/// actual type of the page state it stores.
///
/// Note: while it is hypothetically possible for this to hold neither a state
/// nor document metadata, that will never happen without user intervention.
#[derive(Debug)]
pub struct PssEntry {
    /// The page state, if any exists. This may come with a guarantee that no
    /// state will ever exist.
    state: PssState<Box<dyn AnyFreeze>>,
    /// The document metadata of the page, which can be cached to prevent future
    /// requests to the server.
    head: Option<String>,
    /// A list of widgets this page depends on, by their path. This allows quick
    /// indexing of the widgets that should potentially be evicted when the page
    /// using them is evicted. (Note that widgets are only evicted when all
    /// pages that depend on them have all been evicted.)
    ///
    /// As there is never a centralized list of the dependencies of any given
    /// page, this will be gradually filled out as the page is rendered.
    /// (This is why it is critical that pages are pure functions on the
    /// state they use with respect to the widgets on which they depend.)
    dependencies: Vec<PathMaybeWithLocale>,
    /// A list of dependents by path. For pages, this will always be empty.
    ///
    /// This is used by widgets to declare the pages that depend on them,
    /// creating the reverse of the `dependencies` path. This is used so we
    /// can quickly iterate through each of the widgets a page uses when
    /// we're about to evict it and remove only those that aren't being used
    /// by any other pages.
    dependents: Vec<PathMaybeWithLocale>,
}
impl Default for PssEntry {
    fn default() -> Self {
        Self {
            // There could be state later
            state: PssState::None,
            head: None,
            dependencies: Vec::new(),
            dependents: Vec::new(),
        }
    }
}
impl PssEntry {
    /// Declare that this entry will *never* have state. This should be done by
    /// macros that definitively know the structure of a page. This action
    /// is revocable under HSR conditions only.
    ///
    /// Note that this will not be preserved in freezing (allowing greater
    /// flexibility of HSR).
    pub fn set_state_never(&mut self) {
        self.state = PssState::Never;
    }
    /// Adds document metadata to this entry.
    pub fn set_head(&mut self, head: String) {
        self.head = Some(head);
    }
    /// Declares a widget that this page/widget depends on, by its path.
    #[cfg(any(client, doc))]
    fn add_dependency(&mut self, path: PathMaybeWithLocale) {
        self.dependencies.push(path);
    }
    /// Declares a page/widget that this widget is used by, by its path.
    #[cfg(any(client, doc))]
    fn add_dependent(&mut self, path: PathMaybeWithLocale) {
        self.dependents.push(path);
    }
    /// Adds state to this entry. This will return an error if this entry has
    /// previously been marked as having no state.
    ///
    /// If we're setting state for HSR, this function's should be interpreted
    /// with caution: if the user has added state to a template/capsule that
    /// previously didn't have state, then nothing in the code will try to
    /// set it to never having had state (and there will be nothing in the
    /// frozen state for it), which is fine; but, if they *removed* state
    /// from an entity that previously had it, this will return an error to the
    /// HSR thaw attempt (which will try to add the old state back). In that
    /// case, the error should be discarded by the caller, who should accept
    /// the changed data model.
    pub fn set_state(&mut self, state: Box<dyn AnyFreeze>) -> Result<(), ClientError> {
        if let PssState::Never = self.state {
            Err(ClientInvariantError::IllegalStateRegistration.into())
        } else {
            self.state = PssState::Some(state);
            Ok(())
        }
    }
}

/// The page state of a PSS entry. This is used to determine whether or not we
/// need to request data from the server.
#[derive(Debug)]
pub enum PssState<T> {
    /// There is state.
    Some(T),
    /// There is no state, but there could be some in future.
    None,
    /// There is no state, and there never will be any (i.e. this page does not
    /// use state).
    Never,
}

/// The various things the PSS can contain for a single page. It might have
/// state, a head, both, or neither.
///
/// *Note: the `P` in the `PSS` acronym used to stand for page (pre-widgets),
/// and it now stands for Perseus, as its removal creates a considerably less
/// desirable acronym.*
#[derive(Debug)]
pub enum PssContains {
    /// There is no entry for this page.
    None,
    /// There is page state only recorded for this page.
    State,
    /// There is only document metadata recorded for this page. There is no
    /// state recorded, but that doesn't mean the page has none.
    Head,
    /// There is document metadata recorded for this page, along with an
    /// assurance that there will never be any state.
    HeadNoState,
    /// Both document metadata and page state are present for this page.
    All,
    /// We have a [`PageDataPartial`] for the given page, since it was preloaded
    /// by some other function (likely the user's action). This will need proper
    /// processing into a state.
    Preloaded,
}
