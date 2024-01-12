use crate::domain::error::CommonError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("Deserialization error: {0}")]
    Json(String),
    #[error("Query parsing error: {0}")]
    Query(String),
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(CommonError {
            message: match self {
                HttpError::Json(_) => String::from("The request body contains invalid JSON"),
                HttpError::Query(_) => String::from("The request query parameters are invalid"),
            },
            code: match self {
                HttpError::Json(_) => 50109,
                HttpError::Query(_) => 50110,
            },
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::Json(_) => StatusCode::BAD_REQUEST,
            HttpError::Query(_) => StatusCode::BAD_REQUEST,
        }
    }
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(CommonError {
        message: String::from("404: Not Found"),
        code: 0,
    })
}
