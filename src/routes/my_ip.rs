use crate::get_application_name;

use actix_web::get;
use actix_web::web::Data;
use actix_web::HttpResponse;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    origin: String,
}

#[get("/my-ip")]
pub async fn show(hb: Data<Handlebars<'static>>) -> HttpResponse {
    let api_data = reqwest::Client::new()
        .get("https://httpbin.org/ip")
        .send()
        .await
        .unwrap()
        .json::<ApiResponse>()
        .await
        .unwrap();

    let data: serde_json::Value = json!({
        "title": get_application_name(),
        "ip": api_data.origin
    });

    let body: String = hb.render("my-ip", &data).unwrap();

    HttpResponse::Ok().body(body)
}
