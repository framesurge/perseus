#[cfg(not(feature = "hydrate"))]
use sycamore::web::DomNode;
#[cfg(feature = "hydrate")]
use sycamore::web::HydrateNode;
use sycamore::{prelude::Scope, utils::render::insert, view::View};
use web_sys::Element;

use crate::template::TemplateNodeType;

/// Renders or hydrates the given view to the given node,
/// depending on feature flags. This will atuomatically handle
/// proper scoping.
///
/// This has the option to force a render by ignoring the initial elements.
///
/// **Warning:** if hydration is being used, it is expected that
/// the given view was created inside a `with_hydration_context()` closure.
// TODO Make sure hydration will work when it's targeted at a blank canvas...
// XXX This is *highly* dependent on internal Sycamore implementation
// details! (TODO PR for `hydrate_to_with_scope` etc.)
#[allow(unused_variables)]
pub(crate) fn render_or_hydrate(
    cx: Scope,
    view: View<TemplateNodeType>,
    parent: Element,
    force_render: bool,
) {
    #[cfg(feature = "hydrate")]
    {
        // We need `sycamore::hydrate_to_with_scope()`!
        // --- Verbatim copy from Sycamore, changed for known scope ---
        // Get children from parent into a View to set as the initial node value.
        let mut children = Vec::new();
        let child_nodes = parent.child_nodes();
        for i in 0..child_nodes.length() {
            children.push(child_nodes.get(i).unwrap());
        }
        let children = children
            .into_iter()
            .map(|x| View::new_node(HydrateNode::from_web_sys(x)))
            .collect::<Vec<_>>();

        insert(
            cx,
            &HydrateNode::from_web_sys(parent.into()),
            view, // We assume this was created in `with_hydration_context(..)`
            if force_render {
                None
            } else {
                Some(View::new_fragment(children))
            },
            None,
            false,
        );
    }
    #[cfg(not(feature = "hydrate"))]
    {
        // We have to delete the existing content before we can render the new stuff
        parent.set_inner_html("");
        insert(
            cx,
            &DomNode::from_web_sys(parent.into()),
            view,
            None,
            None,
            false,
        );
    }
}