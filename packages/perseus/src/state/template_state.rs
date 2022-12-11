use std::{any::TypeId, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

/// A marker for when the type of template state is unknown.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct UnknownStateType;

/// A wrapper for template state stored as a [`Value`]. This loses the
/// underlying type information, but allows for serialization. This is a
/// necessary compromise, since, without types being first-class citizens in
/// Rust, full template type management appears presently impossible.
#[derive(Clone, Debug)]
pub struct TemplateStateWithType<T: Serialize + DeserializeOwned> {
    /// The underlying state, stored as a [`Value`].
    pub(crate) state: Value,
    /// The user's actual type.
    ty: PhantomData<T>,
}
impl<T: Serialize + DeserializeOwned + 'static> TemplateStateWithType<T> {
    /// Convert the template state into its underlying concrete type, when that
    /// type is known.
    pub(crate) fn to_concrete(self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.state)
    }
    /// Creates a new empty template state.
    pub fn empty() -> Self {
        Self {
            state: Value::Null,
            ty: PhantomData,
        }
    }
    /// Checks if this state is empty. This not only checks for states created
    /// as `Value::Null`, but also those created with `()` explicitly set as
    /// their underlying type.
    pub fn is_empty(&self) -> bool {
        self.state.is_null() || TypeId::of::<T>() == TypeId::of::<()>()
    }
    /// Creates a new template state by deserializing from a string.
    pub(crate) fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        let state = Self {
            state: serde_json::from_str(s)?,
            ty: PhantomData,
        };
        Ok(state)
    }
    /// Creates a new template state from a pre-deserialized [`Value`].
    ///
    /// Note that end users should almost always prefer `::from_str()`, and this
    /// is intended primarily for server integrations.
    pub fn from_value(v: Value) -> Self {
        Self {
            state: v,
            ty: PhantomData,
        }
    }
    /// Transform this into a [`TemplateStateWithType`] with a different type.
    /// Once this is done, `.to_concrete()` can be used to get this type out
    /// of the container.
    pub(crate) fn change_type<U: Serialize + DeserializeOwned>(self) -> TemplateStateWithType<U> {
        TemplateStateWithType {
            state: self.state,
            ty: PhantomData,
        }
    }
}

// Any user state should be able to be made into this with a simple `.into()`
// for ergonomics
impl<T: Serialize + DeserializeOwned> From<T> for TemplateState {
    fn from(state: T) -> Self {
        Self {
            // This will happen at Perseus build-time (and should never happen unless the user uses non-string map types)
            state: serde_json::to_value(state).expect("serializing template state failed (this is almost certainly due to non-string map keys in your types, which can't be serialized by serde)"),
            ty: PhantomData,
        }
    }
}

/// A type alias for template state that has been converted into a [`Value`]
/// without retaining the information of the original type, which is done
/// internally to eliminate the need for generics, which cannot be used
/// internally in Perseus for user state. The actual type is restored at the
/// last minute when it's needed.
pub type TemplateState = TemplateStateWithType<UnknownStateType>;
