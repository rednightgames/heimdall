use crate::domain::error::RepositoryError;

#[derive(Debug)]
pub struct R2RepositoryError(RepositoryError);

impl R2RepositoryError {
    pub fn into_inner(self) -> RepositoryError {
        self.0
    }
}

impl From<s3::error::S3Error> for R2RepositoryError {
    fn from(error: s3::error::S3Error) -> Self {
        R2RepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

impl From<std::num::ParseIntError> for R2RepositoryError {
    fn from(error: std::num::ParseIntError) -> Self {
        R2RepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

impl From<chrono::ParseError> for R2RepositoryError {
    fn from(error : chrono::ParseError) -> Self {
        R2RepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}