//! This module contains implementations of common Rust collections that work well with
//! Perseus' reactive state platform. So that this is as extensible as possible, all the
//! code in this module follows a general pattern, so that you can easily create your own
//! implementations as necessary!
//!
//! First, it is important to understand that each type has two versions: `Collection` and
//! `CollectionNested`. The former works with elements that will be simply wrapped in `RcSignal`s,
//! while the latter expects its elements to implement `MakeRx` etc. This difference can be
//! extremely useful for users, because sometimes you'll want a vector of some reactive `struct`,
//! while other times you'll just want a vector of `String`s.
//!
//! Second, each reactive collection is a thin wrapper over the basic collection. For example,
//! [`RxVecNested`] is defined as `struct RxVecNested<T>(Vec<T>)`, with several constraints on
//! `T`.
//!
//! Third, each reactive collection has itself two types: `RxCollection` and `RxCollectionRx`.
//! The former is the base, unreactive collections, while the latter is fully reactive. This
//! is just like how the `#[derive(ReactiveState)]` macro creates an alias type `MyStateRx`
//! for some state `MyState`. Note that these `struct`s should have the same type bounds on `T`.
//!
//! Fourth, the unreactive types will have to implement `Serialize` and `Deserialize`, from Serde.
//! You can get this working by *omitting* the `Serialize + DeserializeOwned` bounds on `T`
//! in the unreactive type definition, and by then letting the derive macros from Serde fill
//! them in automatically. Note the use of `DeserializeOwned` in type bounds, which avoids
//! lifetime concerns and HRTBs.
//!
//! Finally, every file in this module follows the same code pattern, allowing maximal extensibility
//! and self-documentation:
//!
//! ```
//! // --- Type definitions ---
//! // ...
//! // --- Reactivity implementations ---
//! // ...
//! // --- Dereferencing ---
//! // ...
//! // --- Conversion implementation ---
//! // ...
//! // --- Freezing implementation ---
//! // ...
//! ```
//!
//! The *type definitions* section contains the actual definitions of the reactive and unreactive
//! collection types, while the *reactivity implementations* section contains the implementations
//! of `MakeRx` for the unreactive type, and `MakeUnrx` for the reactive type.
//!
//! The *dereferencing* section contains implementations that allow users to use the methods of
//! the underlying collection on these wrapper types. For example, by implementing `Deref` with
//! a target of `Vec<T::Rx>` for `RxVecNestedRx<T>`, users can take that reactive type and call
//! methods like `.iter()` on it.
//!
//! The *conversion implementation* section implements `From` for the unreactive type, allowing
//! users to easily create the base unreactive type from the type it wraps (e.g. `RxVecNested<T>`
//! from a `Vec<T>`). This is primarily used in functions like `get_build_state`, where users
//! can create the normal Rust collection, and just add `.into()` to integrate it with the Perseus
//! state platform. Note that we **do not** implement `From` for the reactive version, as this will
//! never be of use to users (reactive types should only ever come out of Perseus itself, so they
//! can be registered in the state store, etc.).
//!
//! Finally, the *freezing implementation* section implements `Freeze` for the reactive type, which
//! allows Perseus to turn it into a `String` easily for state freezing. Thawing is handled automatically
//! and internally.
//!
//! *A brief note on `RxResult<T, E>`: the reactive result type does not follow the patterns described
//! above, and it defined in a separate module, because it does not have a non-nested equivalent. This
//! is because such a thing would have no point, as there are no 'fields' in a `Result` (or any other
//! `enum`, for that matter) itself. If a non-nested version is required, one should simply use
//! `std::result::Result`. The same goes for `Option<T>`, although there is presently no defined reactive
//! container for this.*
//!
//! **Note:** as a user, you will still have to use `#[rx(nested)]` over any reactive types you use!

mod rx_vec_nested;

pub use rx_vec_nested::{RxVecNested, RxVecNestedRx};
