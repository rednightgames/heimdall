use crate::domain::models::environment::{CreateEnvironment, Environment};
use crate::domain::models::id::ID;
use crate::domain::repositories::environment::{EnvironmentQueryParams, EnvironmentRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::scylla;
use crate::infrastructure::error::{DecodeError, ScyllaRepositoryError};
use crate::infrastructure::models::environment::ScyllaEnvironment;
use crate::infrastructure::queries::{
    CREATE_CONFIGS_KEYSPACE_QUERY, CREATE_ENVIRONMENTS_TABLE_QUERY, CREATE_ENVIRONMENT_QUERY,
};
use async_trait::async_trait;
use cdrs_tokio::frame::TryFromRow;
use cdrs_tokio::query_values;
use chrono::Utc;
use std::sync::Arc;

pub struct EnvironmentScyllaRepository {
    repository: Arc<scylla::Session>,
}

impl EnvironmentScyllaRepository {
    pub async fn new(repository: Arc<scylla::Session>) -> Self {
        repository
            .query(CREATE_CONFIGS_KEYSPACE_QUERY)
            .await
            .expect("ScyllaDB: initialisation failed: initialize keyspace");

        repository
            .query(CREATE_ENVIRONMENTS_TABLE_QUERY)
            .await
            .expect("ScyllaDB: initialisation failed: initialize table");

        EnvironmentScyllaRepository { repository }
    }
}

#[async_trait]
impl EnvironmentRepository for EnvironmentScyllaRepository {
    async fn create(&self, id: ID, new_env: &CreateEnvironment) -> RepositoryResult<Environment> {
        let cloned = new_env.clone();
        let created_at = Utc::now().timestamp_millis();

        self.repository
            .query_with_values(
                CREATE_ENVIRONMENT_QUERY,
                query_values!(id, cloned.name.clone(), created_at),
            )
            .await
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?;

        Ok(Environment {
            id,
            name: cloned.name,
            created_at,
        })
    }

    async fn list(
        &self,
        params: EnvironmentQueryParams,
    ) -> RepositoryResult<ResultPaging<Environment>> {
        let mut envs: Vec<Environment> = vec![];
        let mut curr_page = Some(0);

        if params.next_page.is_some() {
            curr_page = Option::from(
                String::from_utf8(
                    base64_url::decode(params.next_page().as_str())
                        .map_err(|err| DecodeError::from(err).into_inner())?,
                )
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            )
        }

        let rows = self
            .repository
            .query_with_values(
                r#"SELECT * FROM configs.environments WHERE id > ? LIMIT ?;"#,
                query_values!(curr_page, params.page_size() as i32),
            )
            .await
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
            .response_body()
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
            .into_rows()
            .ok_or_else(|| ScyllaRepositoryError::from(String::from("Rows not found")).into_inner())?;

        for row in rows {
            envs.push(Environment::from(
                ScyllaEnvironment::try_from_row(row)
                    .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?,
            ))
        }

        let next_page = if let Some(last_env) = envs.last() {
            if envs.len() == params.page_size() {
                Some(base64_url::encode(last_env.id.to_string().as_str()))
            } else {
                None
            }
        } else {
            None
        };

        Ok(ResultPaging {
            code: 0,
            items: envs,
            next_page,
        })
    }
}
