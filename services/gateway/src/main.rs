use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use futures::future;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let http_server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(hello)
    })
    .bind(("0.0.0.0", 80))?
    .run();

    let https_server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(hello)
    })
    .bind(("0.0.0.0", 443))?
    .run();

    future::try_join(http_server, https_server).await?;

    Ok(())
}
