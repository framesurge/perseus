/// Defines app information about i18n, specifically about which locales are
/// supported.
#[derive(Clone, Debug)]
pub struct Locales {
    /// The default locale, which will be used as a fallback if the user's
    /// locale can't be extracted. This will be built for at build-time.
    pub default: String,
    /// All other supported locales, which will all be built at build time.
    pub other: Vec<String>,
    /// Whether or not the user is actually using i18n. This is set here because
    /// most things that need locale data also need it.
    pub using_i18n: bool,
}
impl Locales {
    /// Gets all the supported locales by combining the default, and other.
    pub fn get_all(&self) -> Vec<&String> {
        let mut vec = vec![&self.default];
        vec.extend(&self.other);

        vec
    }
    /// Checks if the given locale is supported.
    pub fn is_supported(&self, locale: &str) -> bool {
        let locales = self.get_all();
        locales.iter().any(|l| *l == locale)
    }
}
