use crate::domain::error::CommonError;
use crate::domain::models::environment::{CreateEnvironment, Environment};
use crate::domain::models::id::ID;
use crate::domain::repositories::environment::{EnvironmentQueryParams, EnvironmentRepository};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::environment::EnvironmentService;
use crate::domain::storages::environment::EnvironmentStorage;
use async_trait::async_trait;
use futures::TryFutureExt;
use id::Generator;
use std::sync::Arc;

#[derive(Clone)]
pub struct EnvironmentServiceImpl {
    pub identifier: Generator,
    pub repository: Arc<dyn EnvironmentRepository>,
    pub storage: Arc<dyn EnvironmentStorage>,
}

impl EnvironmentServiceImpl {
    pub fn new(
        identifier: Generator,
        repository: Arc<dyn EnvironmentRepository>,
        storage: Arc<dyn EnvironmentStorage>,
    ) -> Self {
        EnvironmentServiceImpl {
            identifier,
            repository,
            storage,
        }
    }
}

#[async_trait]
impl EnvironmentService for EnvironmentServiceImpl {
    async fn create(&self, env: CreateEnvironment) -> Result<Environment, CommonError> {
        let cloned = env.clone();
        let id = self.identifier.clone().generate();

        self.repository
            .create(id, &cloned)
            .await
            .map_err(CommonError::from)
    }

    async fn list(
        &self,
        params: EnvironmentQueryParams,
    ) -> Result<ResultPaging<Environment>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(CommonError::from)
    }

    async fn delete(&self, environment_id: ID) -> Result<(), CommonError> {
        tokio::try_join!(
            self.storage.delete(environment_id).map_err(CommonError::from),
            self.repository.delete(environment_id).map_err(CommonError::from),
        )?;

        Ok(())
    }
}
