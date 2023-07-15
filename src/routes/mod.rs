use actix_web::{web, HttpResponse};

pub mod home;
pub mod my_ip;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(home::show)
        .service(my_ip::show)
        .service(web::resource("/health").route(web::get().to(health_check)));
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "available" }))
}
