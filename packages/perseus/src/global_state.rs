use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A container for global state in Perseus. This is designed as a context store, in which one of each type can be stored. Therefore, it acts very similarly to Sycamore's context system,
/// though it's specifically designed for each template to store one reactive properties object. In theory, you could interact with this entirely independently of Perseus' state interface,
/// though this isn't recommended.
///
/// For now, `struct`s stored in global state should have their reactivity managed by the inserter (usually the Perseus interface). However, this will change radically when Sycamore's
/// proposals for fine-grained reactivity are stabilized.
#[derive(Default, Clone)]
pub struct GlobalState {
    /// A map of type IDs to anything, allowing one storage of each type (each type is intended to a properties `struct` for a template). Entries must be `Clone`able becasue we assume them
    /// to be `Signal`s or `struct`s composed of `Signal`s.
    // Technically, this should be `Any + Clone`, but that's not possible without something like `dyn_clone`, and we don't need it because we can restrict on the methods instead!
    map: Rc<RefCell<HashMap<TypeId, Box<dyn Any>>>>,
}
impl GlobalState {
    /// Gets an element out of the state by its type.
    pub fn get<T: Any + Clone>(&self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let map = self.map.borrow();
        map.get(&type_id).map(|val| {
            if let Some(val) = val.downcast_ref::<T>() {
                (*val).clone()
            } else {
                // We extracted it by its type ID, it certainly should be able to downcast to that same type ID!
                unreachable!()
            }
        })
    }
    /// Adds a new element to the state by its type. Any existing element with the same type will be silently overriden (use `.contains()` to check first if needed).
    pub fn add<T: Any + Clone>(&mut self, val: T) {
        let type_id = TypeId::of::<T>();
        let mut map = self.map.borrow_mut();
        map.insert(type_id, Box::new(val));
    }
    /// Checks if the state contains the element of the given type.
    pub fn contains<T: Any + Clone>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.map.borrow().contains_key(&type_id)
    }
}

// These are tests for the `#[make_rx]` proc macro (here temporarily)
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[test]
    fn named_fields() {
        #[perseus_macro::make_rx(TestRx)]
        struct Test {
            foo: String,
            bar: u16,
        }

        let new = Test {
            foo: "foo".to_string(),
            bar: 5,
        }
        .make_rx();
        new.bar.set(6);
    }

    #[test]
    fn nested() {
        #[perseus_macro::make_rx(TestRx)]
        // `Serialize`, `Deserialize`, and `Clone` are automatically derived
        #[rx::nested("nested", NestedRx)]
        struct Test {
            #[serde(rename = "foo_test")]
            foo: String,
            bar: u16,
            // This will get simple reactivity
            // This annotation is unnecessary though
            baz: Baz,
            // This will get fine-grained reactivity
            nested: Nested,
        }
        #[derive(Serialize, Deserialize, Clone)]
        struct Baz {
            test: String,
        }
        #[perseus_macro::make_rx(NestedRx)]
        struct Nested {
            test: String,
        }

        let new = Test {
            foo: "foo".to_string(),
            bar: 5,
            baz: Baz {
                // We won't be able to `.set()` this
                test: "test".to_string(),
            },
            nested: Nested {
                // We will be able to `.set()` this
                test: "nested".to_string(),
            },
        }
        .make_rx();
        new.bar.set(6);
        new.baz.set(Baz {
            test: "updated".to_string(),
        });
        new.nested.test.set("updated".to_string());
        let _ = new.clone();
    }
}
