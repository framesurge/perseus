use super::Template;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use sycamore::prelude::{Scope, View};

/// Gets a `HashMap` of the given templates by their paths for serving. This
/// should be manually wrapped for the pages your app provides for convenience.
#[macro_export]
macro_rules! get_templates_map {
    [
        $($template:expr),+
    ] => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert(
                    $template.get_path(),
                    ::std::rc::Rc::new($template)
                );
            )+

            map
        }
    };
}

/// Gets a `HashMap` of the given templates by their paths for serving. This
/// should be manually wrapped for the pages your app provides for convenience.
///
/// This is the thread-safe version, which should only be used on the server.
#[macro_export]
macro_rules! get_templates_map_atomic {
    [
        $($template:expr),+
    ] => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert(
                    $template.get_path(),
                    ::std::sync::Arc::new($template)
                );
            )+

            map
        }
    };
}

/// A type alias for a `HashMap` of `Template`s. This uses `Rc`s to make the
/// `Template`s cloneable. In server-side multithreading, `ArcTemplateMap`
/// should be used instead.
pub type TemplateMap<G> = HashMap<String, Rc<Template<G>>>;
/// A type alias for a `HashMap` of `Template`s that uses `Arc`s for
/// thread-safety. If you don't need to share templates between threads, use
/// `TemplateMap` instead.
pub type ArcTemplateMap<G> = HashMap<String, Arc<Template<G>>>;
