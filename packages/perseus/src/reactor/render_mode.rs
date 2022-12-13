use crate::{error_views::{ErrorViews, ServerErrorData}, errors::ServerError, path::*, state::TemplateState, stores::ImmutableStore, template::ArcTemplateMap};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};
use sycamore::web::Html;

/// The status of a build-time render.
#[derive(Debug)]
pub(crate) enum RenderStatus {
    /// The render is proceeding well.
    Ok,
    /// There was an error.
    Err(ServerError),
    /// The render was cancelled, since a widget couldn't be rendered at
    /// build-time.
    Cancelled,
}
impl Default for RenderStatus {
    fn default() -> Self {
        Self::Ok
    }
}

/// The different modes of rendering on the engine-side. On the browser-side,
/// there is only one mode of rendering.
///
/// Ths render mode is primarily used to inform the non-delayed widgets of how
/// they should render.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub(crate) enum RenderMode<G: Html> {
    /// We're rendering at build-time. Any non-delayed widgets should render if
    /// they are not going to alter the render properties of their caller.
    /// Otherwise, they should silently fail the render and set the attached
    /// [`Cell`] of this variant to `true` to inform the renderer.
    Build {
        /// Whether or not the render was cancelled due to a capsule being
        /// unable to be rendered (having this determined *during* the
        /// render avoids the need for the user to specify
        /// all their pages' dependencies (which might be impossible with
        /// incremental generation)).
        render_status: Rc<RefCell<RenderStatus>>,
        /// The render configuration for widgets. This will include both widgets
        /// that are safe to be built at build-time, and widgets that
        /// are not.
        widget_render_cfg: HashMap<String, String>,
        /// The app's immutable store. (This is cheap to clone.)
        immutable_store: ImmutableStore,
        /// The app's templates, including capsules.
        templates: ArcTemplateMap<G>,
        /// An accumulator of the widget states involved in rendering this
        /// template. We need to be able to collect these to later send
        /// them to clients for hydration.
        widget_states: Rc<RefCell<HashMap<String, (String, Value)>>>,
    },
    /// We're rendering at request-time in order to determine what the
    /// dependencies of this page/widget are. Each widget should check if
    /// its state is available in the given map, proceeding with its
    /// render if it is, or simply adding its route to a simple accumulator if
    /// not.
    ///
    /// Once we get to the last layer of dependencies, the accumulator will come
    /// out with nothing new, and then the return value is the
    /// fully-rendered content!
    Request {
        /// The widget states and attached capsule names. Each of these is fallible,
        /// and the widget component will render an appropriate error page if necessary.
        widget_states: Rc<HashMap<PathMaybeWithLocale, Result<(String, TemplateState), ServerErrorData>>>,
        /// The app's templates and capsules.
        templates: ArcTemplateMap<G>,
        /// The app's error views.
        error_views: Arc<ErrorViews<G>>,
        /// A list of the paths to widgets that haven't yet been resolved in any
        /// way. These will be deduplicated and then resolved in
        /// parallel, along with having their states built.
        ///
        /// These paths do not contain the locale because a capsule from a
        /// different locale can never be included.
        unresolved_widget_accumulator: Rc<RefCell<Vec<PathWithoutLocale>>>,
    },
    /// We're rendering a head, where widgets are not allowed.
    Head,
    /// We're rendering an error, where widgets are not allowed.
    Error,
}
