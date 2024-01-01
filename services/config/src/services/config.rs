use crate::domain::error::CommonError;
use crate::domain::models::config::{Config, CreateConfig};
use crate::domain::models::id::ID;
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::config::ConfigService;
use async_trait::async_trait;
use id::Generator;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConfigServiceImpl {
    pub identifier: Generator,
    pub repository: Arc<dyn ConfigRepository>,
}

impl ConfigServiceImpl {
    pub fn new(identifier: Generator, repository: Arc<dyn ConfigRepository>) -> Self {
        ConfigServiceImpl {
            identifier,
            repository,
        }
    }
}

#[async_trait]
impl ConfigService for ConfigServiceImpl {
    async fn create(&self, config: CreateConfig) -> Result<Config, CommonError> {
        let cloned = config.clone();
        let id = self.identifier.clone().generate();

        self.repository
            .create(id, &cloned)
            .await
            .map_err(|err| CommonError::from(err))
    }

    async fn list(&self, params: ConfigQueryParams) -> Result<ResultPaging<Config>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|err| CommonError::from(err))
    }

    async fn get(&self, config_id: ID) -> Result<Config, CommonError> {
        self.repository
            .get(config_id)
            .await
            .map_err(|err| CommonError::from(err))
    }

    async fn delete(&self, config_id: ID) -> Result<(), CommonError> {
        self.repository
            .delete(config_id)
            .await
            .map_err(|err| CommonError::from(err))
    }
}
