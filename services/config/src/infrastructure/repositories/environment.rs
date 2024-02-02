use crate::domain::models::environment::{CreateEnvironment, Environment};
use crate::domain::models::id::ID;
use crate::domain::repositories::environment::{EnvironmentQueryParams, EnvironmentRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::connectors::scylla;
use crate::infrastructure::error::{DecodeError, ScyllaRepositoryError};
use crate::infrastructure::models::count::ScyllaCount;
use crate::infrastructure::models::environment::ScyllaEnvironment;
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
            .query(r#"create keyspace if not exists configs with replication = {'class': 'SimpleStrategy', 'replication_factor': 1};"#)
            .await
            .expect("scylla: initialisation failed: initialize keyspace");

        repository
            .query(r#"create table if not exists configs.environments (id bigint, name text, created_at timestamp, primary key (id));"#)
            .await
            .expect("scylla: initialisation failed: initialize table");

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
                r#"insert into configs.environments ("id", "name", "created_at") values (?, ?, ?);"#,
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
        let mut curr_page = 0;

        if params.next_page.is_some() {
            curr_page = String::from_utf8(
                base64_url::decode(params.next_page().as_str())
                    .map_err(|err| DecodeError::from(err).into_inner())?,
            )
            .unwrap()
            .parse::<i64>()
            .unwrap()
        }

        async fn fetch_envs(
            repository: Arc<scylla::Session>,
            curr_page: ID,
            page_size: i32,
        ) -> RepositoryResult<Vec<Environment>> {
            let mut envs: Vec<Environment> = vec![];

            let rows = repository
                .query_with_values(
                    r#"select * from configs.environments where token(id) > ? limit ?;"#,
                    query_values!(curr_page, page_size),
                )
                .await
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .response_body()
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .into_rows()
                .ok_or_else(|| ScyllaRepositoryError::from("Rows not found").into_inner())?;

            for row in rows {
                envs.push(Environment::from(
                    ScyllaEnvironment::try_from_row(row)
                        .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?,
                ))
            }

            Ok(envs)
        }

        async fn fetch_count(
            repository: Arc<scylla::Session>,
            curr_page: ID,
        ) -> RepositoryResult<i64> {
            let rows = repository
                .query_with_values(
                    r#"select count(*) from configs.environments where token(id) > ?;"#,
                    query_values!(curr_page),
                )
                .await
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .response_body()
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .into_rows()
                .ok_or_else(|| ScyllaRepositoryError::from("Rows not found").into_inner())?;

            rows.last().map_or(Ok(0), |r| {
                Ok(ScyllaCount::try_from_row(r.clone())
                    .map_err(|err| ScyllaRepositoryError::from(err).into_inner())
                    .map(|count| count.into_inner())
                    .unwrap_or(0))
            })
        }

        let (envs, count) = tokio::try_join!(
            fetch_envs(
                self.repository.clone(),
                curr_page,
                params.page_size() as i32
            ),
            fetch_count(self.repository.clone(), curr_page)
        )?;

        let next_page = if let Some(last_env) = envs.last() {
            if count > envs.len() as i64 {
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

    async fn delete(&self, environment_id: ID) -> RepositoryResult<()> {
        async fn delete_environment(
            repository: Arc<scylla::Session>,
            environment_id: ID,
        ) -> RepositoryResult<()> {
            repository
                .query_with_values(
                    r#"delete from configs.environments where id = ?;"#,
                    query_values!(environment_id),
                )
                .await
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .response_body()
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?;

            Ok(())
        }

        async fn delete_configs(
            repository: Arc<scylla::Session>,
            environment_id: ID,
        ) -> RepositoryResult<()> {
            repository
                .query_with_values(
                    r#"delete from configs.configs where environment_id = ?;"#,
                    query_values!(environment_id),
                )
                .await
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .response_body()
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?;

            Ok(())
        }

        async fn fetch_count(
            repository: Arc<scylla::Session>,
            environment_id: ID,
        ) -> RepositoryResult<i64> {
            let rows = repository
                .query_with_values(
                    r#"select count(*) from configs.environments where id = ?;"#,
                    query_values!(environment_id),
                )
                .await
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .response_body()
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .into_rows()
                .ok_or_else(|| ScyllaRepositoryError::new("Not found", "Environment not exists", 104).into_inner())?;

            rows.last().map_or(Ok(0), |r| {
                Ok(ScyllaCount::try_from_row(r.clone())
                    .map_err(|err| ScyllaRepositoryError::from(err).into_inner())
                    .map(|count| count.into_inner())
                    .unwrap_or(0))
            })
        }

        let ((), (), count) = tokio::try_join!(
            delete_environment(self.repository.clone(), environment_id,),
            delete_configs(self.repository.clone(), environment_id,),
            fetch_count(self.repository.clone(), environment_id)
        )?;

        if count != 1 {
            Err(ScyllaRepositoryError::new("Not found", "Environment not exists", 104).into_inner())
        } else {
            Ok(())
        }
    }
}
