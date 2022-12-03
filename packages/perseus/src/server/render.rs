use std::collections::HashMap;

use crate::errors::*;
use crate::i18n::TranslationsManager;
use crate::page_data::PageData;
use crate::state::GlobalStateCreator;
use crate::stores::{ImmutableStore, MutableStore};
use crate::template::{ArcCapsuleMap, ArcTemplateMap, StateGeneratorInfo, States, Template, TemplateMap, TemplateState, UnknownStateType, WidgetStates};
use crate::translator::Translator;
use crate::Request;
use crate::SsrNode;
use chrono::{DateTime, Utc};
use futures::FutureExt;
use futures::future::{BoxFuture, try_join_all};

/// Clones a `Request` from its internal parts.
fn clone_req(raw: &Request) -> Request {
    let mut builder = Request::builder();

    for (name, val) in raw.headers() {
        builder = builder.header(name, val);
    }

    builder
        .uri(raw.uri())
        .method(raw.method())
        .version(raw.version())
        // We always use an empty body because, in a Perseus request, only the URI matters
        // Any custom data should therefore be sent in headers (if you're doing that, consider a
        // dedicated API)
        .body(())
        .unwrap() // This should never fail...
}

/// Gets the path with the locale, returning it without if i18n isn't being
/// used.
fn get_path_with_locale(path_without_locale: &str, translator: &Translator) -> String {
    let locale = translator.get_locale();
    match locale.as_str() {
        "xx-XX" => path_without_locale.to_string(),
        locale => format!("{}/{}", locale, path_without_locale),
    }
}

/// Renders a template that uses state generated at build-time. This can't be
/// used for pages that revalidate because their data are stored in a mutable
/// store.
///
/// This returns a body, head, and state, since all are from stores.
async fn render_build_state(
    path_encoded: &str,
    immutable_store: &ImmutableStore,
    render_html: bool,
) -> Result<(String, String, TemplateState), ServerError> {
    // Get the static HTML
    let html = if render_html {
        immutable_store
            .read(&format!("static/{}.html", path_encoded))
            .await?
    } else {
        String::new()
    };
    let head = immutable_store
        .read(&format!("static/{}.head.html", path_encoded))
        .await?;
    // Get the static JSON
    let state = match immutable_store
        .read(&format!("static/{}.json", path_encoded))
        .await
    {
        Ok(state) => TemplateState::from_str(&state)
            .map_err(|err| ServerError::InvalidPageState { source: err })?,
        Err(_) => TemplateState::empty(),
    };

    Ok((html, head, state))
}
/// Renders a template that uses state generated at build-time. This is
/// specifically for page that revalidate, because they store data
/// in the mutable store.
///
/// This returns a body, head, and state, since all are from stores.
async fn render_build_state_for_mutable(
    path_encoded: &str,
    mutable_store: &impl MutableStore,
    render_html: bool,
) -> Result<(String, String, TemplateState), ServerError> {
    // Get the static HTML
    let html = if render_html {
        mutable_store
            .read(&format!("static/{}.html", path_encoded))
            .await?
    } else {
        String::new()
    };
    let head = mutable_store
        .read(&format!("static/{}.head.html", path_encoded))
        .await?;
    // Get the static JSON
    let state = match mutable_store
        .read(&format!("static/{}.json", path_encoded))
        .await
    {
        Ok(state) => TemplateState::from_str(&state)
            .map_err(|err| ServerError::InvalidPageState { source: err })?,
        Err(_) => TemplateState::empty(),
    };

    Ok((html, head, state))
}
/// Renders a template that generated its state at request-time. Note that
/// revalidation and incremental generation have no impact on SSR-rendered
/// pages. This does everything at request-time, and so doesn't need a mutable
/// or immutable store.
///
/// As this involves state computation, this only returns the state.
async fn get_request_state(
    template: &Template<SsrNode>,
    build_info: StateGeneratorInfo<UnknownStateType>,
    req: Request,
) -> Result<TemplateState, ServerError> {
    // Generate the initial state (this may generate an error, but there's no file
    // that can't exist)
    let state = template.get_request_state(build_info, req).await?;

    Ok(state)
}
/// Renders a template that wants to amalgamate build state with request state.
/// This does everything at request-time, and so doesn't need a mutable or
/// immutable store.
///
/// As this is always the final item, this returns a body and head along with
/// the state.
async fn render_amalgamated_state(
    template: &Template<SsrNode>,
    build_info: StateGeneratorInfo<UnknownStateType>,
    translator: &Translator,
    global_state: &TemplateState,
    build_state: TemplateState,
    request_state: TemplateState,
    render_html: bool,
) -> Result<(String, String, TemplateState), ServerError> {
    let path_with_locale = get_path_with_locale(&build_info.path, &translator);
    // Generate the initial state (this may generate an error, but there's no file
    // that can't exist)
    let state = template
        .amalgamate_states(build_info, build_state, request_state)
        .await?;

    let html = if render_html {
        sycamore::render_to_string(|cx| {
            template.render_for_template_server(
                path_with_locale,
                state.clone(),
                global_state.clone(),
                cx,
                translator,
            )
        })
    } else {
        String::new()
    };
    let head = template.render_head_str(state.clone(), global_state.clone(), translator);

    Ok((html, head, state))
}
/// Checks if a template that uses incremental generation has already been
/// cached. If the template was prerendered by *build paths*, then it will have
/// already been matched because those are declared verbatim in the render
/// configuration. Therefore, this function only searches for pages that have
/// been cached later, which means it needs a mutable store.
///
/// This returns a body and a head.
///
/// This accepts a `render_html` directive because, if it needed to cache
/// anything, then it would return `None`, and that's handled outside this
/// function.
async fn get_incremental_cached(
    path_encoded: &str,
    mutable_store: &impl MutableStore,
    render_html: bool,
) -> Option<(String, String)> {
    let html_res = if render_html {
        mutable_store
            .read(&format!("static/{}.html", path_encoded))
            .await
    } else {
        Ok(String::new())
    };

    // We should only treat it as cached if it can be accessed and if we aren't in
    // development (when everything should constantly reload)
    match html_res {
        Ok(html) if !cfg!(debug_assertions) => {
            // If the HTML exists, the head must as well
            let head = mutable_store
                .read(&format!("static/{}.head.html", path_encoded))
                .await
                .unwrap();
            Some((html, head))
        }
        Ok(_) | Err(_) => None,
    }
}
/// Checks if a template should revalidate by time. All revalidation timestamps
/// are stored in a mutable store, so that's what this function uses.
async fn should_revalidate(
    template: &Template<SsrNode>,
    path_encoded: &str,
    mutable_store: &impl MutableStore,
    build_info: StateGeneratorInfo<UnknownStateType>,
    req: Request,
) -> Result<bool, ServerError> {
    let mut should_revalidate = false;
    // If it revalidates after a certain period of time, we need to check that
    // BEFORE the custom logic
    if template.revalidates_with_time() {
        // Get the time when it should revalidate (RFC 3339)
        // This will be updated, so it's in a mutable store
        let datetime_to_revalidate_str = mutable_store
            .read(&format!("static/{}.revld.txt", path_encoded))
            .await?;
        let datetime_to_revalidate = DateTime::parse_from_rfc3339(&datetime_to_revalidate_str)
            .map_err(|e| {
                let serve_err: ServeError = e.into();
                serve_err
            })?;
        // Get the current time (UTC)
        let now = Utc::now();

        // If the datetime to revalidate is still in the future, end with `false`
        if datetime_to_revalidate > now {
            return Ok(false);
        }
        should_revalidate = true;
    }

    // Now run the user's custom revalidation logic
    if template.revalidates_with_logic() {
        should_revalidate = template.should_revalidate(build_info, req).await?;
    }
    Ok(should_revalidate)
}
/// Revalidates a template. All information about templates that revalidate
/// (timestamp, content, head, and state) is stored in a mutable store, so
/// that's what this function uses.
///
/// Despite this involving state computation, it needs to write a body and
/// head to the mutable store, so it returns those along with the state.
///
/// This receives no directive about not rendering content HTML, since it
/// has to for future caching anyway.
async fn revalidate(
    template: &Template<SsrNode>,
    build_info: StateGeneratorInfo<UnknownStateType>,
    translator: &Translator,
    path_encoded: &str,
    global_state: &TemplateState,
    mutable_store: &impl MutableStore,
) -> Result<(String, String, TemplateState), ServerError> {
    let path_with_locale = get_path_with_locale(&build_info.path, &translator);
    // We need to regenerate and cache this page for future usage (until the next
    // revalidation)
    let state = template.get_build_state(build_info).await?;
    let html = sycamore::render_to_string(|cx| {
        template.render_for_template_server(
            path_with_locale,
            state.clone(),
            global_state.clone(),
            cx,
            translator,
        )
    });
    let head = template.render_head_str(state.clone(), global_state.clone(), translator);
    // Handle revalidation, we need to parse any given time strings into datetimes
    // We don't need to worry about revalidation that operates by logic, that's
    // request-time only
    if template.revalidates_with_time() {
        // IMPORTANT: we set the new revalidation datetime to the interval from NOW, not
        // from the previous one So if you're revalidating many pages weekly,
        // they will NOT revalidate simultaneously, even if they're all queried thus
        let datetime_to_revalidate = template
            .get_revalidate_interval()
            .unwrap()
            .compute_timestamp();
        mutable_store
            .write(
                &format!("static/{}.revld.txt", path_encoded),
                &datetime_to_revalidate,
            )
            .await?;
    }
    mutable_store
        .write(
            &format!("static/{}.json", path_encoded),
            &state.state.to_string(),
        )
        .await?;
    mutable_store
        .write(&format!("static/{}.html", path_encoded), &html)
        .await?;
    mutable_store
        .write(&format!("static/{}.head.html", path_encoded), &head)
        .await?;

    Ok((html, head, state))
}

/// The properties required to get data for a page.
#[derive(Debug)]
pub struct GetPageProps<'a, M: MutableStore, T: TranslationsManager> {
    /// The raw path (which must not contain the locale).
    pub raw_path: &'a str,
    /// The locale to render for.
    pub locale: &'a str,
    /// Whether or not the page was matched on a template using incremental
    /// generation that didn't prerender it with build paths (these use the
    /// mutable store).
    pub was_incremental_match: bool,
    /// The request data.
    pub req: Request,
    /// The pre-built global state. If the app does not generate global state
    /// at build-time, then this will be an empty state. Importantly, we may
    /// render request-time global state, or even amalgamate that with
    /// build-time state.
    ///
    /// See `build.rs` for further details of the quirks involved in this
    /// system.
    pub global_state: &'a TemplateState,
    /// The global state creator.
    pub global_state_creator: &'a GlobalStateCreator,
    /// An immutable store.
    pub immutable_store: &'a ImmutableStore,
    /// A mutable store.
    pub mutable_store: &'a M,
    /// A translations manager.
    pub translations_manager: &'a T,
    /// A map of all the app's templates, including capsules.
    pub templates: &'a ArcTemplateMap<SsrNode>,
}

/// Internal logic behind [`get_page`]. The only differences are that this takes
/// a full template rather than just a template name, which can avoid an
/// unnecessary lookup if you already know the template in full (e.g. initial
/// load server-side routing). Because this handles templates with potentially
/// revalidation and incremental generation, it uses both mutable and immutable
/// stores.
///
/// This returns the [`PageData`] and the global state (which may have been
/// recomputed at request-time).
///
/// If `render_html` is set to `false` here, then no content HTML will be
/// generated (designed for subsequent loads), and any capsule dependencies
/// will *not* be built (state will be built in total isolation, see the book
/// for details).
///
/// This will return an error if the dependency tree for this template has not yet been resolved.
///
/// The `full_global_state` parameter should only be provided in recursions, and should be
/// `None` otherwise.
pub fn get_page_for_template<M: MutableStore, T: TranslationsManager>(
    GetPageProps {
        raw_path,
        locale,
        was_incremental_match,
        req,
        global_state: built_global_state,
        global_state_creator: gsc,
        immutable_store,
        mutable_store,
        translations_manager,
        templates,
    }: GetPageProps<'_, M, T>,
    template: &Template<SsrNode>,
    render_html: bool,
    // Whether or not the caller was this function itself
    is_recursion: bool,
    full_global_state: Option<&TemplateState>,
) -> BoxFuture<'static, Result<(PageData, TemplateState, WidgetStates), ServerError>> {
    async move {

    }.boxed()
}

// Purely internal representation
struct State {
    state: TemplateState,
    head: String,
}

/// Gets the state of a page/widget, without performing any rendering. This is intended
/// for purely internal use for recursion, and end users should use `get_page_state()`.
async fn get_state_internal<M: MutableStore, T: TranslationsManager>(
    GetPageProps {
        raw_path,
        locale,
        was_incremental_match,
        req,
        immutable_store,
        mutable_store,
        translations_manager,
        templates,
        // We ignore the global state components, since we're given the prepared global state
        global_state: _,
        global_state_creator: _,
    }: GetPageProps<'_, M, T>,
    template: &Template<SsrNode>,
    // If this is provided, we'll ignore whatever build-time global state was provided (since
    // there's no point determining the global state multiple times in a recursion)
    global_state: &TemplateState,
) -> Result<State, ServerError> {
    // Since `Request` is not actually `Clone`able, we hack our way around needing
    // it three times
    // An `Rc` won't work because of future constraints, and an `Arc`
    // seems a little unnecessary
    // TODO This is ridiculous
    let req_2 = clone_req(&req);
    let req_3 = clone_req(&req);
    // Get a translator for this locale (for sanity we hope the manager is caching)
    let translator = translations_manager
        .get_translator_for_locale(locale.to_string())
        .await?;

    // // If we're rendering HTML, we'll need to include all non-delayed widget dependencies, and therefore
    // // we need to know their states, so recursively build down the dependency tree
    // let mut widget_states = HashMap::new();
    // if is_recursion {
    //     // We need this block so the read lock is safely dropped before we spawn any futures, otherwise they aren't `Send`
    //     let resolved_deps = {
    //         let resolved_deps_raw = template.resolved_widgets.read().unwrap();
    //         if resolved_deps_raw.is_none() {
    //             return Err(ServerError::DepTreeNotResolved);
    //         }
    //         resolved_deps_raw.unwrap().clone()
    //     };
    //     let mut futs = Vec::new();
    //     for widget in template.widgets.iter() {
    //         // The resolved dependencies are built from the widgets, so this will exist
    //         let resolved = resolved_deps.get(widget).unwrap();
    //         let resolved = resolved.clone();
    //         // Similarly, this would already have failed if it didn't exist
    //         let capsule = templates.get(&resolved.capsule_name).unwrap();
    //         // Build the capsule
    //         let fut = get_page_for_template(
    //             GetPageProps {
    //                 raw_path: &widget,
    //                 locale: &resolved.locale,
    //                 was_incremental_match: resolved.was_incremental_match,
    //                 req: clone_req(&req), // TODO
    //                 global_state: built_global_state, // This will be paid no mind
    //                 global_state_creator: gsc,
    //                 immutable_store,
    //                 mutable_store,
    //                 translations_manager,
    //                 templates,
    //             },
    //             capsule,
    //             false, // No matter what the top-level caller wanted, we do NOT want to build HTML for the capsules, that will be done by the capsule HOC
    //             true, // We are recursing
    //             Some(&global_state), // To avoid double-building for request state etc.
    //         );
    //         futs.push(fut);
    //         // Now build all the dependencies in parallel
    //         // TODO Prevent double-building dependencies here with a cache (the same dependency won't ever be built in parallel unless it's double-specified)
    //         let results = try_join_all(futs).await?;
    //         for (dep_data, _, dep_widget_states) in results.into_iter() {
    //             let dep_widget_states = match dep_widget_states {
    //                 WidgetStates::Map(map) => map,
    //                 // It's this function, we know
    //                 _ => unreachable!(),
    //             };
    //             // Now add all the states generated by any further recursions that made (into its own dependencies) to our dependency map
    //             widget_states.extend(dep_widget_states);
    //             // And add all the states from the dependencies of the template/capsule we're actually building for
    //             widget_states.insert(widget.to_string(), TemplateState::from_value(dep_data.state));
    //         }
    //     }
    // }
    // // This now has the entire dependency tree in it, and can be inserted into context so that widgets can actually be built
    // let widget_states = WidgetStates::Map(widget_states);

    let path = raw_path;
    // Remove `/` from the path by encoding it as a URL (that's what we store) and
    // add the locale
    let path_encoded = format!("{}-{}", locale, urlencoding::encode(path));

    // Get the extra build data for this template (this will always exist)
    let build_extra = match immutable_store
        .read(&format!("static/{}.extra.json", template.get_path()))
        .await
    {
        Ok(state) => {
            TemplateState::from_str(&state).map_err(|err| ServerError::InvalidBuildExtra {
                template_name: template.get_path(),
                source: err,
            })?
        }
        // If this happens, then the immutable store has been tampered with, since
        // the build logic generates some kind of state for everything
        Err(_) => {
            return Err(ServerError::MissingBuildExtra {
                template_name: template.get_path(),
            })
        }
    };
    let build_info = StateGeneratorInfo {
        path: path.to_string(),
        locale: locale.to_string(),
        extra: build_extra,
    };

    let path_with_locale = get_path_with_locale(&build_info.path, &translator);

    // Only a single string of HTML is needed, and it will be overridden if
    // necessary (priorities system)
    // We might set this to something cached, and later override it
    // This will only be populated if `render_html` is `true`
    let mut html = String::new();
    // The same applies for the document metadata
    let mut head = String::new();
    // Multiple rendering strategies may need to amalgamate different states
    let mut states = States::new();

    // Handle build state (which might use revalidation or incremental)
    if template.uses_build_state() || template.is_basic() {
        // If the template uses incremental generation, that is its own contained
        // process
        if template.uses_incremental() && was_incremental_match {
            // This template uses incremental generation, and this page was built and cached
            // at runtime in the mutable store Get the cached content if it
            // exists (otherwise `None`)
            let html_and_head_opt =
                get_incremental_cached(&path_encoded, mutable_store, render_html).await;
            match html_and_head_opt {
                // It's cached
                Some((html_val, head_val)) => {
                    // Check if we need to revalidate
                    if should_revalidate(
                        template,
                        &path_encoded,
                        mutable_store,
                        build_info.clone(),
                        req,
                    )
                        .await?
                    {
                        let (html_val, head_val, state) = revalidate(
                            template,
                            build_info.clone(),
                            &translator,
                            &path_encoded,
                            &global_state,
                            mutable_store,
                        )
                            .await?;
                        // That revalidation will have returned a body and head, which we can
                        // provisionally use
                        html = html_val;
                        head = head_val;
                        states.build_state = state;
                    } else {
                        // That incremental cache check will have returned a body and head, which we
                        // can provisionally use
                        html = html_val;
                        head = head_val;
                        // Get the static JSON (if it exists, but it should)
                        // THis wouldn't be present if the user had set up incremental generation
                        // without build state (which would be remarkably silly)
                        states.build_state = match mutable_store
                            .read(&format!("static/{}.json", path_encoded))
                            .await
                        {
                            Ok(state) => TemplateState::from_str(&state)
                                .map_err(|err| ServerError::InvalidPageState { source: err })?,
                            Err(_) => TemplateState::empty(),
                        };
                    }
                }
                // It's not cached
                // All this uses the mutable store because this will be done at runtime
                None => {
                    // We need to generate and cache this page for future usage (even if
                    // `render_html` is `false`) Even if we're going to
                    // amalgamate later, we still have to perform incremental
                    // caching, which means a potentially unnecessary page build
                    let state = template.get_build_state(build_info.clone()).await?;
                    let html_val = sycamore::render_to_string(|cx| {
                        template.render_for_template_server(
                            path_with_locale.clone(),
                            state.clone(),
                            global_state.clone(),
                            cx,
                            &translator,
                        )
                    });
                    let head_val =
                        template.render_head_str(state.clone(), global_state.clone(), &translator);
                    // Handle revalidation, we need to parse any given time strings into datetimes
                    // We don't need to worry about revalidation that operates by logic, that's
                    // request-time only Obviously we don't need to revalidate
                    // now, we just created it
                    if template.revalidates_with_time() {
                        let datetime_to_revalidate = template
                            .get_revalidate_interval()
                            .unwrap()
                            .compute_timestamp();
                        // Write that to a static file, we'll update it every time we revalidate
                        // Note that this runs for every path generated, so it's fully usable with
                        // ISR
                        mutable_store
                            .write(
                                &format!("static/{}.revld.txt", path_encoded),
                                &datetime_to_revalidate,
                            )
                            .await?;
                    }
                    // Cache all that
                    mutable_store
                        .write(
                            &format!("static/{}.json", path_encoded),
                            &state.state.to_string(),
                        )
                        .await?;
                    // Write that prerendered HTML to a static file
                    mutable_store
                        .write(&format!("static/{}.html", path_encoded), &html_val)
                        .await?;
                    mutable_store
                        .write(&format!("static/{}.head.html", path_encoded), &head_val)
                        .await?;

                    states.build_state = state;
                    html = html_val;
                    head = head_val;
                }
            }
        } else {
            // If we're here, incremental generation is either not used or it's irrelevant
            // because the page was rendered in the immutable store at build time

            // Handle if we need to revalidate
            // It'll be in the mutable store if we do
            if should_revalidate(
                template,
                &path_encoded,
                mutable_store,
                build_info.clone(),
                req,
            )
                .await?
            {
                let (html_val, head_val, state) = revalidate(
                    template,
                    build_info.clone(),
                    &translator,
                    &path_encoded,
                    &global_state,
                    mutable_store,
                )
                    .await?;
                // That revalidation will have produced a head and body, which we can
                // provisionally use
                html = html_val;
                head = head_val;
                states.build_state = state;
            } else if template.revalidates() {
                // The template does revalidate, but it doesn't need to revalidate now
                // Nonetheless, its data will be the mutable store
                // This is just fetching, not computing
                let (html_val, head_val, state) =
                    render_build_state_for_mutable(&path_encoded, mutable_store, render_html)
                    .await?;
                html = html_val;
                head = head_val;
                states.build_state = state;
            } else {
                // If we don't need to revalidate and this isn't an incrementally generated
                // template, everything is immutable
                // Again, this just fetches
                let (html_val, head_val, state) =
                    render_build_state(&path_encoded, immutable_store, render_html).await?;
                html = html_val;
                head = head_val;
                states.build_state = state;
            }
        }
    }
    // Handle request state
    if template.uses_request_state() {
        // Because this never needs to write to a file or the like, this just generates
        // the state We can therefore avoid an unnecessary page build in
        // templates with state amalgamation If we're using amalgamation, the
        // page will be built soon If we're not, and there's no build state,
        // then we still need to build, which we'll do after we've checked for
        // amalgamation
        let state = get_request_state(template, build_info.clone(), req_2).await?;
        states.request_state = state;
    }

    // Amalgamate the states
    // If the user has defined custom logic for this, we'll defer to that
    // Otherwise, request trumps build
    // Of course, if only one state was defined, we'll just use that regardless
    //
    // If we're not using amalgamation, and only the request state is defined, then
    // we still need to build the page. We don't do that earlier so we can avoid
    // double-building with amalgamation.
    let state = if !states.both_defined() && template.uses_request_state() {
        // If we only have one state, and it's from request time, then we need to build
        // the template with it now
        let state = states.get_defined()?;

        let head_val = template.render_head_str(state.clone(), global_state.clone(), &translator);
        head = head_val;
        // We should only render the HTML if necessary, since we're not caching
        if render_html {
            let html_val = sycamore::render_to_string(|cx| {
                template.render_for_template_server(
                    path_with_locale,
                    state.clone(),
                    global_state.clone(),
                    cx,
                    &translator,
                )
            });
            html = html_val;
        }
        state
    } else if !states.both_defined() {
        // If we only have one state, and it's not from request time, then we've already
        // built. If there is any state at all, this will be `Some(state)`,
        // otherwise `None` (if this template doesn't take state)
        states.get_defined()?
    } else if template.can_amalgamate_states() {
        // We know that both the states are defined
        // The HTML is currently built with the wrong state, so we have to update it
        let (html_val, head_val, state) = render_amalgamated_state(
            template,
            build_info,
            &translator,
            &global_state,
            states.build_state,
            states.request_state,
            render_html,
        )
            .await?;
        html = html_val;
        head = head_val;
        state
    } else {
        // We do have multiple states, but there's no resolution function, so we have to
        // prefer request state
        // That means we have to build the page for it,
        // since we haven't yet
        let state = states.request_state;
        let head_val = template.render_head_str(state.clone(), global_state.clone(), &translator);
        // We should only render the HTML if necessary, since we're not caching
        if render_html {
            let html_val = sycamore::render_to_string(|cx| {
                template.render_for_template_server(
                    path_with_locale,
                    state.clone(),
                    global_state.clone(),
                    cx,
                    &translator,
                )
            });
            html = html_val;
        }
        head = head_val;
        state
    };

    // Combine everything into one JSON object
    // If we aren't rendering content HTML, then we won't even bother including it
    // (since it could actually have something in it, particularly from
    // revalidation/incremental generation, which generates regardless for
    // caching)
    let res = if render_html {
        PageData {
            content: html,
            state: state.state,
            head,
        }
    } else {
        PageData {
            content: String::new(),
            state: state.state,
            head,
        }
    };

    Ok((res, global_state))
}

/// Gets the full global state from the state generated at build-time and the generator itself.
///
/// This should only be called once per API call.
async fn get_full_global_state<'a>(
    built_state: &'a TemplateState,
    gsc: &'a GlobalStateCreator,
    locale: &'a str,
    req: Request,
) -> Result<TemplateState, ServerError> {
    let global_state = if gsc.uses_request_state() {
        let req_state = gsc.get_request_state(locale.to_string(), req).await?;
        // If we have a non-empty build-time state, we'll need to amalgamate
        if !built_state.is_empty() {
            if gsc.can_amalgamate_states() {
                gsc.amalgamate_states(locale.to_string(), built_state.clone(), req_state)
                   .await?
            } else {
                // No amalgamation capability, request time state takes priority
                req_state
            }
        } else {
            req_state
        }
    } else {
        // This global state is purely generated at build-time (or nonexistent)
        built_state.clone()
    };

    Ok(global_state)
}

/// Gets the state of a page/widget, without performing any rendering.
///
/// This returns a tuple of the page state (with empty contents) and the
/// full global state which may have been updated from what was provided
/// (which was from build-time). Note that this is *all* request-specific.
pub async fn get_state<M: MutableStore, T: TranslationsManager>(
    props: GetPageProps<'_, M, T>,
    template: &Template<SsrNode>,
) -> Result<(PageData, TemplateState), ServerError> {
    // Get the up-to-date global state
    let global_state = get_full_global_state(
        props.global_state,
        props.global_state_creator,
        props.locale,
        clone_req(&props.req),
    ).await?;

    let state = get_state_internal(props, template, &global_state).await?;
    Ok((PageData {
        head: state.head,
        state: state.state.state,
        content: String::new(),
    }, global_state))
}

/// Gets the state and contents of a page/widget, without performing any rendering.
///
/// This returns a tuple of the page state and the
/// full global state which may have been updated from what was provided
/// (which was from build-time). Note that this is *all* request-specific.
///
/// When rendering contents, we need to resolve all the underlying widgets,
/// which involves a substantial degree of recursion. If there are infinite dependency
/// loops, they may not be detected until this function call!
pub async fn get_page_full<M: MutableStore, T: TranslationsManager>(
    props: GetPageProps<'_, M, T>,
    template: &Template<SsrNode>,
) -> Result<(PageData, TemplateState), ServerError> {
    // Get the up-to-date global state
    let global_state = get_full_global_state(
        props.global_state,
        props.global_state_creator,
        props.locale,
        clone_req(&props.req),
    ).await?;

    // Check if there's a

    // The state/head is not dependent on any widgets, the contents are
    let state = get_page_state_internal(props, template, &global_state).await?;

    todo!()
}

// /// Gets the HTML/JSON data for the given page path. This will call
// /// SSG/SSR/etc., whatever is needed for that page.
// ///
// /// This returns the [`PageData`] and the global state (which may have been
// /// recomputed at request-time).
// pub async fn get_page<M: MutableStore, T: TranslationsManager>(
//     props: GetPageProps<'_, M, T>,
//     template_name: &str,
//     templates: &TemplateMap<SsrNode>,
//     render_html: bool,
// ) -> Result<(PageData, TemplateState), ServerError> {
//     let path = props.raw_path;
//     // Get the template to use
//     let template = templates.get(template_name);
//     let template = match template {
//         Some(template) => template,
//         // This shouldn't happen because the client should already have performed checks against the
//         // render config, but it's handled anyway
//         None => {
//             return Err(ServeError::PageNotFound {
//                 path: path.to_string(),
//             }
//             .into())
//         }
//     };

//     // We provide no built global state here because we aren't recursing
//     let res = get_page_for_template(props, template, render_html, false, None).await?;
//     // We don't need to know the states of any underlying dependencies
//     Ok((res.0, res.1))
// }
