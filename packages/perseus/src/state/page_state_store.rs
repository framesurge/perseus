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
    // Technically, this should be `Any + Clone`, but that's not possible without something like
    // `dyn_clone`, and we don't need it because we can restrict on the methods instead!
    map: Rc<RefCell<HashMap<String, Box<dyn AnyFreeze>>>>,
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
    pub fn get<T: AnyFreeze + Clone>(&self, url: &str) -> Option<T> {
        let map = self.map.borrow();
        map.get(url)
            .and_then(|val| val.as_any().downcast_ref::<T>().map(|val| (*val).clone()))
    }
    /// Adds a new element to the state by its URL. Any existing element with
    /// the same URL will be silently overriden (use `.contains()` to check
    /// first if needed).
    ///
    /// This will be added to the end of the `order` property, and any previous
    /// entries of it in that list will be removed.
    pub fn add<T: AnyFreeze + Clone>(&self, url: &str, val: T) {
        let mut map = self.map.borrow_mut();
        map.insert(url.to_string(), Box::new(val));
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
    /// Checks if the state contains an entry for the given URL.
    pub fn contains(&self, url: &str) -> bool {
        self.map.borrow().contains_key(url)
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
    pub fn force_remove(&self, url: &str) -> Option<Box<dyn AnyFreeze>> {
        let mut order = self.order.borrow_mut();
        order.retain(|stored_url| stored_url != url);
        let mut map = self.map.borrow_mut();
        map.remove(url)
    }
}
impl PageStateStore {
    /// Freezes the component entries into a new `HashMap` of `String`s to avoid
    /// extra layers of deserialization.
    // TODO Avoid literally cloning all the page states here if possible
    pub fn freeze_to_hash_map(&self) -> HashMap<String, String> {
        let map = self.map.borrow();
        let mut str_map = HashMap::new();
        for (k, v) in map.iter() {
            let v_str = v.freeze();
            str_map.insert(k.to_string(), v_str);
        }

        str_map
    }
}
impl std::fmt::Debug for PageStateStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageStateStore").finish()
    }
}

// TODO Use `trybuild` properly with all this
////  These are tests for the `#[make_rx]` proc macro (here temporarily)
// #[cfg(test)]
// mod tests {
//     use serde::{Deserialize, Serialize};
//     use crate::state::MakeRx; // We need this to manually use `.make_rx()`

//     #[test]
//     fn named_fields() {
//         #[perseus_macro::make_rx(TestRx)]
//         struct Test {
//             foo: String,
//             bar: u16,
//         }

//         let new = Test {
//             foo: "foo".to_string(),
//             bar: 5,
//         }
//         .make_rx();
//         new.bar.set(6);
//     }

//     #[test]
//     fn nested() {
//         #[perseus_macro::make_rx(TestRx)]
//         // `Serialize`, `Deserialize`, and `Clone` are automatically derived
//         #[rx::nested("nested", NestedRx)]
//         struct Test {
//             #[serde(rename = "foo_test")]
//             foo: String,
//             bar: u16,
//             // This will get simple reactivity
//             // This annotation is unnecessary though
//             baz: Baz,
//             // This will get fine-grained reactivity
//             nested: Nested,
//         }
//         #[derive(Serialize, Deserialize, Clone)]
//         struct Baz {
//             test: String,
//         }
//         #[perseus_macro::make_rx(NestedRx)]
//         struct Nested {
//             test: String,
//         }

//         let new = Test {
//             foo: "foo".to_string(),
//             bar: 5,
//             baz: Baz {
//                 // We won't be able to `.set()` this
//                 test: "test".to_string(),
//             },
//             nested: Nested {
//                 // We will be able to `.set()` this
//                 test: "nested".to_string(),
//             },
//         }
//         .make_rx();
//         new.bar.set(6);
//         new.baz.set(Baz {
//             test: "updated".to_string(),
//         });
//         new.nested.test.set("updated".to_string());
//         let _ = new.clone();
//     }
// }
