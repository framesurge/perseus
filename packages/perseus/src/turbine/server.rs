use super::Turbine;
use crate::{
    error_views::ServerErrorData,
    errors::{err_to_status_code, ServerError},
    i18n::{TranslationsManager, Translator},
    path::{PathMaybeWithLocale, PathWithoutLocale},
    router::{match_route_atomic, RouteInfoAtomic, RouteVerdictAtomic},
    server::get_path_slice,
    state::TemplateState,
    stores::MutableStore,
    utils::get_path_prefix_server,
    Request,
};
use fmterr::fmt_err;
use http::{
    header::{self, HeaderName},
    HeaderMap, HeaderValue, StatusCode,
};
use serde::{Deserialize, Serialize};

/// The integration-agnostic representation of
pub struct ApiResponse {
    /// The actual response body.
    pub body: String,
    /// The additional headers for the response. These will *not* include things
    /// like caching directives and the like, as they are expected to be
    /// handled by integrations.
    pub headers: HeaderMap,
    /// The HTTP status code of the response.
    pub status: StatusCode,
}
impl ApiResponse {
    /// Creates a 200 OK response with the given body and MIME type.
    pub fn ok(body: &str) -> Self {
        Self {
            body: body.to_string(),
            headers: HeaderMap::new(),
            status: StatusCode::OK,
        }
    }
    /// Creates a 404 Not Found response.
    pub fn not_found(msg: &str) -> Self {
        Self {
            body: msg.to_string(),
            headers: HeaderMap::new(),
            status: StatusCode::NOT_FOUND,
        }
    }
    /// Creates some other error response.
    pub fn err(status: StatusCode, body: &str) -> Self {
        Self {
            body: body.to_string(),
            headers: HeaderMap::new(),
            status,
        }
    }
    /// Adds the given header to this response.
    pub fn add_header(&mut self, k: HeaderName, v: HeaderValue) {
        self.headers.insert(k, v);
    }
    /// Sets the `Content-Type` HTTP header to the given MIME type, which tells
    /// the browser what file type it has actually been given. For HTML, this is
    /// especially important!
    ///
    /// As this is typically called last, and only once, it consumes `self` for
    /// ergonomics. If this is not desired, the `.add_header()` method can
    /// be manually invoked.
    ///
    /// # Panics
    ///
    /// This will panic if the given MIME type contains invalid ASCII
    /// characters.
    pub fn content_type(mut self, mime_type: &str) -> Self {
        self.headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str(mime_type).unwrap(),
        );
        self
    }
}

/// The query parameters used in subsequent load requests. This is provided for
/// convenience, since the majority of servers have some kind of mechanism to
/// parse query parameters automatically into `struct`s.
#[derive(Serialize, Deserialize)]
pub struct SubsequentLoadQueryParams {
    /// The name of the template or capsule the queried page or widget was
    /// generated by (since this endpoint is called by the app shell, which
    /// will have performed its own routing).
    pub entity_name: String,
    /// Whether or not this page or widget was an incremental match (returned by
    /// the router). This is required internally.
    pub was_incremental_match: bool,
}

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// The endpoint for getting translations.
    ///
    /// Translations have the `text/plain` MIME type, as they may be in an
    /// entirely arbitrary format, which should be manually parsed.
    pub async fn get_translations(&self, locale: &str) -> ApiResponse {
        // Check if the locale is supported
        if self.locales.is_supported(locale) {
            let translations = self
                .translations_manager
                .get_translations_str_for_locale(locale.to_string())
                .await;
            match translations {
                Ok(translations) => ApiResponse::ok(&translations).content_type("text/plain"),
                Err(err) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, &fmt_err(&err)),
            }
        } else {
            ApiResponse::not_found("locale not supported")
        }
    }

    /// The endpoint for getting page/capsule data through the subsequent load
    /// system.
    ///
    /// The path provided to this may have trailing slashes, these will be
    /// handled. It is expected to end in `.json` (needed for compatibility
    /// with the exporting system).
    ///
    /// Subsequent loads have the MIME type `application/json`.
    pub async fn get_subsequent_load(
        &self,
        raw_path: PathWithoutLocale,
        locale: String,
        entity_name: String,
        was_incremental_match: bool,
        req: Request,
    ) -> ApiResponse {
        // Check if the locale is supported
        if self.locales.is_supported(&locale) {
            // Parse the path
            let raw_path = raw_path.strip_prefix('/').unwrap_or(&raw_path);
            let raw_path = raw_path.strip_suffix('/').unwrap_or(raw_path);
            let path = PathWithoutLocale(match raw_path.strip_suffix(".json") {
                Some(path) => path.to_string(),
                None => {
                    return ApiResponse::err(StatusCode::BAD_REQUEST, "paths must end in `.json`")
                }
            });

            let page_data_partial = self
                .get_state_for_path(path, locale, &entity_name, was_incremental_match, req)
                .await;
            let page_data_partial = match page_data_partial {
                Ok(partial) => partial,
                Err(err) => {
                    // Parse the error to an appropriate status code
                    let status = StatusCode::from_u16(err_to_status_code(&err)).unwrap();
                    let msg = fmt_err(&err);
                    return ApiResponse::err(status, &msg);
                }
            };

            // We know the form of this, and it should never fail
            let page_data_str = serde_json::to_string(&page_data_partial).unwrap();
            ApiResponse::ok(&page_data_str).content_type("application/json")
        } else {
            ApiResponse::not_found("locale not supported")
        }
    }

    /// The endpoint for getting the full HTML contents of a page with no round
    /// trips (except for suspended states and/or delayed widgets). This is
    /// what should be returned to the user when they first ask for a page
    /// in the app.
    ///
    /// This expects to take a raw path without the locale split out that still
    /// needs URL decoding.
    ///
    /// If there's an error anywhere in this function, it will return the HTML
    /// of a proper error page.
    ///
    /// Initial loads *always* (even in the case of errors) have the MIME type
    /// `text/html`.
    pub async fn get_initial_load(
        &self,
        raw_path: PathMaybeWithLocale,
        req: Request,
    ) -> ApiResponse {
        // Decode the URL so we can work with spaces and special characters
        let raw_path = match urlencoding::decode(&raw_path) {
            Ok(path) => path.to_string(),
            Err(err) => {
                return self.html_err(
                    400,
                    fmt_err(&ServerError::UrlDecodeFailed { source: err }),
                    None,
                )
            }
        };
        let raw_path = PathMaybeWithLocale(raw_path.as_str().to_string());

        // Run the routing algorithm to figure out what to do here
        let path_slice = get_path_slice(&raw_path);
        let verdict = match_route_atomic(
            &path_slice,
            &self.render_cfg,
            &self.templates,
            &self.locales,
        );
        match verdict {
            RouteVerdictAtomic::Found(RouteInfoAtomic {
                path,
                template,
                locale,
                was_incremental_match,
            }) => {
                // Get the translations to interpolate into the page
                let translations_str = self
                    .translations_manager
                    .get_translations_str_for_locale(locale.clone())
                    .await;
                let translations_str = match translations_str {
                    Ok(translations) => translations,
                    // We know for sure that this locale is supported, so there's been an internal
                    // server error if it can't be found
                    Err(err) => {
                        return self.html_err(500, fmt_err(&err), None);
                    }
                };

                // We can use those to get a translator efficiently
                let translator = match self
                    .translations_manager
                    .get_translator_for_translations_str(locale, translations_str.clone())
                    .await
                {
                    Ok(translator) => translator,
                    // We need to give a proper translator to the error pages, which we can't
                    Err(err) => return self.html_err(500, fmt_err(&err), None),
                };

                // This returns both the page data and the most up-to-date global state
                let res = self
                    .get_initial_load_for_path(
                        path,
                        &translator,
                        template,
                        was_incremental_match,
                        req,
                    )
                    .await;
                let (page_data, global_state) = match res {
                    Ok(data) => data,
                    Err(err) => {
                        return self.html_err(
                            err_to_status_code(&err),
                            fmt_err(&err),
                            Some((&translator, &translations_str)),
                        )
                    }
                };

                let final_html = self
                    .html_shell
                    .as_ref()
                    .unwrap()
                    .clone()
                    .page_data(&page_data, &global_state, &translations_str)
                    .to_string();
                // NOTE: Yes, the user can fully override the content type...I have yet to find
                // a good use for this given the need to generate a `View`
                // though...
                let mut response = ApiResponse::ok(&final_html).content_type("text/html");

                // Generate and add HTTP headers
                let headers = match template.get_headers(TemplateState::from_value(page_data.state))
                {
                    Ok(headers) => headers,
                    // The pointlessness of returning an error here is well documented
                    Err(err) => {
                        return self.html_err(
                            err_to_status_code(&err),
                            fmt_err(&err),
                            Some((&translator, &translations_str)),
                        )
                    }
                };
                for (key, val) in headers {
                    response.add_header(key.unwrap(), val);
                }

                response
            }
            RouteVerdictAtomic::LocaleDetection(redirect_path) => {
                // TODO Parse the `Accept-Language` header and return a proper redirect
                // Construct a locale redirection fallback
                let html = self
                    .html_shell
                    .as_ref()
                    .unwrap() // We assume the app has been built
                    .clone()
                    .locale_redirection_fallback(
                        // This is the dumb destination we'd use if Wasm isn't enabled (the default
                        // locale). It has *zero* bearing on what the Wasm
                        // bundle will do.
                        &format!(
                            "{}/{}/{}",
                            get_path_prefix_server(),
                            &self.locales.default,
                            // This is a `PathWithoutLocale`
                            redirect_path.0,
                        ),
                    )
                    .to_string();
                // TODO Headers? They weren't here in the old code...
                // This isn't an error, but that's how this API expresses it (302 redirect)
                ApiResponse::err(StatusCode::FOUND, &html).content_type("text/html")
            }
            // Any unlocalized 404s would go to a redirect first
            RouteVerdictAtomic::NotFound { locale } => {
                // Get the translations to interpolate into the page
                let translations_str = self
                    .translations_manager
                    .get_translations_str_for_locale(locale.clone())
                    .await;
                let translations_str = match translations_str {
                    Ok(translations) => translations,
                    // We know for sure that this locale is supported, so there's been an internal
                    // server error if it can't be found
                    Err(err) => {
                        return self.html_err(500, fmt_err(&err), None);
                    }
                };

                // We can use those to get a translator efficiently
                let translator = match self
                    .translations_manager
                    .get_translator_for_translations_str(locale, translations_str.clone())
                    .await
                {
                    Ok(translator) => translator,
                    // We need to give a proper translator to the error pages, which we can't
                    Err(err) => return self.html_err(500, fmt_err(&err), None),
                };

                self.html_err(
                    404,
                    "page not found".to_string(),
                    Some((&translator, &translations_str)),
                )
            }
        }
    }

    /// Creates an HTML error page for when the initial load handler needs one.
    /// This will never provide a translator.
    ///
    /// This assumes that the app has already been actually built.
    ///
    /// # Panics
    /// This will panic implicitly if the given status code is invalid.
    fn html_err(
        &self,
        status: u16,
        msg: String,
        i18n_data: Option<(&Translator, &str)>,
    ) -> ApiResponse {
        let err_data = ServerErrorData { status, msg };
        let html = self.build_error_page(err_data, i18n_data);
        // This can construct a 404 if needed
        ApiResponse::err(StatusCode::from_u16(status).unwrap(), &html).content_type("text/html")
    }
}
