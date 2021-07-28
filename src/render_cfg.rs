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

pub type RenderCfg = HashMap<String, Vec<RenderOpt>>;
