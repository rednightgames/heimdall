use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub description: String,
    pub code: u32,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(&self.0)
    }

    fn status_code(&self) -> StatusCode {
        match &self.0.code {
            102 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl From<RepositoryError> for CommonError {
    fn from(error: RepositoryError) -> Self {
        CommonError {
            message: error.message,
            description: String::from("Repository error"),
            code: 1,
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct StorageError {
    pub message: String,
}

impl From<StorageError> for CommonError {
    fn from(error: StorageError) -> Self {
        CommonError {
            message: error.message,
            description: String::from("Storage error"),
            code: 1,
        }
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
