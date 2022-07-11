use crate::errors::*;
use crate::i18n::TranslationsManager;
use crate::page_data::PageData;
use crate::stores::{ImmutableStore, MutableStore};
use crate::template::{PageProps, States, Template, TemplateMap};
use crate::translator::Translator;
use crate::Request;
use crate::SsrNode;
use chrono::{DateTime, Utc};

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
async fn render_build_state(
    path_encoded: &str,
    immutable_store: &ImmutableStore,
) -> Result<(String, String, Option<String>), ServerError> {
    // Get the static HTML
    let html = immutable_store
        .read(&format!("static/{}.html", path_encoded))
        .await?;
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
async fn render_build_state_for_mutable(
    path_encoded: &str,
    mutable_store: &impl MutableStore,
) -> Result<(String, String, Option<String>), ServerError> {
    // Get the static HTML
    let html = mutable_store
        .read(&format!("static/{}.html", path_encoded))
        .await?;
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
async fn render_request_state(
    template: &Template<SsrNode>,
    translator: &Translator,
    path: &str,
    global_state: &Option<String>,
    req: Request,
) -> Result<(String, String, Option<String>), ServerError> {
    let path_with_locale = get_path_with_locale(path, translator);
    // Generate the initial state (this may generate an error, but there's no file
    // that can't exist)
    let state = Some(
        template
            .get_request_state(path.to_string(), translator.get_locale(), req)
            .await?,
    );
    // Assemble the page properties
    let page_props = PageProps {
        path: path_with_locale,
        state: state.clone(),
        global_state: global_state.clone(),
    };
    // Use that to render the static HTML
    let html = sycamore::render_to_string(|cx| {
        template.render_for_template_server(page_props.clone(), cx, translator)
    });
    let head = template.render_head_str(page_props, translator);

    Ok((html, head, state))
}
/// Checks if a template that uses incremental generation has already been
/// cached. If the template was prerendered by *build paths*, then it will have
/// already been matched because those are declared verbatim in the render
/// configuration. Therefore, this function only searches for pages that have
/// been cached later, which means it needs a mutable store.
async fn get_incremental_cached(
    path_encoded: &str,
    mutable_store: &impl MutableStore,
) -> Option<(String, String)> {
    let html_res = mutable_store
        .read(&format!("static/{}.html", path_encoded))
        .await;

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
        should_revalidate = template.should_revalidate().await?;
    }
    Ok(should_revalidate)
}
/// Revalidates a template. All information about templates that revalidate
/// (timestamp, content, head, and state) is stored in a mutable store, so
/// that's what this function uses.
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
        // IMPORTANT: we set the new revalidation datetime to the interval from NOW, not from the previous one
        // So if you're revalidating many pages weekly, they will NOT revalidate simultaneously, even if they're all queried thus
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
// TODO possible further optimizations on this for futures?
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
) -> Result<PageData, ServerError> {
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
            let html_and_head_opt = get_incremental_cached(&path_encoded, mutable_store).await;
            match html_and_head_opt {
                // It's cached
                Some((html_val, head_val)) => {
                    // Check if we need to revalidate
                    if should_revalidate(template, &path_encoded, mutable_store).await? {
                        let (html_val, head_val, state) = revalidate(
                            template,
                            &translator,
                            path,
                            &path_encoded,
                            global_state,
                            mutable_store,
                        )
                        .await?;
                        // Build-time generated HTML is the lowest priority, so we'll only set it if
                        // nothing else already has
                        if html.is_empty() {
                            html = html_val;
                            head = head_val;
                        }
                        states.build_state = state;
                    } else {
                        // Build-time generated HTML is the lowest priority, so we'll only set it if
                        // nothing else already has
                        if html.is_empty() {
                            html = html_val;
                            head = head_val;
                        }
                        // Get the static JSON (if it exists, but it should)
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
                    // We need to generate and cache this page for future usage
                    let state = Some(
                        template
                            .get_build_state(path.to_string(), locale.to_string())
                            .await?,
                    );
                    // Assemble the page properties
                    let page_props = PageProps {
                        path: path_with_locale,
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
                    // Build-time generated HTML is the lowest priority, so we'll only set it if
                    // nothing else already has
                    if html.is_empty() {
                        html = html_val;
                        head = head_val;
                    }
                }
            }
        } else {
            // If we're here, incremental generation is either not used or it's irrelevant
            // because the page was rendered in the immutable store at build time

            // Handle if we need to revalidate
            // It'll be in the mutable store if we do
            if should_revalidate(template, &path_encoded, mutable_store).await? {
                let (html_val, head_val, state) = revalidate(
                    template,
                    &translator,
                    path,
                    &path_encoded,
                    global_state,
                    mutable_store,
                )
                .await?;
                // Build-time generated HTML is the lowest priority, so we'll only set it if
                // nothing else already has
                if html.is_empty() {
                    html = html_val;
                    head = head_val;
                }
                states.build_state = state;
            } else if template.revalidates() {
                // The template does revalidate, but it doesn't need to revalidate now
                // Nonetheless, its data will be the mutable store
                let (html_val, head_val, state) =
                    render_build_state_for_mutable(&path_encoded, mutable_store).await?;
                // Build-time generated HTML is the lowest priority, so we'll only set it if
                // nothing else already has
                if html.is_empty() {
                    html = html_val;
                    head = head_val;
                }
                states.build_state = state;
            } else {
                // If we don't need to revalidate and this isn't an incrementally generated
                // template, everything is immutable
                let (html_val, head_val, state) =
                    render_build_state(&path_encoded, immutable_store).await?;
                // Build-time generated HTML is the lowest priority, so we'll only set it if
                // nothing else already has
                if html.is_empty() {
                    html = html_val;
                    head = head_val;
                }
                states.build_state = state;
            }
        }
    }
    // Handle request state
    if template.uses_request_state() {
        let (html_val, head_val, state) =
            render_request_state(template, &translator, path, global_state, req).await?;
        // Request-time HTML always overrides anything generated at build-time or
        // incrementally (this has more information)
        html = html_val;
        head = head_val;
        states.request_state = state;
    }

    // Amalgamate the states
    // If the user has defined custom logic for this, we'll defer to that
    // Otherwise we go as with HTML, request trumps build
    // Of course, if only one state was defined, we'll just use that regardless (so
    // `None` prioritization is impossible) If this is the case, the build
    // content will still be served, and then it's up to the client to hydrate it
    // with the new amalgamated state
    let state = if !states.both_defined() {
        states.get_defined()?
    } else if template.can_amalgamate_states() {
        template.amalgamate_states(states)?
    } else {
        states.request_state
    };

    // Combine everything into one JSON object
    let res = PageData {
        content: html,
        state,
        head,
    };

    Ok(res)
}

/// Gets the HTML/JSON data for the given page path. This will call
/// SSG/SSR/etc., whatever is needed for that page. Note that HTML generated at
/// request-time will **always** replace anything generated at build-time,
/// incrementally, revalidated, etc.
pub async fn get_page<M: MutableStore, T: TranslationsManager>(
    props: GetPageProps<'_, M, T>,
    template_name: &str,
    templates: &TemplateMap<SsrNode>,
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

    let res = get_page_for_template(props, template).await?;
    Ok(res)
}
