use crate::state::AnyFreeze;
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
/// Note that the same pages in different locales will have different entries
/// here. If you need to store state for a page across locales, you should use
/// the global state system instead. For apps not using i18n, the page URL will
/// not include any locale.
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
    // Technically, this should be `Any + Clone`, but that's not possible without something like
    // `dyn_clone`, and we don't need it because we can restrict on the methods instead!
    map: Rc<RefCell<HashMap<String, PssEntry>>>,
    /// The order in which pages were submitted to the store. This is used to
    /// evict the state of old pages to prevent Perseus sites from becoming
    /// massive in the browser's memory and slowing the user's browser down.
    order: Rc<RefCell<Vec<String>>>,
    /// The maximum size of the store before pages are evicted, specified in
    /// terms of a number of pages. Note that this pays no attention to the
    /// size in memory of individual pages (which should be dropped manually
    /// if this is a concern).
    ///
    /// Note: whatever you set here will impact HSR.
    max_size: usize,
    /// A list of pages that will be kept in the store no matter what. This can
    /// be used to maintain the states of essential pages regardless of how
    /// much the user has travelled through the site. The *vast* majority of
    /// use-cases for this would be better fulfilled by using global state, and
    /// this API is *highly* likely to be misused! If at all possible, use
    /// global state!
    keep_list: Rc<RefCell<Vec<String>>>,
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
        }
    }
    /// Gets an element out of the state by its type and URL. If the element
    /// stored for the given URL doesn't match the provided type, `None` will be
    /// returned.
    ///
    /// This will NOT return any document metadata, if any exists.
    pub fn get_state<T: AnyFreeze + Clone>(&self, url: &str) -> Option<T> {
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
    pub fn get_head(&self, url: &str) -> Option<String> {
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
    /// entries of it in that list will be removed.
    ///
    /// If there's already an entry for the given URL that has been marked as
    /// not accepting state, this will return `false`, and the entry will
    /// not be added. This *must* be handled for correctness.
    #[must_use]
    pub fn add_state<T: AnyFreeze + Clone>(&self, url: &str, val: T) -> bool {
        let mut map = self.map.borrow_mut();
        // We want to modify any existing entries to avoid wiping out document metadata
        if let Some(entry) = map.get_mut(url) {
            if !entry.set_state(Box::new(val)) {
                return false;
            }
        } else {
            let mut new_entry = PssEntry::default();
            if !new_entry.set_state(Box::new(val)) {
                return false;
            }
            map.insert(url.to_string(), new_entry);
        }
        let mut order = self.order.borrow_mut();
        // If we haven't been told to keep this page, enter it in the order list so it
        // can be evicted later
        if !self.keep_list.borrow().iter().any(|x| x == url) {
            // Get rid of any previous mentions of this page in the order list
            order.retain(|stored_url| stored_url != url);
            order.push(url.to_string());
            // If we've used up the maximum size yet, we should get rid of the oldest pages
            if order.len() > self.max_size {
                // Because this is called on every addition, we can safely assume that it's only
                // one over
                let old_url = order.remove(0);
                map.remove(&old_url); // This will only occur for pages that
                                      // aren't in the keep list, since those
                                      // don't even appear in `order`
            }
        }
        // If we got to here, then there were no issues with not accepting state
        true
    }
    /// Adds document metadata to the entry in the store for the given URL,
    /// creating it if it doesn't exist.
    ///
    /// This will be added to the end of the `order` property, and any previous
    /// entries of it in that list will be removed.
    pub fn add_head(&self, url: &str, head: String) {
        let mut map = self.map.borrow_mut();
        // We want to modify any existing entries to avoid wiping out state
        if let Some(entry) = map.get_mut(url) {
            entry.set_head(head);
        } else {
            let mut new_entry = PssEntry::default();
            new_entry.set_head(head);
            map.insert(url.to_string(), new_entry);
        }
        let mut order = self.order.borrow_mut();
        // If we haven't been told to keep this page, enter it in the order list so it
        // can be evicted later
        if !self.keep_list.borrow().iter().any(|x| x == url) {
            // Get rid of any previous mentions of this page in the order list
            order.retain(|stored_url| stored_url != url);
            order.push(url.to_string());
            // If we've used up the maximum size yet, we should get rid of the oldest pages
            if order.len() > self.max_size {
                // Because this is called on every addition, we can safely assume that it's only
                // one over
                let old_url = order.remove(0);
                map.remove(&old_url); // This will only occur for pages that
                                      // aren't in the keep list, since those
                                      // don't even appear in `order`
            }
        }
    }
    /// Sets the given entry as not being able to take any state. Any future
    /// attempt to register state for it will lead to silent failures and/or
    /// panics.
    pub fn set_state_never(&self, url: &str) {
        let mut map = self.map.borrow_mut();
        // If there's no entry for this URl yet, we'll create it
        if let Some(entry) = map.get_mut(url) {
            entry.set_state_never();
        } else {
            let mut new_entry = PssEntry::default();
            new_entry.set_state_never();
            map.insert(url.to_string(), new_entry);
        }
    }
    /// Checks if the state contains an entry for the given URL.
    pub fn contains(&self, url: &str) -> PssContains {
        let map = self.map.borrow();
        let entry = match map.get(url) {
            Some(entry) => entry,
            None => return PssContains::None,
        };
        match entry.state {
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
        }
    }
    /// Force the store to keep a certain page. This will prevent it from being
    /// evicted from the store, regardless of how many other pages are
    /// entered after it.
    ///
    /// Warning: in the *vast* majority of cases, your use-case for this will be
    /// far better served by the global state system! (If you use this with
    /// mutable state, you are quite likely to shoot yourself in the foot.)
    pub fn force_keep(&self, url: &str) {
        let mut order = self.order.borrow_mut();
        // Get rid of any previous mentions of this page in the order list (which will
        // prevent this page from ever being evicted)
        order.retain(|stored_url| stored_url != url);
        let mut keep_list = self.keep_list.borrow_mut();
        keep_list.push(url.to_string());
    }
    /// Forcibly removes a page from the store. Generally, you should never need
    /// to use this function, but it's provided for completeness. This could
    /// be used for preventing a certain page from being frozen,
    /// if necessary. Note that calling this in development will cause HSR to
    /// not work (since it relies on the state freezing system).
    ///
    /// This returns the page's state, if it was found.
    pub fn force_remove(&self, url: &str) -> Option<PssEntry> {
        let mut order = self.order.borrow_mut();
        order.retain(|stored_url| stored_url != url);
        let mut map = self.map.borrow_mut();
        map.remove(url)
    }
}
impl PageStateStore {
    /// Freezes the component entries into a new `HashMap` of `String`s to avoid
    /// extra layers of deserialization. This does NOT include document
    /// metadata, which will be re-requested from the server. (There is no
    /// point in freezing that, since it can't be unique for the user's page
    /// interactions, as it's added directly as the server sends it.)
    // TODO Avoid literally cloning all the page states here if possible
    pub fn freeze_to_hash_map(&self) -> HashMap<String, String> {
        let map = self.map.borrow();
        let mut str_map = HashMap::new();
        for (k, entry) in map.iter() {
            // Only freeze the underlying state if there is any (we want to minimize space
            // usage)
            if let PssState::Some(state) = &entry.state {
                let v_str = state.freeze();
                str_map.insert(k.to_string(), v_str);
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
pub struct PssEntry {
    /// The page state, if any exists. This may come with a guarantee that no
    /// state will ever exist.
    state: PssState<Box<dyn AnyFreeze>>,
    /// The document metadata of the page, which can be cached to prevent future
    /// requests to the server.
    head: Option<String>,
}
impl Default for PssEntry {
    fn default() -> Self {
        Self {
            // There could be state later
            state: PssState::None,
            head: None,
        }
    }
}
impl PssEntry {
    /// Declare that this entry will *never* have state. This should be done by
    /// macros that definitively know the structure of a page. This action
    /// is irrevocable, since a page cannot transition from never taking state
    /// to taking some later in Perseus.
    ///
    /// Note that this will not be preserved in freezing (allowing greater
    /// flexibility of HSR).
    ///
    /// **Warning:** manually calling in the wrong context this may lead to the
    /// complete failure of your application!
    pub fn set_state_never(&mut self) {
        self.state = PssState::Never;
    }
    /// Adds document metadata to this entry.
    pub fn set_head(&mut self, head: String) {
        self.head = Some(head);
    }
    /// Adds state to this entry. This will return false and do nothing if the
    /// entry has been marked as never being able to accept state.
    #[must_use]
    pub fn set_state(&mut self, state: Box<dyn AnyFreeze>) -> bool {
        if let PssState::Never = self.state {
            false
        } else {
            self.state = PssState::Some(state);
            true
        }
    }
}

/// The page state of a PSS entry. This is used to determine whether or not we
/// need to request data from the server.
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
}
