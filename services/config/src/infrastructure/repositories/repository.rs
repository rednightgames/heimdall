use crate::domain::models::config::CreateConfig;
use crate::domain::models::{config::Config, id::ID};
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::{RepositoryResult, ResultPaging, QueryParams};
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
    async fn create(&self, id: ID, new_config: &CreateConfig) -> RepositoryResult<Config> {
        self.bucket
            .put_object_with_content_type(
                format!("{}/{}.{}.json", new_config.environment, id, new_config.name),
                new_config.config.as_bytes(),
                "application/json",
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        Ok(Config {
            id: 0,
            name: String::from("value"),
            config: String::from(""),
            environment: String::from("value"),
            created_at: 0,
        })
    }

    async fn list(&self, params: ConfigQueryParams) -> RepositoryResult<ResultPaging<Config>> {
        let mut configs: Vec<Config> = vec![];

        let (res, _) = self
            .bucket
            .list_page(String::from("stage/"), Option::from(String::from("/")), None, None, Option::from(params.page_size()))
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        println!("{}", params.page_size());
        println!("{}", base64_url::encode(res.next_continuation_token.unwrap().as_str()));

        for obj in res.contents {
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
                name: String::from(""),
                config: String::from(""),
                environment: String::from("value"),
                created_at: timeshtamp,
            });
        }

        Ok(ResultPaging {
            code: 0,
            items: configs,
        })
    }

    async fn get(&self, config_id: ID) -> RepositoryResult<Config> {
        Ok(Config {
            id: config_id,
            name: String::from(""),
            config: String::from(""),
            environment: String::from("value"),
            created_at: 0,
        })
    }

    async fn delete(&self, config_id: ID) -> RepositoryResult<()> {
        Ok(())
    }
}
