// This binary builds all the pages with SSG

use serde::{Serialize, de::DeserializeOwned};
use crate::{
    page::Page,
    config_manager::ConfigManager,
    render_cfg::RenderOpt
};
use crate::errors::*;
use std::any::Any;

/// Builds a page, writing static data as appropriate. This should be used as part of a larger build process.
pub fn build_page<Props: Serialize + DeserializeOwned + Any>(page: Page<Props>, config_manager: &impl ConfigManager) -> Result<Vec<RenderOpt>> {
    let mut render_opts: Vec<RenderOpt> = Vec::new();
    let page_path = page.get_path();

    // Handle the boolean properties
    if page.revalidates() {
        render_opts.push(RenderOpt::Revalidated);
    }
    if page.uses_incremental() {
        render_opts.push(RenderOpt::Incremental);
    }

    // Handle static path generation
    // Because we iterate over the paths, we need a base path if we're not generating custom ones (that'll be overriden if needed)
    let paths = match page.uses_build_paths() {
        true => {
            render_opts.push(RenderOpt::StaticPaths);
            page.get_build_paths()?
        },
        false => vec![page_path.clone()]
    };

    // Iterate through the paths to generate initial states if needed
    for path in paths.iter() {
        // If needed, we'll contruct a full path that's URL encoded so we can easily save it as a file
        // BUG: insanely nested paths won't work whatsoever if the filename is too long, maybe hash instead?
        let full_path = match render_opts.contains(&RenderOpt::StaticPaths) {
            true => urlencoding::encode(&format!("{}/{}", &page_path, path)).to_string(),
            // We don't want to concatenate the name twice if we don't have to
            false => page_path.clone()
        };

        // Handle static initial state generation
        // We'll only write a static state if one is explicitly generated
        if page.uses_build_state() {
            render_opts.push(RenderOpt::StaticProps);
            // We pass in the latter part of the path, without the base specifier (because that would be the same for everything in the template)
            let initial_state = page.get_build_state(path.to_string())?;
            let initial_state_str = serde_json::to_string(&initial_state).unwrap();
            // Write that intial state to a static JSON file
            config_manager
                .write(&format!("./dist/static/{}.json", full_path), &initial_state_str)
                .unwrap();
            // Prerender the page using that state
            let prerendered = sycamore::render_to_string(
                ||
                    page.render_for_template(Some(initial_state))
            );
            // Write that prerendered HTML to a static file
            config_manager
                .write(&format!("./dist/static/{}.html", full_path), &prerendered)
                .unwrap();
        }

        // Handle server-side rendering
        // By definition, everything here is done at request-time, so there's not really much to do
        // Note also that if a page only uses SSR, it won't get prerendered at build time whatsoever
        if page.uses_request_state() {
            render_opts.push(RenderOpt::Server);
        }

        // If the page is very basic, prerender without any state
        if page.is_basic() {
            render_opts.push(RenderOpt::StaticProps);
            let prerendered = sycamore::render_to_string(
                ||
                    page.render_for_template(None)
            );
            // Write that prerendered HTML to a static file
            config_manager
                .write(&format!("./dist/static/{}.html", full_path), &prerendered)
                .unwrap();
        }
    }

    Ok(render_opts)
}

/// Runs the build process of building many different pages. This is done with a macro because typing for a function means we have to do
/// things on the heap.
/// (Any better solutions are welcome in PRs!)
#[macro_export]
macro_rules! build_pages {
    (
        [$($page:expr),+],
        $config_manager:expr
    ) => {
        let mut render_conf: $crate::render_cfg::RenderCfg = ::std::collections::HashMap::new();
        $(
            render_conf.insert(
                $page.get_path(),
                $crate::build::build_page($page, $config_manager)
                    .unwrap()
            );
        )+
        $config_manager
            .write("./dist/render_conf.json", &serde_json::to_string(&render_conf).unwrap())
            .unwrap();
    };
}
