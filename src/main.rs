use actix_http::KeepAlive;
use actix_web::web;
use actix_web::HttpServer;
use handix::create_app;
use handix::option::Opt;
use handlebars::Handlebars;

fn setup() -> anyhow::Result<()> {
    let mut log_builder = env_logger::Builder::new();
    log_builder.parse_filters("info");

    log_builder.init();

    Ok(())
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let (opt, config_read_from) = Opt::try_build()?;

    setup()?;

    let mut handlebars: Handlebars<'static> = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref: web::Data<Handlebars<'static>> = web::Data::new(handlebars);

    eprintln!(
        "Config file path:\t{:?}",
        config_read_from
            .map(|config_file_path| config_file_path.display().to_string())
            .unwrap_or_else(|| "none".to_string())
    );

    log::info!("starting HTTP server at http://{}", &opt.http_addr);

    HttpServer::new(move || create_app(handlebars_ref.clone()))
        .disable_signals()
        .keep_alive(KeepAlive::Os)
        .bind(&opt.http_addr)?
        .run()
        .await?;

    Ok(())
}
