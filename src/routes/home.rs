use crate::get_application_name;

use actix_web::get;
use actix_web::web::Data;
use actix_web::HttpResponse;
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
pub async fn show(hb: Data<Handlebars<'static>>) -> HttpResponse {
    let data: serde_json::Value = json!({
        "title": get_application_name(),
    });

    let body: String = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}
