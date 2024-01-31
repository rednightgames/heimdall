use crate::domain::models::config::CreateConfig;
use crate::domain::models::{config::Config, id::ID};
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::{RepositoryResult, ResultPaging};
use crate::infrastructure::databases::s3::Bucket;
use crate::infrastructure::error::S3RepositoryError;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ConfigS3Repository {
    bucket: Arc<Bucket>,
}

impl ConfigS3Repository {
    pub fn new(bucket: Arc<Bucket>) -> Self {
        Self { bucket }
    }
}

#[async_trait]
impl ConfigRepository for ConfigS3Repository {
    async fn create(
        &self,
        id: ID,
        environment_id: ID,
        new_config: &CreateConfig,
    ) -> RepositoryResult<Config> {
        let cloned = new_config.clone();

        self.bucket
            .put_object_with_content_type(
                format!("{}/{}.json", environment_id, id),
                cloned.config.as_bytes(),
                "application/rednight.config",
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        Ok(Config {
            id,
            name: cloned.name,
            config: cloned.config,
            environment_id,
            created_at: 0,
        })
    }

    async fn list(
        &self,
        _environment_id: ID,
        _params: ConfigQueryParams,
    ) -> RepositoryResult<ResultPaging<Config>> {

        Ok(ResultPaging {
            code: 0,
            items: vec![],
            next_page: None,
        })
    }

    async fn get(&self, environment_id: ID, config_id: ID) -> RepositoryResult<Config> {
        let config_data = self
            .bucket
            .get_object(format!(
                "{}/{}.json",
                environment_id, config_id
            ))
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        Ok(Config {
            id: config_id,
            name: String::default(),
            config: String::from_utf8(config_data.bytes().to_vec()).unwrap(),
            environment_id,
            created_at: 0,
        })
    }

    async fn delete(&self, config_id: ID) -> RepositoryResult<()> {
        println!("{}", config_id);
        Ok(())
    }
}
