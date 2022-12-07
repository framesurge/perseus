// This module contains the primary shared logic in Perseus, and is broken up to avoid
// a 2000-line file.

mod utils;
mod renderers;
mod getters;
mod setters;
// These are broken out because of state-management closure wrapping
mod state_setters;

pub(crate) use utils::*;

use sycamore::{prelude::create_scope, view::View, web::Html};
use crate::{template::default_headers, utils::ComputedDuration};
use super::fn_types::*;

/// A single template in an app. Each template is comprised of a Sycamore view,
/// a state type, and some functions involved with generating that state. Pages
/// can then be generated from particular states. For instance, a single `docs`
/// template could have a state `struct` that stores a title and some content,
/// which could then render as many pages as desired.
///
/// You can read more about the templates system [here](https://arctic-hen7.github.io/perseus/en-US/docs/next/core-principles).
///
/// Note that all template states are passed around as `String`s to avoid
/// type maps and other inefficiencies, since they need to be transmitted over
/// the network anyway. As such, this `struct` is entirely state-agnostic,
/// since all the state-relevant functions merely return `String`s. The
/// various proc macros used to annotate such functions (e.g.
/// `#[perseus::build_state]`) perform serialization/deserialization
/// automatically for convenience.
pub struct Template<G: Html> {
    /// The path to the root of the template. Any build paths will be inserted
    /// under this.
    path: String,
    /// A function that will render your template. This will be provided the
    /// rendered properties, and will be used whenever your template needs
    /// to be prerendered in some way. This should be very similar to the
    /// function that hydrates your template on the client side.
    /// This will be executed inside `sycamore::render_to_string`, and should
    /// return a `Template<SsrNode>`. This takes an `Option<Props>`
    /// because otherwise efficient typing is almost impossible for templates
    /// without any properties (solutions welcome in PRs!).
    template: TemplateFn<G>,
    /// A function that will be used to populate the document's `<head>` with
    /// metadata such as the title. This will be passed state in
    /// the same way as `template`, but will always be rendered to a string,
    /// which will then be interpolated directly into the `<head>`,
    /// so reactivity here will not work!
    #[cfg(not(target_arch = "wasm32"))]
    head: HeadFn,
    /// A function to be run when the server returns an HTTP response. This
    /// should return headers for said response, given the template's state.
    /// The most common use-case of this is to add cache control that respects
    /// revalidation. This will only be run on successful responses, and
    /// does have the power to override existing headers. By default, this will
    /// create sensible cache control headers.
    #[cfg(not(target_arch = "wasm32"))]
    set_headers: SetHeadersFn,
    /// A function that generates the information to begin building a template.
    /// This is responsible for generating all the paths that will built for
    /// that template at build-time (which may later be extended with
    /// incremental generation), along with the generation of any extra
    /// state that may be collectively shared by other state generating
    /// functions.
    #[cfg(not(target_arch = "wasm32"))]
    get_build_paths: Option<GetBuildPathsFn>,
    /// Defines whether or not any new paths that match this template will be
    /// prerendered and cached in production. This allows you to
    /// have potentially billions of templates and retain a super-fast build
    /// process. The first user will have an ever-so-slightly slower
    /// experience, and everyone else gets the benefits afterwards. This
    /// requires `get_build_paths`. Note that the template root will NOT
    /// be rendered on demand, and must be explicitly defined if it's wanted. It
    /// can use a different template.
    #[cfg(not(target_arch = "wasm32"))]
    incremental_generation: bool,
    /// A function that gets the initial state to use to prerender the template
    /// at build time. This will be passed the path of the template, and
    /// will be run for any sub-paths.
    #[cfg(not(target_arch = "wasm32"))]
    get_build_state: Option<GetBuildStateFn>,
    /// A function that will run on every request to generate a state for that
    /// request. This allows server-side-rendering. This can be used with
    /// `get_build_state`, though custom amalgamation logic must be provided.
    #[cfg(not(target_arch = "wasm32"))]
    get_request_state: Option<GetRequestStateFn>,
    /// A function to be run on every request to check if a template prerendered
    /// at build-time should be prerendered again. If used with
    /// `revalidate_after`, this function will only be run after that time
    /// period. This function will not be parsed anything specific to the
    /// request that invoked it.
    #[cfg(not(target_arch = "wasm32"))]
    should_revalidate: Option<ShouldRevalidateFn>,
    /// A length of time after which to prerender the template again. The given
    /// duration will be waited for, and the next request after it will lead
    /// to a revalidation. Note that, if this is used with incremental
    /// generation, the counter will only start after the first render
    /// (meaning if you expect a weekly re-rendering cycle for all pages,
    /// they'd likely all be out of sync, you'd need to manually implement
    /// that with `should_revalidate`).
    #[cfg(not(target_arch = "wasm32"))]
    revalidate_after: Option<ComputedDuration>,
    /// Custom logic to amalgamate potentially different states generated at
    /// build and request time. This is only necessary if your template uses
    /// both `build_state` and `request_state`. If not specified and both are
    /// generated, request state will be prioritized.
    #[cfg(not(target_arch = "wasm32"))]
    amalgamate_states: Option<AmalgamateStatesFn>,
    /// Whether or not this template is actually a capsule. This impacts some
    /// aspects of internal handling.
    ///
    /// Do NOT manually change this unless you really know what you're doing!
    pub(crate) is_capsule: bool,
    /// Whether or not this template's pages can have their builds rescheduled
    /// from build-time to request-time if they depend on capsules that aren't ready
    /// with state at build-time. This is included as a precaution to seemingly erroneous
    /// performance changes with pages. If rescheduling is needed and it hasn't been explicitly
    /// allowed, an error will be returned from the build process.
    pub(crate) can_be_rescheduled: bool,
}
impl<G: Html> std::fmt::Debug for Template<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Template")
            .field("path", &self.path)
            .field("template", &"TemplateFn")
            .field("head", &"HeadFn")
            .field("set_headers", &"SetHeadersFn")
            // TODO Server-specific stuff
            .finish()
    }
}
impl<G: Html> Template<G> {
    /// Creates a new [`Template`]. By default, this has absolutely no
    /// associated data. If rendered, it would result in a blank screen.
    pub fn new(path: impl Into<String> + std::fmt::Display) -> Self {
        Self {
            path: path.to_string(),
            // Because of the scope disposer return type, this isn't as trivial as an empty function
            template: Box::new(|_, _, _, _| {
                let disposer = create_scope(|cx| {});
                Ok((View::empty(), disposer))
            }),
            // Unlike `template`, this may not be set at all (especially in very simple apps)
            #[cfg(not(target_arch = "wasm32"))]
            head: Box::new(|_, _| Ok(View::empty())),
            // We create sensible header defaults here
            #[cfg(not(target_arch = "wasm32"))]
            set_headers: Box::new(|_| Ok(default_headers())),
            #[cfg(not(target_arch = "wasm32"))]
            get_build_paths: None,
            #[cfg(not(target_arch = "wasm32"))]
            incremental_generation: false,
            #[cfg(not(target_arch = "wasm32"))]
            get_build_state: None,
            #[cfg(not(target_arch = "wasm32"))]
            get_request_state: None,
            #[cfg(not(target_arch = "wasm32"))]
            should_revalidate: None,
            #[cfg(not(target_arch = "wasm32"))]
            revalidate_after: None,
            #[cfg(not(target_arch = "wasm32"))]
            amalgamate_states: None,
            // There is no mechanism to set this to `true`, except through the `Capsule` struct
            is_capsule: false,
            can_be_rescheduled: false,
        }
    }
}

// The engine needs to know whether or not to use hydration, this is how we pass
// those feature settings through
#[cfg(not(feature = "hydrate"))]
#[doc(hidden)]
pub(crate) type TemplateNodeType = sycamore::prelude::DomNode;
#[cfg(feature = "hydrate")]
#[doc(hidden)]
pub(crate) type TemplateNodeType = sycamore::prelude::HydrateNode;
