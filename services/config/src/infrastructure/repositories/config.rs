use crate::domain::models::config::CreateConfig;
use crate::domain::models::{config::Config, id::ID};
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::error::DecodeError;
use crate::infrastructure::{databases::s3::Bucket, error::S3RepositoryError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::info;
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
        
        info!("id {}", id);
        info!("environment {}", cloned.environment);

        let (res, code) = self.bucket.list_page(format!("{}", cloned.environment), None, None, None, Option::from(1))
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        println!("{}", res.prefix.unwrap());

        println!("res.contents");
        for obj in res.contents {
            println!("{}", obj.key);
        }

        /*
        self.bucket
            .put_object_with_content_type(
                format!("{}/{}.{}.json", cloned.environment, id, cloned.name),
                cloned.config.as_bytes(),
                "application/json",
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let (data, _code) = self
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

         */

        Ok(Config {
            id,
            name: cloned.name,
            config: cloned.config,
            environment: cloned.environment,
            created_at: 0,
        })
    }

    async fn list(&self, params: ConfigQueryParams) -> RepositoryResult<ResultPaging<Config>> {
        let mut configs: Vec<Config> = vec![];
        let mut curr_page = None;

        if !params.next_page().is_empty() {
            curr_page = Option::from(
                String::from_utf8(
                    base64_url::decode(params.next_page().as_str())
                        .map_err(|err| DecodeError::from(err).into_inner())?,
                )
                .unwrap(),
            )
        }

        let environment = params.environment.clone().unwrap_or_default();
        let mut prefix = String::default();
        if params.environment.is_some() {
            prefix.push_str(format!("{}/", environment).as_str());
        }
        if params.query.is_some() {
            prefix.push_str(params.query.clone().unwrap().as_str());
        }
        //let prefix = format!("{}/{}", environment, params.query.clone().unwrap_or_default());

        println!("{}", prefix.clone());

        let (res, _code) = self
            .bucket
            .list_page(
                prefix.clone(),
                Option::from(String::from("/")),
                curr_page,
                None,
                Option::from(params.page_size()),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let mut next_page = None;

        if res.next_continuation_token.is_some() {
            next_page = Option::from(base64_url::encode(
                res.next_continuation_token.unwrap_or_default().as_str(),
            ))
        }

        for obj in res.contents {
            let filename = obj
                .key
                .strip_prefix(format!("{}/", environment).as_str())
                .unwrap()
                .strip_suffix(".json")
                .unwrap();

            let filename_parts: Vec<&str> = filename.split('.').collect();

            let id_str = filename_parts[0];
            let name = String::from(filename_parts[1]);

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
                config: String::default(),
                environment: 0,
                created_at,
            });
        }

        Ok(ResultPaging {
            code: 0,
            items: configs,
            next_page,
        })
    }

    async fn get(&self, config_id: ID) -> RepositoryResult<Config> {
        Ok(Config {
            id: config_id,
            name: String::from(""),
            config: String::from(""),
            environment: 0,
            created_at: 0,
        })
    }

    async fn delete(&self, config_id: ID) -> RepositoryResult<()> {
        println!("{}", config_id);
        Ok(())
    }
}
