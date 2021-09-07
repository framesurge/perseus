use crate::Options;
use actix_web::{web, HttpRequest, HttpResponse};
use perseus::TranslationsManager;

/// The handler for calls to `.perseus/page/*`. This will manage returning errors and the like.
pub async fn translations<T: TranslationsManager>(
    req: HttpRequest,
    opts: web::Data<Options>,
    translations_manager: web::Data<T>,
) -> HttpResponse {
    let locale = req.match_info().query("locale");
    // Check if the locale is supported
    if opts.locales.is_supported(locale) {
        // Create a translator for that locale (and hope the implementation is caching for sanity)
        // We know that it''s supported, which means a failure is a 500
        let translator = translations_manager
            .get_translator_for_locale(locale.to_string())
            .await;
        let translator = match translator {
            Ok(translator) => translator,
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        };
        // Serialize that into a JSON response
        let json = serde_json::to_string(&translator);
        let json = match json {
            Ok(json) => json,
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        };

        HttpResponse::Ok().body(json)
    } else {
        HttpResponse::NotFound().body("locale not supported".to_string())
    }
}
