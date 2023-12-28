use actix_web::{get, web, App, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
