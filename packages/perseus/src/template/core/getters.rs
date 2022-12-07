use sycamore::web::Html;
#[cfg(not(target_arch = "wasm32"))]
use crate::utils::ComputedDuration;
use super::Template;

impl<G: Html> Template<G> {
    /// Gets the path of the template. This is the root path under which any
    /// generated pages will be served. In the simplest case, there will
    /// only be one page rendered, and it will occupy that root position.
    ///
    /// Note that this will automatically transform `index` to an empty string.
    ///
    /// Note that this will prepend `__capsule/` to any capsules automatically.
    pub fn get_path(&self) -> String {
        let base = if self.path == "index" {
            String::new()
        } else {
            self.path.clone()
        };
        if self.is_capsule {
            format!("__capsule/{}", base)
        } else {
            base
        }
    }
    /// Gets the interval after which the template will next revalidate.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_revalidate_interval(&self) -> Option<ComputedDuration> {
        self.revalidate_after.clone()
    }

    // Render characteristic checkers
    /// Checks if this template can revalidate existing prerendered templates.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn revalidates(&self) -> bool {
        self.should_revalidate.is_some() || self.revalidate_after.is_some()
    }
    /// Checks if this template can revalidate existing prerendered templates
    /// after a given time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn revalidates_with_time(&self) -> bool {
        self.revalidate_after.is_some()
    }
    /// Checks if this template can revalidate existing prerendered templates
    /// based on some given logic.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn revalidates_with_logic(&self) -> bool {
        self.should_revalidate.is_some()
    }
    /// Checks if this template can render more templates beyond those paths it
    /// explicitly defines.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_incremental(&self) -> bool {
        self.incremental_generation
    }
    /// Checks if this template is a template to generate paths beneath it.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_build_paths(&self) -> bool {
        self.get_build_paths.is_some()
    }
    /// Checks if this template needs to do anything on requests for it.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_request_state(&self) -> bool {
        self.get_request_state.is_some()
    }
    /// Checks if this template needs to do anything at build time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn uses_build_state(&self) -> bool {
        self.get_build_state.is_some()
    }
    /// Checks if this template has custom logic to amalgamate build and
    /// request states if both are generated.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn can_amalgamate_states(&self) -> bool {
        self.amalgamate_states.is_some()
    }
    /// Checks if this template defines no rendering logic whatsoever. Such
    /// templates will be rendered using SSG. Basic templates can
    /// still modify headers (which could hypothetically be using global state
    /// that's dependent on server-side generation).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn is_basic(&self) -> bool {
        !self.uses_build_paths()
            && !self.uses_build_state()
            && !self.uses_request_state()
            && !self.revalidates()
            && !self.uses_incremental()
    }
}
