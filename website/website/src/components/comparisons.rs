use perseus::GenericNode;
use serde::{Deserialize, Serialize};
use sycamore::prelude::template;
use sycamore::prelude::Template as SycamoreTemplate;

/// A comparison for the comparisons table. Perseus itself also has an entry here. Note that any changes to the properties measured here
/// must also be reflected in the rendering code, which generates a title row independently.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
