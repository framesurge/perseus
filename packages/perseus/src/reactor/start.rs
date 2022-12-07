use sycamore::web::Html;

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

impl<G: Html> Reactor<G> {
    /// Sets the handlers necessary to run the event-driven components of Perseus
    /// (in a reactive web framework, there are quite a few of these). This should
    /// only be executed at the beginning of the browser-side instantiation.
    ///
    /// This takes the app-level scope.
    pub(crate) fn set_handlers(cx: Scope) -> Result<(), ClientError> {
        // We must be in the first load
        assert!(*self.is_first.get(), "attempted to instantiate perseus handlers after first load");

        // Create a derived state for the route announcement
        // We do this with an effect because we only want to update in some cases (when
        // the new page is actually loaded) We also need to know if it's the first
        // page (because we don't want to announce that, screen readers will get that
        // one right)
        let route_announcement = create_signal(cx, String::new());
        // This is not whether the first page has been loaded or not, it's whether or not we're still on it
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

        // This allows us to not run the subsequent load code on the initial load (we
        // need a separate one for the reload commander)
        let is_initial_reload_commander = create_signal(cx, true);

        let router_state = &render_ctx.router;
        create_effect(cx, move || {
            router_state.reload_commander.track();
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
                    // TODO
                    set_view(cx, route_manager, verdict.clone()).await;
                });
            }
        });

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
            crate::spawn_local_scoped(cx, async move {
                match live_reload_rx.await {
                    // This will trigger only once, and then can't be used again
                    // That shouldn't be a problem, because we'll reload immediately
                    Ok(_) => {
                        #[cfg(all(feature = "hsr"))]
                        {
                            let frozen_state = render_ctx.freeze();
                            crate::state::hsr_freeze(frozen_state).await;
                        }
                        crate::state::force_reload();
                        // We shouldn't ever get here unless there was an error, the
                        // entire page will be fully reloaded
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
            crate::spawn_local_scoped(cx, async move {
                // We need to make sure we don't run this more than once, because that would
                // lead to a loop It also shouldn't run on any pages after the
                // initial load
                if self.is_first.get() {
                    self.is_first.set(false);
                    crate::state::hsr_thaw(&render_ctx).await;
                }
            });
        };

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

        Ok(())
    }
}
