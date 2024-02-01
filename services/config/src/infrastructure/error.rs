use crate::domain::error::{RepositoryError, StorageError};

pub struct ScyllaRepositoryError(RepositoryError);

impl ScyllaRepositoryError {
    pub fn into_inner(self) -> RepositoryError {
        self.0
    }
}

impl From<&str> for ScyllaRepositoryError {
    fn from(error: &str) -> Self {
        ScyllaRepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

impl From<cdrs_tokio::error::Error> for ScyllaRepositoryError {
    fn from(error: cdrs_tokio::error::Error) -> Self {
        ScyllaRepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct S3StorageError(StorageError);

impl S3StorageError {
    pub fn into_inner(self) -> StorageError {
        self.0
    }
}

impl From<&str> for S3StorageError {
    fn from(error: &str) -> Self {
        S3StorageError(StorageError {
            message: error.to_string(),
        })
    }
}

impl From<s3::error::S3Error> for S3StorageError {
    fn from(error: s3::error::S3Error) -> Self {
        S3StorageError(StorageError {
            message: error.to_string(),
        })
    }
}

impl From<std::num::ParseIntError> for S3StorageError {
    fn from(error: std::num::ParseIntError) -> Self {
        S3StorageError(StorageError {
            message: error.to_string(),
        })
    }
}

pub struct DecodeError(RepositoryError);

impl DecodeError {
    pub fn into_inner(self) -> RepositoryError {
        self.0
    }
}

impl From<base64_url::base64::DecodeError> for DecodeError {
    fn from(error: base64_url::base64::DecodeError) -> Self {
        DecodeError(RepositoryError {
            message: error.to_string(),
        })
    }
}
