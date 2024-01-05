use crate::domain::models::config::CreateConfig;
use crate::domain::models::{config::Config, id::ID};
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
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
        let cloned = new_config.clone();

        self.bucket
            .put_object_with_content_type(
                format!("{}/{}.{}.json", cloned.environment, id, cloned.name),
                cloned.config.as_bytes(),
                "application/json",
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let (data, _) = self
            .bucket
            .head_object(format!(
                "{}/{}.{}.json",
                cloned.environment, id, cloned.name
            ))
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let created_at = DateTime::parse_from_rfc2822(data.last_modified.unwrap().as_str())
            .map_err(|err| S3RepositoryError::from(err).into_inner())?
            .timestamp_millis();

        Ok(Config {
            id,
            name: cloned.name,
            config: cloned.config,
            environment: cloned.environment,
            created_at,
        })
    }

    async fn list(&self, params: ConfigQueryParams) -> RepositoryResult<ResultPaging<Config>> {
        let mut configs: Vec<Config> = vec![];

        let (res, _) = self
            .bucket
            .list_page(
                format!("{}/", params.environment),
                Option::from(String::from("/")),
                None,
                None,
                Option::from(params.page_size()),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        println!("{}", params.page_size());
        println!(
            "{}",
            base64_url::encode(res.next_continuation_token.unwrap().as_str())
        );

        for obj in res.contents {
            let filename = obj
                .key
                .strip_prefix(format!("{}/", params.environment).as_str())
                .unwrap()
                .strip_suffix(".json")
                .unwrap();

            let filename_parts: Vec<&str> = filename.split('.').collect();

            let id_str = filename_parts[0];
            let name = String::from(filename_parts[1]);
            let environment = params.environment.clone();

            let id: u64 = String::from(id_str)
                .parse::<u64>()
                .map_err(|err| S3RepositoryError::from(err).into_inner())?;

            let created_at = obj
                .last_modified
                .parse::<DateTime<Utc>>()
                .map_err(|err| S3RepositoryError::from(err).into_inner())?
                .timestamp_millis();

            configs.push(Config {
                id,
                name,
                config: String::from(""),
                environment,
                created_at,
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
