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
#[derive(Default, Clone)]
pub struct PageStateStore {
    /// A map of type IDs to anything, allowing one storage of each type (each
    /// type is intended to a properties `struct` for a template). Entries must
    /// be `Clone`able because we assume them to be `Signal`s or `struct`s
    /// composed of `Signal`s.
    // Technically, this should be `Any + Clone`, but that's not possible without something like
    // `dyn_clone`, and we don't need it because we can restrict on the methods instead!
    map: Rc<RefCell<HashMap<String, Box<dyn AnyFreeze>>>>,
}
impl PageStateStore {
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
    pub fn add<T: AnyFreeze + Clone>(&self, url: &str, val: T) {
        let mut map = self.map.borrow_mut();
        map.insert(url.to_string(), Box::new(val));
    }
    /// Checks if the state contains an entry for the given URL.
    pub fn contains(&self, url: &str) -> bool {
        self.map.borrow().contains_key(url)
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
