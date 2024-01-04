use crate::domain::error::CommonError;
use actix_web::HttpResponse;

pub async fn not_found() -> HttpResponse {
    actix_web::HttpResponse::NotFound().json(CommonError {
        message: String::from("404: Not Found"),
        code: 0,
    })
}
