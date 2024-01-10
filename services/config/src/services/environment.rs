use crate::domain::error::CommonError;
use crate::domain::models::environment::Environment;
use crate::domain::repositories::environment::{EnvironmentQueryParams, EnvironmentRepository};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::environment::EnvironmentService;
use async_trait::async_trait;
use id::Generator;
use std::sync::Arc;

#[derive(Clone)]
pub struct EnvironmentServiceImpl {
    pub identifier: Generator,
    pub repository: Arc<dyn EnvironmentRepository>,
}

impl EnvironmentServiceImpl {
    pub fn new(identifier: Generator, repository: Arc<dyn EnvironmentRepository>) -> Self {
        EnvironmentServiceImpl {
            identifier,
            repository,
        }
    }
}

#[async_trait]
impl EnvironmentService for EnvironmentServiceImpl {
    async fn list(
        &self,
        params: EnvironmentQueryParams,
    ) -> Result<ResultPaging<Environment>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(CommonError::from)
    }
}
