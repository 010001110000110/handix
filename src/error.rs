use crate::get_application_name;

use actix_web::body::BoxBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Result;
use handlebars::Handlebars;
use serde_json::json;

// Custom error handlers, to return HTML responses when an error occurs.
pub fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response: HttpResponse = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request: &actix_web::HttpRequest = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let hb: Option<&Handlebars<'_>> = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t: &web::Data<Handlebars<'_>>| t.get_ref());
    match hb {
        Some(hb) => {
            let data: serde_json::Value = json!({
                "title": get_application_name(),
                "error": error,
                "status_code": res.status().as_str()
            });
            let body: std::result::Result<String, handlebars::RenderError> =
                hb.render("error", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
