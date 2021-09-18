use crate::Locales;
use sycamore_router::navigate;

/// Detects which locale the user should be served and redirects appropriately. This should only be used when the user navigates to a
/// page like `/about`, without a locale. This will only work on the client-side (needs access to browser i18n settings). Any pages
/// that direct to this should be explicitly excluded from search engines (they don't show anything until redirected). This is guided
/// by [RFC 4647](https://www.rfc-editor.org/rfc/rfc4647.txt), but is not yet fully compliant (only supports `xx-XX` form locales).
pub fn detect_locale(url: String, locales: Locales) {
    // If nothing matches, we'll use the default locale
    let mut locale = locales.default.clone();

    // We'll use `navigator.languages` to figure out the best locale, falling back to `navigator.language` if necessary
    let navigator = web_sys::window().unwrap().navigator();
    let langs = navigator.languages().to_vec();
    if langs.is_empty() {
        // We'll fall back to `language`, which only gives us one locale to compare with
        // If that isn't supported, we'll automatically fall back to the default locale
        if let Some(lang) = navigator.language() {
            locale = match compare_locale(&lang, locales.get_all()) {
                LocaleMatch::Exact(matched) | LocaleMatch::Language(matched) => matched,
                LocaleMatch::None => locales.default,
            }
        }
    } else {
        // We'll match each language individually, remembering that any exact match is preferable to a language-only match
        for cmp in langs {
            // We can reasonably assume that the user's locales are strings
            let cmp_str = cmp.as_string().unwrap();
            // As per RFC 4647, the first match (exact or language-only) is the one we'll use
            if let LocaleMatch::Exact(matched) | LocaleMatch::Language(matched) =
                compare_locale(&cmp_str, locales.get_all())
            {
                locale = matched;
                break;
            }
        }
    }

    // Imperatively navigate to the localized route
    navigate(&format!("/{}/{}", locale, url));
}

/// The possible outcomes of trying to match a locale.
enum LocaleMatch {
    /// The language and region match to a supported locale.
    Exact(String),
    /// The language (but not the region) matches a supported locale, the first supported locale with that language will be used.
    Language(String),
    /// The given locale isn't supported at all. If all the user's requested locales return this, we should fall back to the default.
    None,
}

/// Compares the given locale with the given vector of locales, identifying the closest match. This handles possible case discrepancies
/// automatically (e.g. Safari before iOS 10.2 returned all locales in lower-case).
///
/// Exact matches with any supported locale are preferred to language-only (and not region) matches. Remember that this function
/// only matches a single locale, not the list of the preferred locales (in which the first of either kind of match is used as per
/// [RFC 4647](https://www.rfc-editor.org/rfc/rfc4647.txt)).
///
/// This does NOT comply fully with [RFC 4647](https://www.rfc-editor.org/rfc/rfc4647.txt) yet, as only `xx-XX` form locales are
/// currently supported. This functionality will eventually be broken out into a separate module for ease of use.
fn compare_locale(cmp: &str, locales: Vec<&String>) -> LocaleMatch {
    let mut outcome = LocaleMatch::None;
    // Split into language and region (e.g. `en-US`) if possible
    let cmp_parts: Vec<&str> = cmp.split('-').collect();

    for locale in locales {
        // Split into language and region (e.g. `en-US`) if possible
        let parts: Vec<&str> = locale.split('-').collect();
        if locale == cmp {
            outcome = LocaleMatch::Exact(locale.to_string());
            // Any exact match voids anything after it (it'll be further down the list or only a partial match from here on)
            break;
        } else if cmp_parts.get(0) == parts.get(0) {
            // If we've already had a partial match higher up the chain, this is void
            // But we shouldn't break in case there's an exact match coming up
            if !matches!(outcome, LocaleMatch::Language(_)) {
                outcome = LocaleMatch::Language(locale.to_string())
            }
        }
        // If there's no match, just continue on for now
    }

    outcome
}
