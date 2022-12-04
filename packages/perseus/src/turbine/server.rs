use crate::{Request, errors::{ServerError, err_to_status_code}, i18n::TranslationsManager, router::{RouteInfoAtomic, RouteVerdictAtomic, match_route_atomic}, server::get_path_slice, stores::MutableStore, template::TemplateState, utils::get_path_prefix_server};
use super::{Turbine, build_error_page::build_error_page};
use fmterr::fmt_err;
use http::{HeaderMap, HeaderValue, StatusCode, header::HeaderName};

/// The integration-agnostic representation of
pub struct ApiResponse {
    /// The actual response body.
    pub body: String,
    /// The additional headers for the response. These will *not* include things like caching directives
    /// and the like, as they are expected to be handled by integrations.
    pub headers: Option<HeaderMap>,
    /// The HTTP status code of the response.
    pub status: StatusCode,
}
impl ApiResponse {
    /// Creates a 200 OK response.
    pub fn ok(body: &str) -> Self {
        Self {
            body: body.to_string(),
            headers: None,
            status: StatusCode::OK,
        }
    }
    /// Creates a 404 Not Found response.
    pub fn not_found(msg: &str) -> Self {
        Self {
            body: msg.to_string(),
            headers: None,
            status: StatusCode::NOT_FOUND,
        }
    }
    /// Creates some other error response.
    pub fn err(status: StatusCode, body: &str) -> Self {
        Self {
            body: body.to_string(),
            headers: None,
            status,
        }
    }
    /// Adds the given header to this response.
    pub fn add_header(&mut self, k: HeaderName, v: HeaderValue) {
        if let Some(headers) = &mut self.headers {
            headers.insert(k, v);
        } else {
            let mut headers = HeaderMap::new();
            headers.insert(k, v);
            self.headers = Some(headers);
        }
    }
}

impl<M: MutableStore, T: TranslationsManager> Turbine<M, T> {
    /// The endpoint for getting translations.
    pub async fn get_translations(&self, locale: &str) -> ApiResponse {
        // Check if the locale is supported
        if self.locales.is_supported(locale) {
            let translations = self.translations_manager
                .get_translations_str_for_locale(locale.to_string())
                .await;
            match translations {
                Ok(translations) => ApiResponse::ok(&translations),
                Err(err) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, &fmt_err(&err)),
            }
        } else {
            ApiResponse::not_found("locale not supported",)
        }
    }

    /// The endpoint for getting page/capsule data through the subsequent load system.
    ///
    /// The path provided to this may have trailing slashes, these will be handled. It
    /// is expected to end in `.json` (needed for compatibility with the exporting system).
    pub async fn get_subsequent_load(
        &self,
        raw_path: &str,
        locale: &str,
        entity_name: &str,
        was_incremental_match: bool,
        req: Request,
    ) -> ApiResponse {
        // Check if the locale is supported
        if self.locales.is_supported(locale) {
            // Parse the path
            let raw_path = raw_path.strip_prefix('/').unwrap_or(&raw_path);
            let raw_path = raw_path.strip_suffix('/').unwrap_or(&raw_path);
            let path = match raw_path.strip_suffix(".json") {
                Some(path) => path,
                None => return ApiResponse::err(StatusCode::BAD_REQUEST, "paths must end in `.json`")
            };

            let page_data_partial = self.get_state_for_path(path, locale, entity_name, was_incremental_match, req).await;
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
            ApiResponse::ok(&page_data_str)
        } else {
            ApiResponse::not_found("locale not supported")
        }
    }

    /// The endpoint for getting the full HTML contents of a page with no round trips (except
    /// for suspended states and/or delayed widgets). This is what should be returned to the
    /// user when they first ask for a page in the app.
    ///
    /// This expects to take a raw path without the locale split out that still needs URL decoding.
    ///
    /// If there's an error anywhere in this function, it will return the HTML of a proper error
    /// page.
    pub async fn get_initial_load(
        &self,
        raw_path: &str, // This includes the locale!
        req: Request,
    ) -> ApiResponse {
        // Decode the URL so we can work with spaces and special chaaracters
        let raw_path = match urlencoding::decode(&raw_path) {
            Ok(path) => path.to_string(),
            // Yes, this would get an encoded path, but if it's *that* malformed, they deserve it
            Err(err) => return self.html_err(raw_path, 400, &fmt_err(&ServerError::UrlDecodeFailed { source: err }))
        };
        let raw_path = raw_path.as_str();

        // Run the routing algorithm to figure out what to do here
        let path_slice = get_path_slice(raw_path);
        let verdict = match_route_atomic(&path_slice, &self.render_cfg, &self.templates, &self.locales);
        match verdict {
            RouteVerdictAtomic::Found(RouteInfoAtomic {
                path,
                template,
                locale,
                was_incremental_match,
            }) => {
                // This returns both the page data and the most up-to-date global state
                let res = self.get_initial_load_for_path(&path, &locale, template, was_incremental_match, req).await;
                let (page_data, global_state) = match res {
                    Ok(data) => data,
                    Err(err) => return self.html_err(raw_path, err_to_status_code(&err), &fmt_err(&err)),
                };

                // Get the translations to interpolate into the page
                let translations = self
                    .translations_manager
                    .get_translations_str_for_locale(locale)
                    .await;
                let translations = match translations {
                    Ok(translations) => translations,
                    // We know for sure that this locale is supported, so there's been an internal
                    // server error if it can't be found
                    Err(err) => {
                        return self.html_err(raw_path, 500, &fmt_err(&err));
                    }
                };

                let final_html = self
                    .html_shell
                    .as_ref()
                    .unwrap()
                    .clone()
                    .page_data(&page_data, &global_state, &translations)
                    .to_string();
                let mut response = ApiResponse::ok(&final_html);

                // Generate and add HTTP headers
                for (key, val) in template.get_headers(TemplateState::from_value(page_data.state)) {
                    response.add_header(key.unwrap(), val);
                }

                response
            },
            RouteVerdictAtomic::LocaleDetection(redirect_path) => {
                // TODO Parse the `Accept-Language` header and return a proper redirect
                // Construct a locale redirection fallback
                let html = self
                    .html_shell
                    .as_ref()
                    .unwrap() // We assume the app has been built
                    .clone()
                    .locale_redirection_fallback(
                        // This is the dumb destination we'd use if Wasm isn't enabled (the default locale). It has
                        // *zero* bearing on what the Wasm bundle will do.
                        &format!(
                            "{}/{}/{}",
                            get_path_prefix_server(),
                            &self.locales.default,
                            redirect_path,
                        )
                    )
                    .to_string();
                // TODO Headers? They weren't here in the old code...
                // This isn't an error, but that's how this API expresses it (302 redirect)
                ApiResponse::err(StatusCode::FOUND, &html)
            },
            RouteVerdictAtomic::NotFound => self.html_err(raw_path, 404, "page not found"),
        }
    }

    /// Creates an HTML error page for when the initial load handler needs one. This will never provide
    /// a translator.
    ///
    /// This assumes that the app has already been actually built.
    fn html_err(&self, url: &str, code: u16, msg: &str) -> ApiResponse {
        let html = build_error_page(url, code, msg, None, &self.error_pages, self.html_shell.as_ref().unwrap());
        // This can construct a 404 if needed
        ApiResponse::err(StatusCode::from_u16(code).unwrap(), &html)
    }
}
