use super::Reactor;
use crate::{
    error_views::ErrorPosition,
    errors::ClientError,
    reactor::InitialView,
    router::{PageDisposer, PerseusRoute, RouteVerdict, RouterLoadState},
    template::BrowserNodeType,
    utils::{checkpoint, render_or_hydrate, replace_head},
};
use sycamore::prelude::{create_effect, create_signal, on_mount, view, ReadSignal, Scope, View};
use sycamore_futures::spawn_local_scoped;
use sycamore_router::{navigate_replace, HistoryIntegration, RouterBase};
use web_sys::Element;

// We don't want to bring in a styling library, so we do this the old-fashioned
// way! We're particularly comprehensive with these because the user could
// *potentially* stuff things up with global rules https://medium.com/@jessebeach/beware-smushed-off-screen-accessible-text-5952a4c2cbfe
const ROUTE_ANNOUNCER_STYLES: &str = r#"
    margin: -1px;
    padding: 0;
    border: 0;
    clip: rect(0 0 0 0);
    height: 1px;
    width: 1px;
    overflow: hidden;
    position: absolute;
    white-space: nowrap;
    word-wrap: normal;
"#;

impl Reactor<BrowserNodeType> {
    /// Sets the handlers necessary to run the event-driven components of
    /// Perseus (in a reactive web framework, there are quite a few of
    /// these). This should only be executed at the beginning of the
    /// browser-side instantiation.
    ///
    /// This is internally responsible for fetching the initial load and
    /// rendering it, starting the reactive cycle based on the given scope
    /// that will handle subsequent loads and the like.
    ///
    /// This takes the app-level scope.
    ///
    /// As Sycamore works by starting a reactive cycle, rather than by calling a
    /// function that never terminates, this will 'finish' as soon as the intial
    /// load is ready. However, in cases of critical errors that have been
    /// successfully displayed, the app-level scope should be disposed of.
    /// If this should occur, this will return `false`, indicating that the
    /// app was not successful. Note that server errors will not cause this,
    /// and they will receive a router. This situation is very rare, and
    /// affords a plugin action for analytics.
    pub(crate) fn start<'a>(&'a self, cx: Scope<'a>) -> bool {
        // We must be in the first load
        assert!(
            self.is_first.get(),
            "attempted to instantiate perseus after first load"
        );

        // --- Route announcer ---

        let route_announcement = create_signal(cx, String::new());
        // Append the route announcer to the end of the document body
        let document = web_sys::window().unwrap().document().unwrap();
        let announcer = document.create_element("p").unwrap();
        announcer.set_attribute("aria-live", "assertive").unwrap();
        announcer.set_attribute("role", "alert").unwrap();
        announcer
            .set_attribute("style", ROUTE_ANNOUNCER_STYLES)
            .unwrap();
        announcer.set_id("__perseus_route_announcer");
        let body_elem: Element = document.body().unwrap().into();
        body_elem
            .append_with_node_1(&announcer.clone().into())
            .unwrap();
        // Update the announcer's text whenever the `route_announcement` changes
        create_effect(cx, move || {
            let ra = route_announcement.get();
            announcer.set_inner_html(&ra);
        });

        // Create a derived state for the route announcement
        // We do this with an effect because we only want to update in some cases (when
        // the new page is actually loaded) We also need to know if it's the first
        // page (because we don't want to announce that, screen readers will get that
        // one right)

        // This is not whether the first page has been loaded or not, it's whether or
        // not we're still on it
        let mut on_first_page = true;
        let load_state = self.router_state.get_load_state_rc();
        create_effect(cx, move || {
            if let RouterLoadState::Loaded { path, .. } = &*load_state.get() {
                if on_first_page {
                    // This is the first load event, so the next one will be for a new page (or at
                    // least something that we should announce, if this page reloads then the
                    // content will change, that would be from thawing)
                    on_first_page = false;
                } else {
                    // TODO Validate approach with reloading
                    // A new page has just been loaded and is interactive (this event only fires
                    // after all rendering and hydration is complete)
                    // Set the announcer to announce the title, falling back to the first `h1`, and
                    // then falling back again to the path
                    let document = web_sys::window().unwrap().document().unwrap();
                    // If the content of the provided element is empty, this will transform it into
                    // `None`
                    let make_empty_none = |val: Element| {
                        let val = val.inner_html();
                        if val.is_empty() {
                            None
                        } else {
                            Some(val)
                        }
                    };
                    let title = document
                        .query_selector("title")
                        .unwrap()
                        .and_then(make_empty_none);
                    let announcement = match title {
                        Some(title) => title,
                        None => {
                            let first_h1 = document
                                .query_selector("h1")
                                .unwrap()
                                .and_then(make_empty_none);
                            match first_h1 {
                                Some(val) => val,
                                // Our final fallback will be the path
                                None => path.to_string(),
                            }
                        }
                    };

                    route_announcement.set(announcement);
                }
            }
        });

        // --- HSR and live reloading ---

        // This section handles live reloading and HSR freezing
        // We used to have an indicator shared to the macros, but that's no longer used
        #[cfg(all(feature = "live-reload", debug_assertions))]
        {
            use crate::state::Freeze;
            // Set up a oneshot channel that we can use to communicate with the WS system
            // Unfortunately, we can't share senders/receivers around without bringing in
            // another crate And, Sycamore's `RcSignal` doesn't like being put into
            // a `Closure::wrap()` one bit
            let (live_reload_tx, live_reload_rx) = futures::channel::oneshot::channel();
            sycamore_futures::spawn_local_scoped(cx, async move {
                match live_reload_rx.await {
                    // This will trigger only once, and then can't be used again
                    // That shouldn't be a problem, because we'll reload immediately
                    Ok(_) => {
                        #[cfg(feature = "hsr")]
                        {
                            let frozen_state = self.freeze();
                            Self::hsr_freeze(frozen_state).await;
                        }
                        crate::state::force_reload();
                        // We shouldn't ever get here unless there was an error,
                        // the entire page will be fully
                        // reloaded
                    }
                    _ => (),
                }
            });

            // If live reloading is enabled, connect to the server now
            // This doesn't actually perform any reloading or the like, it just signals
            // places that have access to the render context to do so (because we need that
            // for state freezing/thawing)
            crate::state::connect_to_reload_server(live_reload_tx);
        }

        // This handles HSR thawing
        #[cfg(all(feature = "hsr", debug_assertions))]
        {
            sycamore_futures::spawn_local_scoped(cx, async move {
                // We need to make sure we don't run this more than once, because that would
                // lead to a loop It also shouldn't run on any pages after the
                // initial load
                if self.is_first.get() {
                    self.is_first.set(false);
                    self.hsr_thaw().await;
                }
            });
        };

        // --- Error handlers ---

        let popup_error_disposer = PageDisposer::default();
        // Broken out for ease if the reactor can't be created
        let popup_error_root = Self::get_popup_err_elem();
        // Now set up the handlers to actually render popup errors (the scope will keep
        // reactivity going as long as it isn't dropped). Popup errors do *not*
        // get access to a router or the like. Ever time `popup_err_view` is
        // updated, this will update too.
        render_or_hydrate(
            cx,
            view! { cx,
                (*self.popup_error_view.get())
            },
            popup_error_root,
            true, // Popup errors are always browser-side-only, so force a full render
        );

        // --- Initial load ---

        // We handle the disposer for the page-wide view, without worrying about
        // widgets, because they're all in child scopes of the page scope,
        // meaning they will be automatically disposed of when the page disposer
        // is called.
        let page_disposer = PageDisposer::default();
        // Get the root we'll be injecting the router into
        let root = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(&format!("#{}", &self.root))
            .unwrap()
            .unwrap();
        // Unless we hear otherwise, we'll hydrate the main view by default (but some
        // errors should trigger a full render). This only matters for the
        // initial load, since view changes are done reactively after that.
        let mut force_render = false;
        // Get the initial load so we have something to put inside the root. Usually, we
        // can simply report errors, but, because we don't actually have a place to put
        // page-wide errors yet, we need to know what this will return so we know if we
        // should proceed.
        let (starting_view, is_err) = match self.get_initial_view(cx) {
            Ok(InitialView::View(view, disposer)) => {
                // SAFETY: There's nothing in there right now, and we know that for sure
                // because it's the initial load (asserted above). Also, we're in the app-level
                // scope.
                unsafe {
                    page_disposer.update(disposer);
                }

                // Note that the router state has already been correctly set to `Loaded`
                (view, false)
            }
            // On a redirect, return a view that just redirects straight away (of course,
            // this will be created inside a router, so everything works nicely)
            Ok(InitialView::Redirect(dest)) => {
                force_render = true;
                (
                    view! { cx,
                            ({
                                let dest = dest.clone();
                                on_mount(cx, move || {
                                    navigate_replace(&dest);
                                });
                                View::empty()
                            })
                    },
                    false,
                )
            }
            // We still need the page-wide view
            Err(err @ ClientError::ServerError { .. }) => {
                // Rather than worrying about multi-file invariants, just do the error
                // handling manually for sanity
                let (head_str, body_view, disposer) =
                    self.error_views.handle(cx, err, ErrorPosition::Page);
                replace_head(&head_str);

                // SAFETY: There's nothing in there right now, and we know that for sure
                // because it's the initial load (asserted above). Also, we're in the app-level
                // scope.
                unsafe {
                    page_disposer.update(disposer);
                }

                // For apps using exporting, it's very possible that the prerendered may be
                // unlocalized, and this may be localized. Hence, we clear the contents.
                force_render = true;
                (body_view, true)
            }
            // Popup error: we will not create a router, terminating immediately
            // and instructing the caller to dispose of the scope
            Err(err) => {
                // Rather than worrying about multi-file invariants, just do the error
                // handling manually for sanity
                let (_, body_view, _disposer) =
                    self.error_views.handle(cx, err, ErrorPosition::Popup);
                self.popup_error_view.set(body_view); // Popups never hydrate

                // Signal the top-level disposer, which will also call the child scope disposer
                // ignored above
                return false;
            }
        };
        self.current_view.set(starting_view);

        // --- Reload commander ---

        // This allows us to not run the subsequent load code on the initial load (we
        // need a separate one for the reload commander)
        let is_initial_reload_commander = create_signal(cx, true);
        let router_state = &self.router_state;
        let page_disposer_2 = page_disposer.clone();
        let popup_error_disposer_2 = popup_error_disposer.clone();
        create_effect(cx, move || {
            router_state.reload_commander.track();
            // These use `RcSignal`s, so there's still only one actual disposer for each
            let page_disposer_2 = page_disposer_2.clone();
            let popup_error_disposer_2 = popup_error_disposer_2.clone();

            // Using a tracker of the initial state separate to the main one is fine,
            // because this effect is guaranteed to fire on page load (they'll both be set)
            if *is_initial_reload_commander.get_untracked() {
                is_initial_reload_commander.set(false);
            } else {
                // Get the route verdict and re-run the function we use on route changes
                // This has to be untracked, otherwise we get an infinite loop that will
                // actually break client browsers (I had to manually kill Firefox...)
                // TODO Investigate how the heck this actually caused an infinite loop...
                let verdict = router_state.get_last_verdict();
                let verdict = match verdict {
                    Some(verdict) => verdict,
                    // If the first page hasn't loaded yet, terminate now
                    None => return,
                };
                spawn_local_scoped(cx, async move {
                    // Get the subsequent view and handle errors
                    match self.get_subsequent_view(cx, verdict.clone()).await {
                        Ok((view, disposer)) => {
                            self.current_view.set(view);
                            // SAFETY: We're outside the old page's scope
                            unsafe {
                                page_disposer_2.update(disposer);
                            }
                        }
                        Err(err) => {
                            // Any errors should be gracefully reported, and their disposers
                            // placed into the correct `Signal` for future managament
                            let (disposer, pagewide) = self.report_err(cx, err);
                            // SAFETY: We're outside the old error/page's scope
                            if pagewide {
                                unsafe {
                                    page_disposer_2.update(disposer);
                                }
                            } else {
                                unsafe {
                                    popup_error_disposer_2.clone().update(disposer);
                                }
                            }
                        }
                    };
                });
            }
        });

        // --- Router! ---

        if is_err {
            checkpoint("error");
        } else {
            checkpoint("page_interactive");
        }

        // Now set up the full router
        // let popup_error_disposer_2 = popup_error_disposer.clone();
        render_or_hydrate(
            cx,
            view! { cx,
                RouterBase(
                    integration = HistoryIntegration::new(),
                    // This will be immediately updated and fixed up
                    route = PerseusRoute {
                        // This is completely invalid, but will never be read
                        verdict: RouteVerdict::NotFound { locale: "xx-XX".to_string() },
                        cx: Some(cx),
                    },
                    view = move |cx, route: &ReadSignal<PerseusRoute>| {
                        // Do this on every update to the route, except the first time, when we'll use the initial load
                        create_effect(cx, move || {
                            route.track();
                            // These use `RcSignal`s, so there's still only one actual disposer for each
                            let page_disposer_2 = page_disposer.clone();
                            let popup_error_disposer_2 = popup_error_disposer.clone();

                            if self.is_first.get() {
                                // HSR will take care of this if it's enabled
                                #[cfg(not(all(debug_assertions, feature = "hsr")))]
                                self.is_first.set(false);
                            } else {
                                spawn_local_scoped(cx, async move {
                                    let route = route.get();
                                    let verdict = route.get_verdict();

                                    // Get the subsequent view and handle errors
                                    match self.get_subsequent_view(cx, verdict.clone()).await {
                                        Ok((view, disposer)) => {
                                            self.current_view.set(view);
                                            // SAFETY: We're outside the old page's scope
                                            unsafe { page_disposer_2.update(disposer); }
                                        }
                                        Err(err) => {
                                            // Any errors should be gracefully reported, and their disposers
                                            // placed into the correct `Signal` for future managament
                                            let (disposer, pagewide) = self.report_err(cx, err);
                                            // SAFETY: We're outside the old error/page's scope
                                            if pagewide {
                                                unsafe { page_disposer_2.update(disposer); }
                                            } else {
                                                unsafe { popup_error_disposer_2.clone().update(disposer); }
                                            }
                                        }
                                    };
                                });
                            }
                        });

                        // This template is reactive, and will be updated as necessary
                        view! { cx,
                            (*self.current_view.get())
                        }
                    }
                )
            },
            root,
            force_render, /* Depending on whether or not there's an error, we might force
                           * a full render */
        );

        // If we successfully got here, the app is running!
        true
    }

    /// Gets the element for popup errors (used in both full startup and
    /// critical failures).
    ///
    /// This is created on the engine-side to avoid hydration issues.
    pub(crate) fn get_popup_err_elem() -> Element {
        let document = web_sys::window().unwrap().document().unwrap();
        // If we can't get the error container, it's logical to panic
        document.get_element_by_id("__perseus_popup_error").unwrap()
    }
}
