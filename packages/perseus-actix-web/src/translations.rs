use crate::Options;
use actix_web::{web, HttpRequest, HttpResponse};
use fmterr::fmt_err;
use perseus::internal::i18n::TranslationsManager;

/// The handler for calls to `.perseus/translations/{locale}`. This will manage returning errors and the like. THe JSON body returned
/// from this does NOT include the `locale` key, just a `HashMap<String, String>` of the translations themselves.
pub async fn translations<T: TranslationsManager>(
    req: HttpRequest,
    opts: web::Data<Options>,
    translations_manager: web::Data<T>,
) -> HttpResponse {
    let locale = req.match_info().query("locale");
    // Check if the locale is supported
    if opts.locales.is_supported(locale) {
        // We know that the locale is supported, so any failure to get translations is a 500
        let translations = translations_manager
            .get_translations_str_for_locale(locale.to_string())
            .await;
        let translations = match translations {
            Ok(translations) => translations,
            Err(err) => return HttpResponse::InternalServerError().body(fmt_err(&err)),
        };

        HttpResponse::Ok().body(translations)
    } else {
        HttpResponse::NotFound().body("locale not supported".to_string())
    }
}
