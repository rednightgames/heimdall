use actix_web::{middleware::Logger, web, App, HttpServer};
use config::api::controllers::config_handler::create_config_handler;
use config::api::error::not_found;
use config::api::grpc::config::ConfigService;
use config::api::proto::config::config_server::ConfigServer;
use config::container::Container;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let container = Container::default();
    let config_service = container.config_service.clone();

    tokio::spawn(async move {
        Server::builder()
            .add_service(ConfigServer::new(ConfigService::default()))
            .serve("0.0.0.0:6667".parse().unwrap())
            .await
            .unwrap();
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(config_service.clone()))
            .wrap(Logger::default())
            .service(web::scope("/").route("", web::post().to(create_config_handler)))
            .default_service(web::to(not_found))
    })
    .bind(("0.0.0.0", 6666))?
    .run()
    .await
    .unwrap();

    Ok(())
}
