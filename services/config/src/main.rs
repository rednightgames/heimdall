use actix_web::{middleware::Logger, web, App, HttpServer};
use config::api::controllers::config_handler::create_config_handler;
use config::api::error::not_found;
use config::container::Container;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let container = Container::default();
    let config_service = container.config_service.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(config_service.clone()))
            .wrap(Logger::default())
            .service(web::scope("/configs").route("", web::post().to(create_config_handler)))
            .default_service(web::to(not_found))
    })
    .bind(("0.0.0.0", 6666))?
    .run()
    .await
}
