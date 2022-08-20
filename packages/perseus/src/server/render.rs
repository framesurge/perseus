use crate::errors::*;
use crate::i18n::TranslationsManager;
use crate::page_data::PageData;
use crate::stores::{ImmutableStore, MutableStore};
use crate::template::{PageProps, States, Template, TemplateMap};
use crate::translator::Translator;
use crate::Request;
use crate::SsrNode;
use chrono::{DateTime, Utc};

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
) -> Result<(String, String, Option<String>), ServerError> {
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
        Ok(state) => Some(state),
        Err(_) => None,
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
) -> Result<(String, String, Option<String>), ServerError> {
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
        Ok(state) => Some(state),
        Err(_) => None,
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
    translator: &Translator,
    path: &str,
    req: Request,
) -> Result<Option<String>, ServerError> {
    // Generate the initial state (this may generate an error, but there's no file
    // that can't exist)
    let state = Some(
        template
            .get_request_state(path.to_string(), translator.get_locale(), req)
            .await?,
    );

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
    translator: &Translator,
    path: &str,
    global_state: &Option<String>,
    build_state: String,
    request_state: String,
    render_html: bool,
) -> Result<(String, String, Option<String>), ServerError> {
    let path_with_locale = get_path_with_locale(path, translator);
    // Generate the initial state (this may generate an error, but there's no file
    // that can't exist)
    let state = Some(
        template
            .amalgamate_states(
                path.to_string(),
                translator.get_locale(),
                build_state,
                request_state,
            )
            .await?,
    );

    // Assemble the page properties
    let page_props = PageProps {
        path: path_with_locale,
        state: state.clone(),
        global_state: global_state.clone(),
    };
    let html = if render_html {
        sycamore::render_to_string(|cx| {
            template.render_for_template_server(page_props.clone(), cx, translator)
        })
    } else {
        String::new()
    };
    let head = template.render_head_str(page_props, translator);

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
    translator: &Translator,
    path: &str,
    req: Request,
) -> Result<bool, ServerError> {
    let mut should_revalidate = false;
    // If it revalidates after a certain period of time, we needd to check that
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
        should_revalidate = template
            .should_revalidate(path.to_string(), translator.get_locale(), req)
            .await?;
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
    translator: &Translator,
    path: &str,
    path_encoded: &str,
    global_state: &Option<String>,
    mutable_store: &impl MutableStore,
) -> Result<(String, String, Option<String>), ServerError> {
    let path_with_locale = get_path_with_locale(path, translator);
    // We need to regenerate and cache this page for future usage (until the next
    // revalidation)
    let state = Some(
        template
            .get_build_state(
                format!("{}/{}", template.get_path(), path),
                translator.get_locale(),
            )
            .await?,
    );
    // Assemble the page properties
    let page_props = PageProps {
        path: path_with_locale,
        state: state.clone(),
        global_state: global_state.clone(),
    };
    let html = sycamore::render_to_string(|cx| {
        template.render_for_template_server(page_props.clone(), cx, translator)
    });
    let head = template.render_head_str(page_props, translator);
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
            &state.clone().unwrap(),
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
    /// The stringified global state to use in the render process.
    pub global_state: &'a Option<String>,
    /// An immutable store.
    pub immutable_store: &'a ImmutableStore,
    /// A mutable store.
    pub mutable_store: &'a M,
    /// A translations manager.
    pub translations_manager: &'a T,
}

/// Internal logic behind [`get_page`]. The only differences are that this takes
/// a full template rather than just a template name, which can avoid an
/// unnecessary lookup if you already know the template in full (e.g. initial
/// load server-side routing). Because this handles templates with potentially
/// revalidation and incremental generation, it uses both mutable and immutable
/// stores.
///
/// If `render_html` is set to `false` here, then no content HTML will be
/// generated (designed for subsequent loads).
pub async fn get_page_for_template<M: MutableStore, T: TranslationsManager>(
    GetPageProps {
        raw_path,
        locale,
        was_incremental_match,
        req,
        global_state,
        immutable_store,
        mutable_store,
        translations_manager,
    }: GetPageProps<'_, M, T>,
    template: &Template<SsrNode>,
    render_html: bool,
) -> Result<PageData, ServerError> {
    // Since `Request` is not actually `Clone`able, we hack our way around needing
    // it twice An `Rc` won't work because of future constraints, and an `Arc`
    // seems a little unnecessary
    let req_2 = clone_req(&req);
    // Get a translator for this locale (for sanity we hope the manager is caching)
    let translator = translations_manager
        .get_translator_for_locale(locale.to_string())
        .await?;

    let mut path = raw_path;
    // If the path is empty, we're looking for the special `index` page
    if path.is_empty() {
        path = "index";
    }
    // Remove `/` from the path by encoding it as a URL (that's what we store) and
    // add the locale
    let path_encoded = format!("{}-{}", locale, urlencoding::encode(path));
    let path_with_locale = get_path_with_locale(path, &translator);

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
                        &translator,
                        path,
                        req,
                    )
                    .await?
                    {
                        let (html_val, head_val, state) = revalidate(
                            template,
                            &translator,
                            path,
                            &path_encoded,
                            global_state,
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
                            Ok(state) => Some(state),
                            Err(_) => None,
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
                    let state = Some(
                        template
                            .get_build_state(path.to_string(), locale.to_string())
                            .await?,
                    );
                    // Assemble the page properties
                    let page_props = PageProps {
                        path: path_with_locale.clone(),
                        state: state.clone(),
                        global_state: global_state.clone(),
                    };
                    let html_val = sycamore::render_to_string(|cx| {
                        template.render_for_template_server(page_props.clone(), cx, &translator)
                    });
                    let head_val = template.render_head_str(page_props, &translator);
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
                            &state.clone().unwrap(),
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
                &translator,
                path,
                req,
            )
            .await?
            {
                let (html_val, head_val, state) = revalidate(
                    template,
                    &translator,
                    path,
                    &path_encoded,
                    global_state,
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
        let state = get_request_state(template, &translator, path, req_2).await?;
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

        // Assemble the page properties
        let page_props = PageProps {
            path: path_with_locale,
            state: state.clone(),
            global_state: global_state.clone(),
        };
        let head_val = template.render_head_str(page_props.clone(), &translator);
        head = head_val;
        // We should only render the HTML if necessary, since we're not caching
        if render_html {
            let html_val = sycamore::render_to_string(|cx| {
                template.render_for_template_server(page_props, cx, &translator)
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
            &translator,
            path,
            global_state,
            states.build_state.unwrap(),
            states.request_state.unwrap(),
            render_html,
        )
        .await?;
        html = html_val;
        head = head_val;
        state
    } else {
        // We do have multiple states, but there's no resolution function, so we have to
        // prefer request state That means we have to build the page for it,
        // since we haven't yet
        let state = states.request_state;
        // Assemble the page properties
        let page_props = PageProps {
            path: path_with_locale,
            state: state.clone(),
            global_state: global_state.clone(),
        };
        let head_val = template.render_head_str(page_props.clone(), &translator);
        // We should only render the HTML if necessary, since we're not caching
        if render_html {
            let html_val = sycamore::render_to_string(|cx| {
                template.render_for_template_server(page_props, cx, &translator)
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
            state,
            head,
        }
    } else {
        PageData {
            content: String::new(),
            state,
            head,
        }
    };

    Ok(res)
}

/// Gets the HTML/JSON data for the given page path. This will call
/// SSG/SSR/etc., whatever is needed for that page.
pub async fn get_page<M: MutableStore, T: TranslationsManager>(
    props: GetPageProps<'_, M, T>,
    template_name: &str,
    templates: &TemplateMap<SsrNode>,
    render_html: bool,
) -> Result<PageData, ServerError> {
    let mut path = props.raw_path;
    // If the path is empty, we're looking for the special `index` page
    if path.is_empty() {
        path = "index";
    }
    // Get the template to use
    let template = templates.get(template_name);
    let template = match template {
        Some(template) => template,
        // This shouldn't happen because the client should already have performed checks against the
        // render config, but it's handled anyway
        None => {
            return Err(ServeError::PageNotFound {
                path: path.to_string(),
            }
            .into())
        }
    };

    let res = get_page_for_template(props, template, render_html).await?;
    Ok(res)
}
