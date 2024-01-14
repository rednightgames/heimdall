use actix_web::{middleware::Logger, web, App, HttpServer};
use config::api::controllers::config_handler::{create_config_handler, list_config_handler};
use config::api::controllers::environment_handler::{
    create_environment_handler, list_environment_handler,
};
use config::api::error::{not_found, HttpError};
use config::api::grpc::config::ConfigService;
use config::api::proto::config::config_server::ConfigServer;
use config::container::Container;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let container = Container::default();
    let config_service = container.config_service.clone();
    let environment_service = container.environment_service.clone();

    tokio::spawn(async move {
        Server::builder()
            .add_service(ConfigServer::new(ConfigService::default()))
            .serve("0.0.0.0:6667".parse().unwrap())
            .await
            .unwrap();
    });

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::JsonConfig::default()
                    .error_handler(|err, _req| HttpError::Json(err.to_string()).into()),
            )
            .app_data(
                web::QueryConfig::default()
                    .error_handler(|err, _req| HttpError::Query(err.to_string()).into()),
            )
            .app_data(web::Data::from(config_service.clone()))
            .app_data(web::Data::from(environment_service.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/config")
                    .route("", web::post().to(create_config_handler))
                    .route("", web::get().to(list_config_handler)),
            )
            .service(
                web::scope("/environment")
                    .route("", web::post().to(create_environment_handler))
                    .route("", web::get().to(list_environment_handler)),
            )
            .default_service(web::to(not_found))
    })
    .bind(("0.0.0.0", 6666))?
    .run()
    .await
    .unwrap();

    Ok(())
}
