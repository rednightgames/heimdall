use crate::domain::models::config::CreateConfig;
use crate::domain::models::{config::Config, id::ID};
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::{RepositoryResult, ResultPaging};
use crate::infrastructure::{databases::s3::Bucket, error::S3RepositoryError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
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
    async fn create(&self, new_config: &CreateConfig) -> RepositoryResult<Config> {
        self.bucket
            .put_object_with_content_type(
                format!("{}/{}.json", new_config.environment, "0"),
                new_config.config.as_bytes(),
                "application/json",
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        Ok(Config {
            id: 0,
            config: String::from(""),
            environment: String::from("value"),
            created_at: 0,
        })
    }

    async fn list(&self, params: ConfigQueryParams) -> RepositoryResult<ResultPaging<Config>> {
        let mut configs: Vec<Config> = vec![];

        let res = self
            .bucket
            .list(String::default(), Option::from(String::from("/")))
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        for rec in res {
            for obj in rec.contents {
                let filename = obj.key.strip_suffix(".json").unwrap_or_default();

                let id = String::from(filename)
                    .parse::<u64>()
                    .map_err(|err| S3RepositoryError::from(err).into_inner())?;

                let timeshtamp = obj
                    .last_modified
                    .parse::<DateTime<Utc>>()
                    .map_err(|err| S3RepositoryError::from(err).into_inner())?
                    .timestamp_millis();

                configs.push(Config {
                    id,
                    config: String::from(""),
                    environment: String::from("value"),
                    created_at: timeshtamp,
                });
            }
        }

        Ok(ResultPaging {
            code: 0,
            items: configs,
        })
    }

    async fn get(&self, config_id: ID) -> RepositoryResult<Config> {
        Ok(Config {
            id: config_id,
            config: String::from(""),
            environment: String::from("value"),
            created_at: 0,
        })
    }

    async fn delete(&self, config_id: ID) -> RepositoryResult<()> {
        Ok(())
    }
}
