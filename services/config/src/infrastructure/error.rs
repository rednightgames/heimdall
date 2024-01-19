use crate::domain::error::RepositoryError;

pub struct ScyllaRepositoryError(RepositoryError);

impl ScyllaRepositoryError {
    pub fn into_inner(self) -> RepositoryError {
        self.0
    }
}

impl From<String> for ScyllaRepositoryError {
    fn from(error: String) -> Self {
        ScyllaRepositoryError(RepositoryError { message: error })
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
pub struct S3RepositoryError(RepositoryError);

impl S3RepositoryError {
    pub fn into_inner(self) -> RepositoryError {
        self.0
    }
}

impl From<String> for S3RepositoryError {
    fn from(error: String) -> Self {
        S3RepositoryError(RepositoryError { message: error })
    }
}

impl From<s3::error::S3Error> for S3RepositoryError {
    fn from(error: s3::error::S3Error) -> Self {
        S3RepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

impl From<std::num::ParseIntError> for S3RepositoryError {
    fn from(error: std::num::ParseIntError) -> Self {
        S3RepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

impl From<chrono::ParseError> for S3RepositoryError {
    fn from(error: chrono::ParseError) -> Self {
        S3RepositoryError(RepositoryError {
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
