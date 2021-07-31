// This binary builds all the templates with SSG

use crate::{
    template::Template,
    config_manager::ConfigManager,
    render_cfg::{RenderOpt, RenderCfg, TemplatesCfg, PagesCfg}
};
use crate::errors::*;
use std::collections::HashMap;
use sycamore::prelude::SsrNode;

/// Builds a template, writing static data as appropriate. This should be used as part of a larger build process. This returns both a list
/// of the extracted render options for this template (needed at request time), a list of pages that it explicitly generated, and a boolean
/// as to whether or not it only generated a single page to occupy the template's root path (`true` unless using using build-time path
/// generation).
pub fn build_template(
    template: Template<SsrNode>,
    config_manager: &impl ConfigManager
) -> Result<
    (
        Vec<RenderOpt>,
        Vec<String>,
        bool
    )
> {
    let mut render_opts: Vec<RenderOpt> = Vec::new();
    let mut single_page = false;
    let template_path = template.get_path();

    // Handle the boolean properties
    if template.revalidates() {
        render_opts.push(RenderOpt::Revalidated);
    }
    if template.uses_incremental() {
        render_opts.push(RenderOpt::Incremental);
    }

    // Handle static path generation
    // Because we iterate over the paths, we need a base path if we're not generating custom ones (that'll be overriden if needed)
    let paths = match template.uses_build_paths() {
        true => {
            render_opts.push(RenderOpt::StaticPaths);
            template.get_build_paths()?
        },
        false => {
            single_page = true;
            vec![String::new()]
        }
    };
    // Add the rest of the render options before we loop over defined pages
    if template.uses_build_state() {
        render_opts.push(RenderOpt::StaticProps);
    }
    if template.uses_request_state() {
        render_opts.push(RenderOpt::Server);
    }

    // Iterate through the paths to generate initial states if needed
    for path in paths.iter() {
        // If needed, we'll contruct a full path that's URL encoded so we can easily save it as a file
        // BUG: insanely nested paths won't work whatsoever if the filename is too long, maybe hash instead?
        let full_path = match render_opts.contains(&RenderOpt::StaticPaths) {
            true => urlencoding::encode(&format!("{}/{}", &template_path, path)).to_string(),
            // We don't want to concatenate the name twice if we don't have to
            false => template_path.clone()
        };

        // Handle static initial state generation
        // We'll only write a static state if one is explicitly generated
        if render_opts.contains(&RenderOpt::StaticProps) {
            // We pass in the latter part of the path, without the base specifier (because that would be the same for everything in the template)
            let initial_state = template.get_build_state(path.to_string())?;
            // Write that intial state to a static JSON file
            config_manager
                .write(&format!("./dist/static/{}.json", full_path), &initial_state)?;
            // Prerender the template using that state
            let prerendered = sycamore::render_to_string(
                ||
                    template.render_for_template(Some(initial_state))
            );
            // Write that prerendered HTML to a static file
            config_manager
                .write(&format!("./dist/static/{}.html", full_path), &prerendered)?;
        }

        // Note that SSR has already been handled by checking for `.uses_request_state()` above, we don't need to do any rendering here
        // If a template only uses SSR, it won't get prerendered at build time whatsoever

        // If the template is very basic, prerender without any state
        // It's safe to add a property to the render options here because `.is_basic()` will only return true if path generation is not being used (or anything else)
        if template.is_basic() {
            render_opts.push(RenderOpt::StaticProps);
            let prerendered = sycamore::render_to_string(
                ||
                    template.render_for_template(None)
            );
            // Write that prerendered HTML to a static file
            config_manager
                .write(&format!("./dist/static/{}.html", full_path), &prerendered)?;
        }
    }

    Ok((render_opts, paths, single_page))
}

// TODO function to build pages
/// Runs the build process of building many different templates.
pub fn build_templates(templates: Vec<Template<SsrNode>>, config_manager: &impl ConfigManager) -> Result<()> {
    let mut templates_conf: TemplatesCfg = HashMap::new();
    let mut pages_conf: PagesCfg = HashMap::new();
    // Create each of the templates
    for template in templates {
        let template_root_path = template.get_path();
        let is_incremental = template.uses_incremental();
        
        let (render_opts, pages, single_page) = build_template(template, config_manager)?;
        templates_conf.insert(
            template_root_path.clone(),
            render_opts
        );
        // If the tempalte represents a single page itself, we don't need any concatenation
        if single_page {
            pages_conf.insert(
                template_root_path.clone(),
                template_root_path.clone()
            );
        } else {
            // Add each page that the template explicitly generated (ignoring ISR for now)
            for page in pages {
                pages_conf.insert(
                    format!("{}/{}", &template_root_path, &page),
                    template_root_path.clone()
                );
            }
            // Now if the page uses ISR, add an explicit `/*` in there after the template root path
            // Incremental rendering requires build-time path generation
            if is_incremental {
                pages_conf.insert(
                    format!("{}/*", &template_root_path),
                    template_root_path.clone()
                );
            }
        }
    }

    let render_conf = RenderCfg {
        templates: templates_conf,
        pages: pages_conf
    };
    config_manager
        .write("./dist/render_conf.json", &serde_json::to_string(&render_conf)?)?;

    Ok(())
}
