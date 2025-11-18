mod core; // So called because this contains what is essentially the core exposed logic of Perseus
#[cfg(engine)]
mod default_headers;
// mod render_ctx;
mod capsule;
#[cfg(engine)]
mod fn_types;
#[cfg(engine)]
mod states;
mod widget_component;

pub use self::core::*;
#[cfg(engine)]
pub use fn_types::*; /* There are a lot of render function traits in here, there's no
                      * point in spelling them all out */
#[cfg(engine)]
pub(crate) use default_headers::default_headers;
// pub use render_ctx::RenderCtx;
// pub(crate) use render_ctx::{RenderMode, RenderStatus};
pub use capsule::{Capsule, CapsuleInner};
#[cfg(engine)]
pub(crate) use states::States;

use crate::{errors::ClientError, path::PathMaybeWithLocale, state::TemplateState};
use sycamore::{
    prelude::{Scope, ScopeDisposer},
    view::View,
};
// Everything else in `fn_types.rs` is engine-only
/// The type of functions that are given a state and render a page.
pub(crate) type TemplateFn<G> = Box<
    dyn for<'a> Fn(
            Scope<'a>,
            PreloadInfo,
            TemplateState,
            PathMaybeWithLocale,
        ) -> Result<(View<G>, ScopeDisposer<'a>), ClientError>
        + Send
        + Sync,
>;
