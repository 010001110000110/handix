pub mod error;
pub mod option;
pub mod routes;

use actix_cors::Cors;
use actix_http::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceResponse};
use actix_web::web;
use actix_web::web::Data;
use handlebars::Handlebars;
use option::Opt;

pub fn get_application_name() -> String {
    let (opt, _) = Opt::try_build().unwrap();
    String::from(opt.name)
}

pub fn create_app(
    hb: Data<Handlebars<'static>>,
) -> actix_web::App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let app = actix_web::App::new()
        .configure(|s: &mut web::ServiceConfig| configure_data(s, hb))
        .configure(routes::configure)
        .configure(|s: &mut web::ServiceConfig| configure_static_files(s));

    app.wrap(error::error_handlers())
        .wrap(
            Cors::default()
                .send_wildcard()
                .allow_any_header()
                .allow_any_origin()
                .allow_any_method()
                .max_age(86_400), // 24h
        )
        .wrap(actix_web::middleware::Logger::default())
        .wrap(actix_web::middleware::Compress::default())
        .wrap(actix_web::middleware::NormalizePath::new(
            actix_web::middleware::TrailingSlash::Trim,
        ))
}

pub fn configure_data(config: &mut web::ServiceConfig, hb: Data<Handlebars<'static>>) {
    config.app_data(hb);
}

pub fn configure_static_files(config: &mut web::ServiceConfig) {
    config.service(actix_files::Files::new("/styles", "./static/styles").use_last_modified(true));
}
