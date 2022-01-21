use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A key type for the `PageStateStore` that denotes both a page's state type and its URL.
#[derive(Hash, PartialEq, Eq)]
pub struct PageStateKey {
    state_type: TypeId,
    url: String,
}

/// A container for page state in Perseus. This is designed as a context store, in which one of each type can be stored. Therefore, it acts very similarly to Sycamore's context system,
/// though it's specifically designed for each page to store one reactive properties object. In theory, you could interact with this entirely independently of Perseus' state interface,
/// though this isn't recommended.
///
/// Note that the same pages in different locales will have different entries here. If you need to store state for a page across locales, you should use the global state system instead. For apps
/// not using i18n, the page URL will not include any locale.
// TODO Make this work with multiple pages for a single template
#[derive(Default, Clone)]
pub struct PageStateStore {
    /// A map of type IDs to anything, allowing one storage of each type (each type is intended to a properties `struct` for a template). Entries must be `Clone`able becasue we assume them
    /// to be `Signal`s or `struct`s composed of `Signal`s.
    // Technically, this should be `Any + Clone`, but that's not possible without something like `dyn_clone`, and we don't need it because we can restrict on the methods instead!
    map: Rc<RefCell<HashMap<PageStateKey, Box<dyn Any>>>>,
}
impl PageStateStore {
    /// Gets an element out of the state by its type and URL.
    pub fn get<T: Any + Clone>(&self, url: &str) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let key = PageStateKey {
            state_type: type_id,
            url: url.to_string(),
        };
        let map = self.map.borrow();
        map.get(&key).map(|val| {
            if let Some(val) = val.downcast_ref::<T>() {
                (*val).clone()
            } else {
                // We extracted it by its type ID, it certainly should be able to downcast to that same type ID!
                unreachable!()
            }
        })
    }
    /// Adds a new element to the state by its type and URL. Any existing element with the same type and URL will be silently overriden (use `.contains()` to check first if needed).
    pub fn add<T: Any + Clone>(&mut self, url: &str, val: T) {
        let type_id = TypeId::of::<T>();
        let key = PageStateKey {
            state_type: type_id,
            url: url.to_string(),
        };
        let mut map = self.map.borrow_mut();
        map.insert(key, Box::new(val));
    }
    /// Checks if the state contains the element of the given type for the given page.
    pub fn contains<T: Any + Clone>(&self, url: &str) -> bool {
        let type_id = TypeId::of::<T>();
        let key = PageStateKey {
            state_type: type_id,
            url: url.to_string(),
        };
        self.map.borrow().contains_key(&key)
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
