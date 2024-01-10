use crate::domain::models::environment::Environment;
use crate::domain::repositories::environment::{EnvironmentQueryParams, EnvironmentRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::s3::Bucket;
use crate::infrastructure::error::S3RepositoryError;
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

        let (res, _) = self
            .bucket
            .list_page(
                String::default(),
                Option::from(String::from("/")),
                None,
                None,
                Option::from(params.page_size()),
            )
            .await
            .map_err(|err| S3RepositoryError::from(err).into_inner())?;

            for obj in res.common_prefixes {
            for o in obj {
                println!("{}", o.prefix);
                envs.push(Environment {
                    id: 0,
                    name: String::from(o.prefix.strip_suffix('/').unwrap()),
                })
            }
        }

        Ok(ResultPaging {
            code: 0,
            items: envs,
        })
    }
}
