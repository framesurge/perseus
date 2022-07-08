use fmterr::fmt_err;
use perseus::{i18n::TranslationsManager, server::ServerOptions};
use std::sync::Arc;
use warp::http::Response;

pub async fn translations_handler<T: TranslationsManager>(
    locale: String,
    opts: Arc<ServerOptions>,
    translations_manager: Arc<T>,
) -> Response<String> {
    // Check if the locale is supported
    if opts.locales.is_supported(&locale) {
        // We know that the locale is supported, so any failure to get translations is a
        // 500
        let translations = translations_manager
            .get_translations_str_for_locale(locale.to_string())
            .await;
        let translations = match translations {
            Ok(translations) => translations,
            Err(err) => return Response::builder().status(500).body(fmt_err(&err)).unwrap(),
        };

        Response::new(translations)
    } else {
        Response::builder()
            .status(404)
            .body("locale not supported".to_string())
            .unwrap()
    }
}
