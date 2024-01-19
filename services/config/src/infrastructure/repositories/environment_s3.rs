use crate::domain::models::environment::{CreateEnvironment, Environment};
use crate::domain::models::id::ID;
use crate::domain::repositories::environment::{EnvironmentQueryParams, EnvironmentRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::s3::Bucket;
use crate::infrastructure::error::{DecodeError, S3RepositoryError};
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
    async fn create(&self, id: ID, new_env: &CreateEnvironment) -> RepositoryResult<Environment> {
        let cloned = new_env.clone();
        let env_file = format!(r#"{{"name": "{}"}}"#, cloned.name);

        self.bucket
            .put_object_with_content_type(
                format!("{}.{}/environment.json", id, cloned.name),
                env_file.as_bytes(),
                "application/rednight.manifest",
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        Ok(Environment {
            name: cloned.name,
            id,
            created_at: 0,
        })
    }

    async fn list(
        &self,
        params: EnvironmentQueryParams,
    ) -> RepositoryResult<ResultPaging<Environment>> {
        let mut envs: Vec<Environment> = vec![];
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

        let (res, _) = self
            .bucket
            .list_page(
                String::default(),
                Option::from(String::from("/")),
                curr_page,
                None,
                Option::from(params.page_size()),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

        if let Some(obj) = res.common_prefixes {
            for o in obj {
                let name_parts: Vec<&str> =
                    o.prefix.strip_suffix('/').unwrap().split('.').collect();

                let id_str = name_parts[0];
                let name = String::from(name_parts[1]);

                let id: i64 = String::from(id_str)
                    .parse::<i64>()
                    .map_err(|err| S3RepositoryError::from(err).into_inner())?;

                envs.push(Environment {
                    id,
                    name,
                    created_at: 0,
                })
            }
        }

        let mut next_page = None;

        if res.next_continuation_token.is_some() {
            next_page = Option::from(base64_url::encode(
                res.next_continuation_token.unwrap_or_default().as_str(),
            ))
        }

        Ok(ResultPaging {
            code: 0,
            items: envs,
            next_page,
        })
    }
}
