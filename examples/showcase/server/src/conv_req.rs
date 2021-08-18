use perseus::{HttpRequest, Request};

// TODO set up proper error handling in an integration crate
/// Converts an Actix Web request into an `http::request`.
pub fn convert_req(raw: &actix_web::HttpRequest) -> Result<Request, String> {
	let req = HttpRequest::builder()
		.body(())
		.map_err(|err| format!("converting actix web request to perseus-compliant request failed: '{}'", err))?;
	
	Ok(req)
}