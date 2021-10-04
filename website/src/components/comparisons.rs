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

/// Returns all the current comparisons to Perseus for display in a table
pub fn get_comparisons() -> Vec<Comparison> {
    vec![
        Comparison {
            name: "NextJS".to_string(),
            supports_ssg: FeatureSupport::Full,
            supports_ssr: FeatureSupport::Full,
            supports_ssr_ssg_same_page: FeatureSupport::None,
            supports_i18n: FeatureSupport::Full,
            supports_incremental: FeatureSupport::Full,
            supports_revalidation: FeatureSupport::Full,
            inbuilt_cli: FeatureSupport::Full,
            inbuilt_routing: FeatureSupport::Full,
            supports_shell: FeatureSupport::Full,
            supports_deployment: FeatureSupport::Full,
            supports_exporting: FeatureSupport::Full,
            language: "JavaScript/TypeScript".to_string(),
        },
        // TODO
    ]
}
