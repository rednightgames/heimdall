use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use futures::future;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let http_server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .service(hello)
            .service(echo)
    })
    .bind(("0.0.0.0", 80))?
    .run();

    let https_server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .service(hello)
            .service(echo)
    })
    .bind(("0.0.0.0", 443))?
    .run();

    future::try_join(http_server, https_server).await?;

    Ok(())
}
