#[cfg(any(client, doc))]
use sycamore::utils::render::insert;
#[cfg(all(not(feature = "hydrate"), any(client, doc)))]
use sycamore::web::DomNode;
#[cfg(engine)]
use sycamore::web::SsrNode;
use sycamore::{prelude::Scope, view::View};

/// Renders or hydrates the given view to the given node,
/// depending on feature flags. This will atuomatically handle
/// proper scoping.
///
/// This has the option to force a render by ignoring the initial elements.
///
/// **Warning:** if hydration is being used, it is expected that
/// the given view was created inside a `with_hydration_context()` closure.
// XXX This is *highly* dependent on internal Sycamore implementation
// details! (TODO PR for `hydrate_to_with_scope` etc.)
#[cfg(any(client, doc))]
#[allow(unused_variables)]
pub(crate) fn render_or_hydrate(
    cx: Scope,
    view: View<crate::template::BrowserNodeType>,
    parent: web_sys::Element,
    force_render: bool,
) {
    use sycamore::utils::hydrate::{with_hydration_context, with_no_hydration_context};

    #[cfg(feature = "hydrate")]
    {
        use sycamore::web::HydrateNode;

        // If we're forcing a proper render, then we'll have to remove existing content
        if force_render {
            parent.set_inner_html("");
        }

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
            if force_render {
                with_no_hydration_context(|| view)
            } else {
                with_hydration_context(|| view)
            },
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

/// Renders the given view to a string in a fallible manner, managing hydration
/// automatically.
// XXX This is *highly* dependent on internal Sycamore implementation
// details!
#[cfg(engine)]
pub(crate) fn ssr_fallible<E>(
    view_fn: impl FnOnce(Scope) -> Result<View<SsrNode>, E>,
) -> Result<String, E> {
    use sycamore::web::WriteToString;
    use sycamore::{prelude::create_scope_immediate, utils::hydrate::with_hydration_context}; // XXX This may become private one day!

    let mut ret = Ok(String::new());
    create_scope_immediate(|cx| {
        // Usefully, this wrapper can return anything!
        let view_res = with_hydration_context(|| view_fn(cx));
        match view_res {
            Ok(view) => {
                let mut view_str = String::new();
                for node in view.flatten() {
                    node.write_to_string(&mut view_str);
                }
                ret = Ok(view_str);
            }
            Err(err) => {
                ret = Err(err);
            }
        }
    });

    ret
}
