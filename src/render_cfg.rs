use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// The different options for rendering. Each page has at least one of these that specifies what should be done at build and request-time.
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum RenderOpt {
    /// Creates a list of page paths all based on the same template at build-time. `StaticProps` is mandatory with this.
    StaticPaths,
    /// Any paths not pre-defined at build-time will be prerendered on the server as they would've been were they defined at build-time.
    /// They'll then be cached for future requests. If you have a lot of paths to render, this is the best option rather than building them
    /// all at build-time. This requires `StaticPaths` and thus `StaticProps`.
    Incremental,
    /// Prerenders props statically at build-time.
    StaticProps,
    /// Rerenders static pages based on some condition. This requires `StaticProps`.
    Revalidated,
    /// Prerenders props dynamically on the server at request-time. If used with `StaticProps`, the properties will be amalgamated and
    /// the HTML will be rerendered. Note that this option decreases TTFB, and `StaticProps` should be preferred if possible.
    Server,
}

pub type TemplatesCfg = HashMap<String, Vec<RenderOpt>>;
pub type PagesCfg = HashMap<String, String>;

/// The configuration that details how to render each page. Every known page path has an entry here except for those with ISR.
/// Any page that uses ISR (by defining the `TODO` property on its `Page` definition) has an entry for the template followed by `/*` to
/// avoid storing potentially billions of pages in this file. Any explicitly defined page though will be present in here for maximum speed.
/// Note that ISR is not compatible with defining other pages specifically under the root of the ISR template with different templates (e.g.
/// defining `/posts/*` and then defining a new template `/posts/index`, you'd have to use the same template there if you use ISR).
#[derive(Serialize, Deserialize)]
pub struct RenderCfg {
    /// All the registered templates. Each of these corresponds to a `Page` definition. They all have a series of render options that are
    /// automatically generated with `build_page` (invoked by `build_pages!`).
    pub templates: TemplatesCfg,
    /// The actual pages themselves, each mapping to the name of their template that defines their render options. Again, ISR rendered pages
    /// are stored with a wildcard here.
    pub pages: PagesCfg,
}
