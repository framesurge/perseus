// use std::any::{Any, TypeId};
// use std::collections::HashMap;
//
// /// A container for global state in Perseus. This is designed as a context store, in which one of each type can be stored. Therefore, it acts very similarly to Sycamore's context system,
// /// though it's specifically designed for each template to store one reactive properties object. In theory, you could interact with this entirely independently of Perseus' state interface,
// /// though this isn't recommended.
// ///
// /// For now, `struct`s stored in global state should have tehir reactivity managed by the inserter (usually the Perseus interface). However, this will change radically when Sycamore's
// /// proposals for fine-grained reactivity are stabilized.
// #[derive(Default)]
// pub struct GlobalState {
//     /// A map of type IDs to anything, allowing one storage of each type (each type is intended to a properties `struct` for a template). Entries must be `Clone`able becasue we assume them
//     /// to be `Signal`s or `struct`s composed of `Signal`s.
//     map: HashMap<TypeId, Box<dyn Any>>,
// }
// impl GlobalState {
//     pub fn get<T: Any>(&self) -> Option<T> {
//         let type_id = TypeId::of::<T>();
//         todo!()
//         // match self.map.get(&type_id) {

//         // }
//     }
// }

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
        // The Serde derivations will be stripped from the reactive version, but others will remain
        #[derive(Clone)]
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
        #[derive(Clone)]
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
        let new_2 = new.clone();
    }
}
