use std::ops::Deref;
use futures::executor::block_on;
use sycamore::{prelude::{Scope, View}, web::Html};
use crate::{RenderCtx, Template, errors::ServerError, template::{RenderStatus, TemplateState, render_ctx::RenderMode}};
use crate::i18n::Translator;

/// A *capsule*, a special type of template in Perseus that can also accept
/// *properties*. Capsules are basically a very special type of Sycamore component
/// that can integrate fully with Perseus' state platform, generating their own states
/// at build-time, request-time, etc. They're then used in one or more pages, and provided
/// extra properties.
pub struct Capsule<G: Html> {
    /// The underlying template (since capsules are just a special type of template).
    pub(crate) template: Template<G>,
    /// A function that returns the fallback view to be rendered between when the page is ready
    /// and when the capsule's state has been fetched.
    ///
    /// Note that this starts as `None`, but, if it's not set, `PerseusApp` will panic. So, for later
    /// code, this can be assumed to be always `Some`.
    pub(crate) fallback: Option<Box<dyn Fn(Scope) -> View<G>>>,
}
impl<G: Html> std::fmt::Debug for Capsule<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capsule")
            // TODO
            .finish()
    }
}
impl<G: Html> Capsule<G> {
    /// Creates a new [`Capsule`] with the given path. The argument provided here functions
    /// in the same way as the argument given to [`Template`] does.
    pub fn new(path: impl Into<String> + std::fmt::Display) -> Self {
        // We create a template with this path, and then turn it into a capsule
        let mut template = Template::new(path);
        template.is_capsule = true;
        Self {
            template,
            fallback: None,
        }
    }
    /// Declares the fallback view to render for this capsule. When Perseus renders a page
    /// of your app, it fetches the page itself, along with all the capsules it needs.
    /// If the page is ready before all the capsules, then it will be displayed immediately,
    /// with fallback views for the capsules that aren't ready yet. Once they are ready, they
    /// will be updated.
    ///
    /// This fallback view cannot access any of the state that the capsule generated, but it can
    /// access any properties provided to it by the page, along with a translator and the like.
    /// This view is fully reactive, it just doesn't have the state yet.
    ///
    /// **Warning:** if you do not set a fallback view for a capsule, your app will not compile!
    // TODO This function should take properties
    pub fn fallback(mut self, view: impl Fn (Scope) -> View<G> + 'static) -> Self {
        self.fallback = Some(Box::new(view));
        self
    }
    /// Sets the fallback for this capsule to be an empty view.
    ///
    /// You should be careful using this function in production, since it is very often not
    /// what you actually want (especially since empty views have no size, which may compromise
    /// your layouts: be sure to test this).
    pub fn empty_fallback(mut self) -> Self {
        self.fallback = Some(Box::new(|cx| sycamore::view! { cx, }));
        self
    }
}

// We want all the methods of `Template` directly accessible
impl<G: Html> Deref for Capsule<G> {
    type Target = Template<G>;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

#[sycamore::component]
pub fn Widget<G: Html>(cx: Scope, path: &str) -> View<G> {
    use sycamore::prelude::*;

    // Handle leading and trailing slashes
    let path = path.strip_prefix('/').unwrap_or(&path);
    let path = path.strip_suffix('/').unwrap_or(&path);

    // This will always be rendered with access to the Perseus render context, which we will be working with a lot!
    let render_ctx = RenderCtx::from_ctx(cx);
    match &render_ctx.render_mode {
        RenderMode::Build {
            render_status,
            widget_render_cfg,
            immutable_store,
            templates,
            widget_states,
        } => {
            // If the render status isn't good, don't even bother proceeding, and fail-fast instead
            if !matches!(*render_status.borrow(), RenderStatus::Ok) {
                return View::empty()
            }

            // Check if we're in the
            if let Some(capsule_name) = widget_render_cfg.get(path) {
                let capsule = match templates.get(capsule_name) {
                    Some(capsule) => capsule,
                    None => panic!(""),
                };
                // Make sure this capsule would be safe for building
                // If this were an incrementally generated widget, we wouldn't have even gotten this far, as
                // it wouldn't be in the render config
                if capsule.uses_request_state() || capsule.revalidates() {
                    *render_status.borrow_mut() = RenderStatus::Cancelled;
                    View::empty()
                } else {
                    let translator = use_context::<Signal<Translator>>(cx).get_untracked();
                    // Get the path in a way we can work with
                    let path_encoded = format!(
                        "{}-{}",
                        translator.get_locale(),
                        // The user provided this
                        urlencoding::encode(path)
                    );
                    // Since this widget has state built at build-time that will never change, it *must*
                    // be in the immutable store (only revalidating states go into the mutable store,
                    // and this would be `false` in the map if it revalidated!)
                    // The immutable store is really just a filesystem API, and we have no choice
                    // but to block here
                    let state = match block_on(immutable_store
                                               .read(&format!("static/{}.head.html", path_encoded))) {
                        Ok(state) => state,
                        Err(err) => {
                            *render_status.borrow_mut() = RenderStatus::Err(err.into());
                            return View::empty()
                        }
                    };
                    let state = match TemplateState::from_str(&state) {
                        Ok(state) => state,
                        Err(err) => {
                            *render_status.borrow_mut() = RenderStatus::Err(ServerError::InvalidPageState { source: err });
                            return View::empty()
                        },
                    };

                    // Add this to the list of widget states so they can be written for later use
                    widget_states.borrow_mut().insert(path.to_string(), (capsule_name.to_string(), state.state.clone()));

                    // capsule.render_widget_for_template_server(path.to_string(), state, cx)
                    todo!()
                }
            } else {
                // This widget will be incrementally generated (TODO should we try to build it now?)
                *render_status.borrow_mut() = RenderStatus::Cancelled;
                View::empty()
            }
        },
        RenderMode::Request {
            widget_states,
            templates,
            unresolved_widget_accumulator
        } => {
            // Check if we've already built this widget (i.e. are we up to this layer, or a later one?)
            match widget_states.get(path) {
                Some((capsule_name, state)) => {
                    // Get the capsule this widget was generated by
                    let capsule = match templates.get(capsule_name) {
                        Some(capsule) => capsule,
                        None => panic!(""),
                    };
                    // Use that to render the widget for the server-side (this should *not* create a new render context)
                    // BUG Need to prove to the compiler that we'll always be returning an `SsrNode` on the engine-side...
                    // capsule.render_widget_for_template_server(path.to_string(), state.clone(), cx)
                    todo!()
                },
                None => {
                    // Just add this path to the list of unresolved ones, and it will be resolved in time for the next pass
                    unresolved_widget_accumulator.borrow_mut().push(path.to_string());
                    View::empty()
                },
            }
        },
        RenderMode::Head => panic!("widgets cannot be used in heads")
    }
}
