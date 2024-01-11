use crate::domain::models::environment::Environment;
use crate::domain::repositories::environment::{EnvironmentQueryParams, EnvironmentRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::s3::Bucket;
use crate::infrastructure::error::{S3RepositoryError, DecodeError};
use async_trait::async_trait;
use std::sync::Arc;

pub struct EnvironmentS3Repository {
    bucket: Arc<Bucket>,
}

impl EnvironmentS3Repository {
    pub fn new(bucket: Arc<Bucket>) -> Self {
        Self { bucket }
    }
}

#[async_trait]
impl EnvironmentRepository for EnvironmentS3Repository {
    async fn list(
        &self,
        params: EnvironmentQueryParams,
    ) -> RepositoryResult<ResultPaging<Environment>> {
        let mut envs: Vec<Environment> = vec![];
        let mut next_token = None;

        if !params.next_page().is_empty() {
            next_token = Option::from(String::from_utf8(base64_url::decode(params.next_page().as_str()).map_err(|err| DecodeError::from(err).into_inner())?).unwrap())
        }

        let (res, _) = self
            .bucket
            .list_page(
                String::default(),
                Option::from(String::from("/")),
                next_token,
                None,
                Option::from(params.page_size()),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        if let Some(obj) = res.common_prefixes {
            for o in obj {
                envs.push(Environment {
                    id: 0,
                    name: String::from(o.prefix.strip_suffix('/').unwrap()),
                })
            }
        }

        let mut next_page = None;

        if res.next_continuation_token.is_some() {
            next_page = Option::from(base64_url::encode(res.next_continuation_token.unwrap_or_default().as_str()))
        }

        Ok(ResultPaging {
            code: 0,
            items: envs,
            next_page
        })
    }
}
