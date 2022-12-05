use std::{any::TypeId, ops::Deref};
use futures::executor::block_on;
use sycamore::{prelude::{Scope, Signal, View, use_context}, web::{Html, SsrNode}};
use crate::{PathWithoutLocale, RenderCtx, Template, errors::ServerError, router::{RouteInfo, RouteVerdict, match_route}, state::PssContains, template::{PreloadInfo, RenderStatus, TemplateState, render_ctx::RenderMode}};
use crate::i18n::Translator;

use super::TemplateNodeType;

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
    pub(crate) fallback: Option<Box<dyn Fn(Scope) -> View<G> + Send + Sync>>,
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
    pub fn fallback(mut self, view: impl Fn (Scope) -> View<G> + Send + Sync + 'static) -> Self {
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

// pub fn Widget<G: Html>(cx: Scope, path: &str) -> View<G> {
//     widget_engine(cx, path)
// }
/// A Sycamore component for rendering a Perseus widget by its path (not including the `__capsule/`
/// prefix).
///
/// # Implementation notes
/// This component behaves completely differently on the engine-side from the browser-side,
/// due to the rather complex nature of the Perseus build process. In some rare cases, you may
/// feel the urge to try to server-side render a widget while in the browser. Attempting this will
/// result in a panic. Attempting to use this on non-browser infrastructure (e.g. with an alternate
/// Sycamore backend) will also fail, as this relies on transmuting behind-the-scenes to perform
/// manual monomorphic specialization (though manual type checks are performed, making UB impossible).
///
/// Use this as documented, and you'll be fine. If you need it in an alternate rendering backend, please
/// open an issue.
#[sycamore::component]
pub fn Widget<G: Html>(cx: Scope, path: &str) -> View<G> {
    // Handle leading and trailing slashes
    let path = path.strip_prefix('/').unwrap_or(&path);
    let path = path.strip_suffix('/').unwrap_or(&path);

    let path = PathWithoutLocale(format!("__capsule/{}", path));

    // On the engine-side, we expect an `SsrNode`
    #[cfg(not(target_arch = "wasm32"))]
    if TypeId::of::<G>() == TypeId::of::<SsrNode>() {
        let view = engine_widget(cx, path);
        // SAFETY: We have generated the correct type of view through manual type checks.
        // If anyone knows a better way to specialize generic functions that aren't
        // methods on traits, please let me know.
        return unsafe { std::mem::transmute_copy(&view) }
    }

    // On the browser-side, we expect a `TemplateNodeType` (i.e. `HydrateNode` or `DomNode`)
    #[cfg(target_arch = "wasm32")]
    if TypeId::of::<G>() == TypeId::of::<TemplateNodeType>() {
        let view = browser_widget(cx, path);
        // SAFETY: We have generated the correct type of view through manual type checks.
        // If anyone knows a better way to specialize generic functions that aren't
        // methods on traits, please let me know.
        return unsafe { std::mem::transmute_copy(&view) }
    }

    // If we've gotten this far, someone is probably trying to server-side render a widget in the browser,
    // which will not work.
    panic!("widget rendering failed (expected `SsrNode` on engine or `DomNode`/`HydrateNode` on browser); if you're trying to server-side render a widget in the browser, this is impossible due to how widgets behave as pages")
}

/// The internal browser-side logic for widgets.
#[cfg(target_arch = "wasm32")]
fn browser_widget(cx: Scope, path: PathWithoutLocale) -> View<TemplateNodeType> {
    let render_ctx = RenderCtx::from_ctx(cx);
    let translator = use_context::<Signal<Translator>>(cx).get_untracked();
    let locale = translator.get_locale();
    let localized_path = match locale.as_str() {
        "xx-XX" => path.to_string(),
        locale => format!("{}/{}", locale, path)
    };
    // This has the locale, and is used as the identifier for the calling page in the PSS.
    // This will be `Some(..)` as long as we're not running in an error page (in which case
    // we should immediately terminate anyway) or the like.
    let caller_path = render_ctx.router.get_path().unwrap();

    // Figure out route information for this
    let path_segments = localized_path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>(); // This parsing is identical to the Sycamore router's
    let verdict = match_route(&path_segments, &render_ctx.render_cfg, &render_ctx.templates, &render_ctx.locales);

    match verdict {
        RouteVerdict::Found(RouteInfo {
            path,
            template: capsule,
            was_incremental_match,
            locale,
        }) => {
            // TODO Declare this as a dependency on the caller
            // TODO Declare the caller as a dependent on this

            capsule.render_widget_for_template_client(
                localized_path,
                cx,
                PreloadInfo {
                    locale,
                    was_incremental_match,
                },
            )
        },
        // Widgets are all resolved on the server-side, meaning they are checked then too (be it at build-time
        // or request-time). If this happpens, the user is rendering an invalid widget on the browser-side only.
        _ => todo!("error page")
    }
}

/// The internal engine-side logic for widgets.
#[cfg(not(target_arch = "wasm32"))]
fn engine_widget(cx: Scope, path: PathWithoutLocale) -> View<SsrNode> {
    use sycamore::prelude::*;

    use crate::PathMaybeWithLocale;

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
            if let Some(capsule_name) = widget_render_cfg.get(&*path) {
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
                    let locale = translator.get_locale();
                    // Get the path in a way we can work with
                    let path_encoded = format!(
                        "{}-{}",
                        &locale,
                        // The user provided this
                        urlencoding::encode(&path)
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

                    capsule.render_widget_for_template_server(PathMaybeWithLocale::new(&path, &locale), state, cx)
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
            match widget_states.get(&*path) {
                Some((capsule_name, state)) => {
                    let translator = use_context::<Signal<Translator>>(cx).get_untracked();
                    let locale = translator.get_locale();
                    // Get the capsule this widget was generated by
                    let capsule = match templates.get(capsule_name) {
                        Some(capsule) => capsule,
                        None => panic!(""),
                    };
                    // Use that to render the widget for the server-side (this should *not* create a new render context)
                    capsule.render_widget_for_template_server(PathMaybeWithLocale::new(&path, &locale), state.clone(), cx)
                },
                None => {
                    // Just add this path to the list of unresolved ones, and it will be resolved in time for the next pass
                    unresolved_widget_accumulator.borrow_mut().push(path);
                    View::empty()
                },
            }
        },
        RenderMode::Head => panic!("widgets cannot be used in heads")
    }

}
