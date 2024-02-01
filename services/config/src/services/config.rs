use crate::domain::error::CommonError;
use crate::domain::models::config::{Config, CreateConfig};
use crate::domain::models::id::ID;
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::config::ConfigService;
use crate::domain::storages::config::ConfigStorage;
use async_trait::async_trait;
use futures::TryFutureExt;
use id::Generator;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConfigServiceImpl {
    pub identifier: Generator,
    pub repository: Arc<dyn ConfigRepository>,
    pub storage: Arc<dyn ConfigStorage>,
}

impl ConfigServiceImpl {
    pub fn new(
        identifier: Generator,
        repository: Arc<dyn ConfigRepository>,
        storage: Arc<dyn ConfigStorage>,
    ) -> Self {
        ConfigServiceImpl {
            identifier,
            repository,
            storage,
        }
    }
}

#[async_trait]
impl ConfigService for ConfigServiceImpl {
    async fn create(
        &self,
        environment_id: ID,
        config: CreateConfig,
    ) -> Result<Config, CommonError> {
        let cloned = config.clone();
        let id = self.identifier.clone().generate();

        let result = tokio::try_join!(
            self.storage
                .create(environment_id, id, &cloned.config)
                .map_err(CommonError::from),
            self.repository
                .create(id, environment_id, &cloned)
                .map_err(CommonError::from)
        );

        match result {
            Ok(((), repository_result)) => Ok(Config {
                id,
                name: cloned.name,
                config: cloned.config,
                environment_id,
                created_at: repository_result.created_at,
            }),
            Err(_error) => {
                let storage = self.storage.clone();
                let repository = self.repository.clone();

                tokio::task::spawn_blocking(move || {
                    drop(storage.delete(environment_id, id));
                    drop(repository.delete(id));
                });

                Err(CommonError {
                    message: "Cannot create new config".to_string(),
                    description: "Database error".to_string(),
                    code: 1,
                })
            }
        }
    }

    async fn list(
        &self,
        environment_id: ID,
        params: ConfigQueryParams,
    ) -> Result<ResultPaging<Config>, CommonError> {
        self.repository
            .list(environment_id, params)
            .await
            .map_err(CommonError::from)
    }

    async fn get(&self, environment_id: ID, config_id: ID) -> Result<Config, CommonError> {
        let (storage_result, repository_result) = tokio::try_join!(
            self.storage
                .get(environment_id, config_id)
                .map_err(CommonError::from),
            self.repository
                .get(environment_id, config_id)
                .map_err(CommonError::from),
        )?;

        Ok(Config {
            id: repository_result.id,
            name: repository_result.name,
            config: storage_result,
            environment_id: repository_result.environment_id,
            created_at: repository_result.created_at,
        })
    }

    async fn delete(&self, config_id: ID) -> Result<(), CommonError> {
        self.repository
            .delete(config_id)
            .await
            .map_err(CommonError::from)
    }
}
