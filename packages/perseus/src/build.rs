// This binary builds all the templates with SSG

use crate::errors::*;
use crate::i18n::{Locales, TranslationsManager};
use crate::state::GlobalStateCreator;
use crate::stores::{ImmutableStore, MutableStore};
use crate::template::ArcTemplateMap;
use crate::template::{BuildPaths, StateGeneratorInfo, Template, TemplateState};
use crate::translator::Translator;
use crate::utils::minify;
use futures::future::try_join_all;
use std::collections::HashMap;
use sycamore::prelude::SsrNode;

/// Builds a template, writing static data as appropriate. This should be used
/// as part of a larger build process. This returns both a list of the extracted
/// render options for this template (needed at request time), a list of pages
/// that it explicitly generated, and a boolean as to whether or not it only
/// generated a single page to occupy the template's root path (`true` unless
/// using using build-time path generation).
pub async fn build_template(
    template: &Template<SsrNode>,
    translator: &Translator,
    (immutable_store, mutable_store): (&ImmutableStore, &impl MutableStore),
    global_state: &TemplateState,
    exporting: bool,
) -> Result<(Vec<String>, bool), ServerError> {
    let mut single_page = false;
    let template_path = template.get_path();

    // If we're exporting, ensure that all the template's strategies are export-safe
    // (not requiring a server)
    if exporting
        && (template.revalidates() ||
        template.uses_incremental() ||
        template.uses_request_state() ||
        // We check amalgamation as well because it involves request state, even if that wasn't provided
        template.can_amalgamate_states())
    {
        return Err(ExportError::TemplateNotExportable {
            template_name: template_path.clone(),
        }
        .into());
    }

    // Handle static path generation
    // Because we iterate over the paths, we need a base path if we're not
    // generating custom ones (that'll be overridden if needed)
    let (paths, build_extra) = match template.uses_build_paths() {
        true => {
            let BuildPaths { paths, extra } = template.get_build_paths().await?;
            // Trim away any trailing `/`s so we don't insert them into the render config
            // That makes rendering an index page from build paths impossible (see #39)
            let paths = paths
                .iter()
                .map(|p| match p.strip_suffix('/') {
                    Some(stripped) => stripped.to_string(),
                    None => p.to_string(),
                })
                .collect();
            (paths, extra)
        }
        false => {
            single_page = true;
            (vec![String::new()], TemplateState::empty())
        }
    };

    // Write the extra build state information to a file now so it can be accessed
    // by request state handlers and the like down the line
    immutable_store
        .write(
            &format!("static/{}.extra.json", template_path),
            &build_extra.state.to_string(),
        )
        .await?;

    // Iterate through the paths to generate initial states if needed
    // Note that build paths pages on incrementally generable pages will use the
    // immutable store
    let mut futs = Vec::new();
    for path in paths.iter() {
        let fut = gen_state_for_path(
            path,
            template,
            translator,
            (immutable_store, mutable_store),
            global_state,
            &build_extra,
        );
        futs.push(fut);
    }
    try_join_all(futs).await?;

    Ok((paths, single_page))
}

/// Generates state for a single page within a template. This is broken out into
/// a separate function for concurrency.
async fn gen_state_for_path(
    path: &str,
    template: &Template<SsrNode>,
    translator: &Translator,
    (immutable_store, mutable_store): (&ImmutableStore, &impl MutableStore),
    global_state: &TemplateState,
    build_extra: &TemplateState,
) -> Result<(), ServerError> {
    let template_path = template.get_path();
    // If needed, we'll construct a full path that's URL encoded so we can easily
    // save it as a file
    let full_path_without_locale = match template.uses_build_paths() {
        true => format!("{}/{}", &template_path, path),
        // We don't want to concatenate the name twice if we don't have to
        false => template_path.clone(),
    };
    // Strip leading/trailing `/`s for the reasons described above
    // Leading is to handle index pages with build paths
    let full_path_without_locale = match full_path_without_locale.strip_suffix('/') {
        Some(stripped) => stripped.to_string(),
        None => full_path_without_locale,
    };
    let full_path_without_locale = match full_path_without_locale.strip_prefix('/') {
        Some(stripped) => stripped.to_string(),
        None => full_path_without_locale,
    };
    // Add the current locale to the front of that and encode it as a URL so we can
    // store a flat series of files BUG: insanely nested paths won't work
    // whatsoever if the filename is too long, maybe hash instead?
    let full_path_encoded = format!(
        "{}-{}",
        translator.get_locale(),
        urlencoding::encode(&full_path_without_locale)
    );
    // And we'll need the full path with the locale for the `PageProps`
    // If it's `xx-XX`, we should just have it without the locale (this may be
    // interacted with by users)
    let locale = translator.get_locale();
    let full_path_with_locale = match locale.as_str() {
        "xx-XX" => full_path_without_locale.clone(),
        locale => format!("{}/{}", locale, &full_path_without_locale),
    };

    let build_info = StateGeneratorInfo {
        path: full_path_without_locale.clone(),
        locale: translator.get_locale(),
        extra: build_extra.clone(),
    };

    // Handle static initial state generation
    // We'll only write a static state if one is explicitly generated
    // If the template revalidates, use a mutable store, otherwise use an immutable
    // one
    if template.uses_build_state() && template.revalidates() {
        // We pass in the path to get a state (including the template path for
        // consistency with the incremental logic)
        let initial_state = template.get_build_state(build_info).await?;
        // Write that initial state to a static JSON file
        mutable_store
            .write(
                &format!("static/{}.json", full_path_encoded),
                &initial_state.state.to_string(),
            )
            .await?;
        // Prerender the template using that state
        let prerendered = sycamore::render_to_string(|cx| {
            template.render_for_template_server(
                full_path_with_locale.clone(),
                initial_state.clone(),
                global_state.clone(),
                cx,
                translator,
            )
        });
        minify(&prerendered, true)?;
        // Write that prerendered HTML to a static file
        mutable_store
            .write(&format!("static/{}.html", full_path_encoded), &prerendered)
            .await?;
        // Prerender the document `<head>` with that state
        // If the page also uses request state, amalgamation will be applied as for the
        // normal content
        let head_str = template.render_head_str(initial_state, global_state.clone(), translator);
        minify(&head_str, true)?;
        mutable_store
            .write(
                &format!("static/{}.head.html", full_path_encoded),
                &head_str,
            )
            .await?;
    } else if template.uses_build_state() {
        // We pass in the path to get a state (including the template path for
        // consistency with the incremental logic)
        let initial_state = template.get_build_state(build_info).await?;
        // Write that initial state to a static JSON file
        immutable_store
            .write(
                &format!("static/{}.json", full_path_encoded),
                &initial_state.state.to_string(),
            )
            .await?;
        // Prerender the template using that state
        let prerendered = sycamore::render_to_string(|cx| {
            template.render_for_template_server(
                full_path_with_locale.clone(),
                initial_state.clone(),
                global_state.clone(),
                cx,
                translator,
            )
        });
        minify(&prerendered, true)?;
        // Write that prerendered HTML to a static file
        immutable_store
            .write(&format!("static/{}.html", full_path_encoded), &prerendered)
            .await?;
        // Prerender the document `<head>` with that state
        // If the page also uses request state, amalgamation will be applied as for the
        // normal content
        let head_str = template.render_head_str(initial_state, global_state.clone(), translator);
        immutable_store
            .write(
                &format!("static/{}.head.html", full_path_encoded),
                &head_str,
            )
            .await?;
    }

    // Handle revalidation, we need to parse any given time strings into datetimes
    // We don't need to worry about revalidation that operates by logic, that's
    // request-time only
    if template.revalidates_with_time() {
        let datetime_to_revalidate = template
            .get_revalidate_interval()
            .unwrap()
            .compute_timestamp();
        // Write that to a static file, we'll update it every time we revalidate
        // Note that this runs for every path generated, so it's fully usable with ISR
        // Yes, there's a different revalidation schedule for each locale, but that
        // means we don't have to rebuild every locale simultaneously
        mutable_store
            .write(
                &format!("static/{}.revld.txt", full_path_encoded),
                &datetime_to_revalidate.to_string(),
            )
            .await?;
    }

    // Note that SSR has already been handled by checking for
    // `.uses_request_state()` above, we don't need to do any rendering here
    // If a template only uses SSR, it won't get prerendered at build time
    // whatsoever

    // If the template is very basic, prerender without any state
    // It's safe to add a property to the render options here because `.is_basic()`
    // will only return true if path generation is not being used (or anything else)
    if template.is_basic() {
        let prerendered = sycamore::render_to_string(|cx| {
            template.render_for_template_server(
                full_path_with_locale,
                TemplateState::empty(),
                global_state.clone(),
                cx,
                translator,
            )
        });
        let head_str =
            template.render_head_str(TemplateState::empty(), global_state.clone(), translator);
        // Write that prerendered HTML to a static file
        immutable_store
            .write(&format!("static/{}.html", full_path_encoded), &prerendered)
            .await?;
        immutable_store
            .write(
                &format!("static/{}.head.html", full_path_encoded),
                &head_str,
            )
            .await?;
    }

    Ok(())
}

/// Builds all pages within a template and compiles its component of the render
/// configuration.
pub async fn build_template_and_get_cfg(
    template: &Template<SsrNode>,
    translator: &Translator,
    (immutable_store, mutable_store): (&ImmutableStore, &impl MutableStore),
    global_state: &TemplateState,
    exporting: bool,
) -> Result<HashMap<String, String>, ServerError> {
    let mut render_cfg = HashMap::new();
    let template_root_path = template.get_path();
    let is_incremental = template.uses_incremental();

    let (pages, single_page) = build_template(
        template,
        translator,
        (immutable_store, mutable_store),
        global_state,
        exporting,
    )
    .await?;
    // If the template represents a single page itself, we don't need any
    // concatenation
    if single_page {
        render_cfg.insert(template_root_path.clone(), template_root_path.clone());
    } else {
        // Add each page that the template explicitly generated (ignoring ISR for now)
        for page in pages {
            let path = format!("{}/{}", &template_root_path, &page);
            // Remove any leading/trailing `/`s for the reasons described above
            let path = match path.strip_suffix('/') {
                Some(stripped) => stripped.to_string(),
                None => path,
            };
            let path = match path.strip_prefix('/') {
                Some(stripped) => stripped.to_string(),
                None => path,
            };
            render_cfg.insert(path, template_root_path.clone());
        }
        // Now if the page uses ISR, add an explicit `/*` in there after the template
        // root path Incremental rendering requires build-time path generation
        if is_incremental {
            render_cfg.insert(
                format!("{}/*", &template_root_path),
                template_root_path.clone(),
            );
        }
    }

    Ok(render_cfg)
}

/// Runs the build process of building many different templates for a single
/// locale. If you're not using i18n, provide a `Translator::empty()`
/// for this. You should only build the most commonly used locales here (the
/// rest should be built on demand).
pub async fn build_templates_for_locale(
    templates: &ArcTemplateMap<SsrNode>,
    translator: &Translator,
    (immutable_store, mutable_store): (&ImmutableStore, &impl MutableStore),
    global_state: &TemplateState,
    exporting: bool,
) -> Result<(), ServerError> {
    // The render configuration stores a list of pages to the root paths of their
    // templates
    let mut render_cfg = HashMap::new();
    // Create each of the templates
    let mut futs = Vec::new();
    for template in templates.values() {
        futs.push(build_template_and_get_cfg(
            template,
            translator,
            (immutable_store, mutable_store),
            global_state,
            exporting,
        ));
    }
    let template_cfgs = try_join_all(futs).await?;
    for template_cfg in template_cfgs {
        render_cfg.extend(template_cfg.into_iter())
    }

    immutable_store
        .write(
            "render_conf.json",
            &serde_json::to_string(&render_cfg).unwrap(),
        )
        .await?;

    Ok(())
}

/// Gets a translator and builds templates for a single locale.
///
/// This will also build the global state for this locale.
pub async fn build_templates_and_translator_for_locale(
    templates: &ArcTemplateMap<SsrNode>,
    locale: String,
    (immutable_store, mutable_store): (&ImmutableStore, &impl MutableStore),
    translations_manager: &impl TranslationsManager,
    gsc: &GlobalStateCreator,
    exporting: bool,
) -> Result<(), ServerError> {
    let translator = translations_manager
        .get_translator_for_locale(locale.to_string())
        .await?;

    let global_state = if gsc.uses_build_state() {
        // Generate the global state and write it to a file
        let global_state = gsc.get_build_state(locale).await?;
        immutable_store
            .write("static/global_state.json", &global_state.state.to_string())
            .await?;
        global_state
    } else {
        // If there's no build-time handler, we'll give an empty state. This will
        // be very unexpected if the user is generating at request-time, since all
        // the pages they have at build-time will be unable to access the global state.
        // We could either completely disable build-time rendering when there's
        // request-time global state generation, or we could give the user smart
        // errors and let them manage this problem themselves by gating their
        // usage of global state at build-time, since `.try_get_global_state()`
        // will give a clear `Ok(None)` at build-time. For speed, the latter
        // approach has been chosen.
        //
        // This is one of the biggest 'gotchas' in Perseus, and is clearly documented!
        TemplateState::empty()
    };

    if exporting && (gsc.uses_request_state() || gsc.can_amalgamate_states()) {
        return Err(ExportError::GlobalStateNotExportable.into());
    }

    build_templates_for_locale(
        templates,
        &translator,
        (immutable_store, mutable_store),
        &global_state,
        exporting,
    )
    .await?;

    Ok(())
}

/// The properties needed to build an app.
#[derive(Debug)]
pub struct BuildProps<'a, M: MutableStore, T: TranslationsManager> {
    /// All the templates in the app.
    pub templates: &'a ArcTemplateMap<SsrNode>,
    /// The app's locales data.
    pub locales: &'a Locales,
    /// An immutable store.
    pub immutable_store: &'a ImmutableStore,
    /// A mutable store.
    pub mutable_store: &'a M,
    /// A translations manager.
    pub translations_manager: &'a T,
    /// The global state creator.
    pub global_state_creator: &'a GlobalStateCreator,
    /// Whether or not we're exporting after this build (changes behavior
    /// slightly).
    pub exporting: bool,
}

/// Runs the build process of building many templates for the given locales
/// data, building directly for all supported locales. This is fine because of
/// how ridiculously fast builds are.
pub async fn build_app<M: MutableStore, T: TranslationsManager>(
    BuildProps {
        templates,
        locales,
        immutable_store,
        mutable_store,
        translations_manager,
        global_state_creator,
        exporting,
    }: BuildProps<'_, M, T>,
) -> Result<(), ServerError> {
    let locales = locales.get_all();
    let mut futs = Vec::new();

    for locale in locales {
        futs.push(build_templates_and_translator_for_locale(
            templates,
            locale.to_string(),
            (immutable_store, mutable_store),
            translations_manager,
            global_state_creator,
            exporting,
        ));
    }
    // Build all locales in parallel
    try_join_all(futs).await?;

    Ok(())
}
