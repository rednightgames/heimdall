use crate::infrastructure::databases::s3;
use crate::infrastructure::repositories::repository::ConfigS3Repository;
use std::sync::Arc;

pub struct Container {}

impl Container {
    fn new() -> Self {
        let repository = ConfigS3Repository::new(Arc::new(s3::connection()));

        Container {}
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
