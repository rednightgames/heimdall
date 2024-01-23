use crate::domain::error::RepositoryError;
use crate::domain::models::config::CreateConfig;
use crate::domain::models::{config::Config, id::ID};
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::s3::Bucket;
use crate::infrastructure::error::DecodeError;
use crate::infrastructure::error::S3RepositoryError;
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
    async fn create(
        &self,
        id: ID,
        environment_id: ID,
        new_config: &CreateConfig,
    ) -> RepositoryResult<Config> {
        let cloned = new_config.clone();

        let (res, _code) = self
            .bucket
            .list_page(
                format!("{}", environment_id),
                None,
                None,
                None,
                Option::from(1),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let environment = if let Some(environment_obj) = res.contents.first() {
            if let Some(index) = environment_obj.key.find('/') {
                let result = &environment_obj.key[0..index];
                result.to_string()
            } else {
                environment_obj.key.to_string()
            }
        } else {
            return Err(RepositoryError {
                message: String::from("Environment not found"),
            });
        };

        self.bucket
            .put_object_with_content_type(
                format!("{}/{}.{}.json", environment, id, cloned.name),
                cloned.config.as_bytes(),
                "application/rednight.config",
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let (data, _code) = self
            .bucket
            .head_object(format!("{}/{}.{}.json", environment, id, cloned.name))
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let created_at = DateTime::parse_from_rfc2822(data.last_modified.unwrap().as_str())
            .map_err(|err| S3RepositoryError::from(err).into_inner())?
            .timestamp_millis();

        Ok(Config {
            id,
            name: cloned.name,
            config: cloned.config,
            environment_id: environment_id,
            created_at,
        })
    }

    async fn list(
        &self,
        environment_id: ID,
        params: ConfigQueryParams,
    ) -> RepositoryResult<ResultPaging<Config>> {
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

        let (res, _code) = self
            .bucket
            .list_page(
                environment_id.to_string(),
                None,
                None,
                None,
                Option::from(1),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let environment = if let Some(environment_obj) = res.contents.first() {
            if let Some(index) = environment_obj.key.find('/') {
                let result = &environment_obj.key[0..index];
                result.to_string()
            } else {
                environment_obj.key.to_string()
            }
        } else {
            return Err(RepositoryError {
                message: String::from("Environment not found"),
            });
        };

        let mut prefix = format!("{}/", environment);
        if params.query.is_some() {
            prefix.push_str(params.query.clone().unwrap().as_str());
        }

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
                .replace(".json", "")
                .replace(".manifest", "");

            if filename == "environment" {
                continue;
            }

            let filename_parts: Vec<&str> = filename.split('.').collect();

            let id_str = filename_parts[0];
            let name = String::from(filename_parts[1]);

            let id: i64 = String::from(id_str)
                .parse::<i64>()
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
                environment_id: 0,
                created_at,
            });
        }

        Ok(ResultPaging {
            code: 0,
            items: configs,
            next_page,
        })
    }

    async fn get(&self, environment_id: ID, config_id: ID) -> RepositoryResult<Config> {
        let (res, _code) = self
            .bucket
            .list_page(
                environment_id.to_string(),
                None,
                None,
                None,
                Option::from(1),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let environment = if let Some(environment_obj) = res.contents.first() {
            if let Some(index) = environment_obj.key.find('/') {
                let result = &environment_obj.key[0..index];
                result.to_string()
            } else {
                environment_obj.key.to_string()
            }
        } else {
            return Err(RepositoryError {
                message: String::from("Environment not found"),
            });
        };

        let (res, _code) = self
            .bucket
            .list_page(
                format!("{}/{}.", environment, config_id),
                None,
                None,
                None,
                Option::from(1),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let config_name = if let Some(config_obj) = res.contents.first() {
            let res = config_obj
                .key
                .replace(format!("{}/{}.", environment, config_id).as_str(), "");

            String::from(res.strip_suffix(".json").unwrap())
        } else {
            return Err(RepositoryError {
                message: String::from("Config not found"),
            });
        };

        let config_data = self
            .bucket
            .get_object(format!(
                "{}/{}.{}.json",
                environment, config_id, config_name
            ))
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let (data, _code) = self
            .bucket
            .head_object(format!(
                "{}/{}.{}.json",
                environment, config_id, config_name
            ))
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        let created_at = DateTime::parse_from_rfc2822(data.last_modified.unwrap().as_str())
            .map_err(|err| S3RepositoryError::from(err).into_inner())?
            .timestamp_millis();

        Ok(Config {
            id: config_id,
            name: config_name,
            config: String::from_utf8(config_data.bytes().to_vec()).unwrap(),
            environment_id,
            created_at,
        })
    }

    async fn delete(&self, config_id: ID) -> RepositoryResult<()> {
        println!("{}", config_id);
        Ok(())
    }
}
