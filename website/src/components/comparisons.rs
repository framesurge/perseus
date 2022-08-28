use perseus::Html;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sycamore::prelude::view;
use sycamore::prelude::Scope;
use sycamore::prelude::View;

/// A comparison for the comparisons table. Perseus itself also has an entry
/// here. Note that any changes to the properties measured here must also be
/// reflected in the rendering code, which generates a title row independently.
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
    /// A bit of text that compares the two frameworks. This should be
    /// localized.
    pub text: String,
}

/// A raw comparison, as would be found on disk. This contains multiple locales,
/// which should be resolved to a single one before being handed to the
/// comparisons page itself.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawComparison {
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
    pub homepage_lighthouse_desktop: u8,
    pub homepage_lighthouse_mobile: u8,
    /// A map of locales to text that compares the frameworks.
    pub text: HashMap<String, String>,
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

/// Renders a Lighthouse score to have a text color. If it's 100, then we use
/// the appropriate emoji.
pub fn render_lighthouse_score<G: Html>(cx: Scope, score: u8) -> View<G> {
    if score == 100 {
        view! { cx,
            span(class = "emoji-green") {
                "💯"
            }
        }
    } else if score >= 90 {
        view! { cx,
            span(class = "text-green-600") {
                (score)
            }
        }
    } else if score >= 50 {
        view! { cx,
            span(class = "text-amber-500") {
                (score)
            }
        }
    } else {
        view! { cx,
            span(class = "text-red-500") {
                (score)
            }
        }
    }
}
