use perseus::GenericNode;
use std::collections::HashMap;
use sycamore::prelude::template;
use sycamore::prelude::Template as SycamoreTemplate;

/// A comparison for the comparisons table. Perseus itself also has an entry here. Note that any changes to the properties measured here
/// must also be reflected in the rendering code, which generates a title row independently.
#[derive(Clone, Debug)]
pub struct Comparison {
    // We deliberately preserve order so that Perseus always comes first
    // That allows us to scroll though the others and keep the first two columns constantly there
    pub name: String,
    pub supports_ssg: FeatureSupport,
    pub supports_ssr: FeatureSupport,
    pub supports_ssr_ssg_same_page: FeatureSupport,
    pub supports_i18n: FeatureSupport,
    pub supports_incremental: FeatureSupport,
    pub supports_revalidation: FeatureSupport,
    pub inbuilt_cli: FeatureSupport,
    pub inbuilt_routing: FeatureSupport,
    pub supports_shell: FeatureSupport,
    pub supports_deployment: FeatureSupport,
    pub supports_exporting: FeatureSupport,
    pub language: String,
    // Ours are 100 and 95, respectively
    pub homepage_lighthouse_desktop: u8,
    pub homepage_lighthouse_mobile: u8,
}

/// The different levels of support for a feature.
#[derive(Clone, Debug)]
pub enum FeatureSupport {
    Full,
    Partial,
    None,
}
impl FeatureSupport {
    pub fn render(&self) -> String {
        match &self {
            Self::Full => "✅".to_string(),
            Self::Partial => "🟡".to_string(),
            Self::None => "❌".to_string(),
        }
    }
}

/// Renders a Lighthouse score to have a text color. If it's 100, then we use the appropriate emoji.
pub fn render_lighthouse_score<G: GenericNode>(score: u8) -> SycamoreTemplate<G> {
    if score == 100 {
        template! {
            "💯"
        }
    } else if score >= 90 {
        template! {
            span(class = "text-green-500") {
                (score)
            }
        }
    } else if score >= 50 {
        template! {
            span(class = "text-amber-500") {
                (score)
            }
        }
    } else {
        template! {
            span(class = "text-red-500") {
                (score)
            }
        }
    }
}

/// Returns Perseus' own data for comparison. The only thing particularly likely to change here is the mobile Lighthouse score.
pub fn get_perseus_comparison() -> Comparison {
    Comparison {
        // The name won't actually be used, we'll use the i18n `perseus` ID
        name: "Perseus".to_string(),
        supports_ssg: FeatureSupport::Full,
        supports_ssr: FeatureSupport::Full,
        supports_ssr_ssg_same_page: FeatureSupport::Full,
        supports_i18n: FeatureSupport::Full,
        supports_incremental: FeatureSupport::Full,
        supports_revalidation: FeatureSupport::Full,
        inbuilt_cli: FeatureSupport::Full,
        inbuilt_routing: FeatureSupport::Full,
        supports_shell: FeatureSupport::Full,
        supports_deployment: FeatureSupport::Full,
        supports_exporting: FeatureSupport::Full,
        language: "Rust".to_string(),
        homepage_lighthouse_desktop: 100,
        homepage_lighthouse_mobile: 95,
    }
}

/// Returns all the current comparisons to Perseus for display in a table
pub fn get_comparisons() -> HashMap<String, Comparison> {
    let mut map = HashMap::new();
    map.insert(
        "NextJS".to_string(),
        Comparison {
            name: "NextJS".to_string(),
            supports_ssg: FeatureSupport::Full,
            supports_ssr: FeatureSupport::Full,
            supports_ssr_ssg_same_page: FeatureSupport::None,
            supports_i18n: FeatureSupport::Partial,
            supports_incremental: FeatureSupport::Full,
            supports_revalidation: FeatureSupport::Full,
            inbuilt_cli: FeatureSupport::Full,
            inbuilt_routing: FeatureSupport::Full,
            supports_shell: FeatureSupport::Full,
            supports_deployment: FeatureSupport::Full,
            supports_exporting: FeatureSupport::Full,
            language: "JavaScript/TypeScript".to_string(),
            homepage_lighthouse_desktop: 100,
            homepage_lighthouse_mobile: 72,
        },
    );
    map.insert(
        "GatsbyJS".to_string(),
        Comparison {
            name: "GatsbyJS".to_string(),
            supports_ssg: FeatureSupport::Full,
            supports_ssr: FeatureSupport::None,
            supports_ssr_ssg_same_page: FeatureSupport::None,
            supports_i18n: FeatureSupport::Partial,
            supports_incremental: FeatureSupport::None,
            supports_revalidation: FeatureSupport::None,
            inbuilt_cli: FeatureSupport::Full,
            inbuilt_routing: FeatureSupport::Full,
            supports_shell: FeatureSupport::Full,
            supports_deployment: FeatureSupport::Full,
            supports_exporting: FeatureSupport::Full,
            language: "JavaScript/TypeScript".to_string(),
            homepage_lighthouse_desktop: 75,
            homepage_lighthouse_mobile: 45, // TODO confirm it's really this bad...
        },
    );
    // TODO use JSON file to get comparisons

    map
}
