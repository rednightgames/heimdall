use crate::domain::models::config::{Config, CreateConfig};
use crate::domain::models::id::ID;
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::connectors::scylla;
use crate::infrastructure::error::{DecodeError, ScyllaRepositoryError};
use crate::infrastructure::models::config::ScyllaConfig;
use crate::infrastructure::models::count::ScyllaCount;
use async_trait::async_trait;
use cdrs_tokio::frame::TryFromRow;
use cdrs_tokio::query_values;
use chrono::Utc;
use std::sync::Arc;

pub struct ConfigScyllaRepository {
    repository: Arc<scylla::Session>,
}

impl ConfigScyllaRepository {
    pub async fn new(repository: Arc<scylla::Session>) -> Self {
        repository
            .query(r#"create keyspace if not exists configs with replication = {'class': 'SimpleStrategy', 'replication_factor': 1};"#)
            .await
            .expect("scylla: initialisation failed: initialize keyspace");

        repository
            .query(r#"create table if not exists configs.configs (id bigint, name text, environment_id bigint, created_at timestamp, primary key (environment_id, id));"#)
            .await
            .expect("scylla: initialisation failed: initialize table");

        ConfigScyllaRepository { repository }
    }
}

#[async_trait]
impl ConfigRepository for ConfigScyllaRepository {
    async fn create(
        &self,
        id: ID,
        environment_id: ID,
        new_config: &CreateConfig,
    ) -> RepositoryResult<Config> {
        let cloned = new_config.clone();
        let created_at = Utc::now().timestamp_millis();

        let rows = self
            .repository
            .query_with_values(
                r#"select count(*) from configs.environments where id = ?"#,
                query_values!(environment_id),
            )
            .await
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
            .response_body()
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
            .into_rows()
            .ok_or_else(|| ScyllaRepositoryError::new("Cannot create config", "Environment not exists", 104).into_inner())?;

        match rows
            .last()
            .and_then(|r| ScyllaCount::try_from_row(r.clone()).ok())
        {
            Some(count) if count.clone().into_inner() == 1 => {}
            _ => {
                return Err(ScyllaRepositoryError::new("Cannot create config", "Environment not exists", 104).into_inner());
            }
        }

        self.repository
            .query_with_values(
                r#"insert into configs.configs (id, name, environment_id, created_at) values (?, ?, ?, ?);"#,
                query_values!(id, cloned.name.clone(), environment_id, created_at),
            )
            .await
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?;

        Ok(Config {
            id,
            name: cloned.name,
            config: String::default(),
            environment_id,
            created_at,
        })
    }

    async fn list(
        &self,
        environment_id: ID,
        params: ConfigQueryParams,
    ) -> RepositoryResult<ResultPaging<Config>> {
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

        async fn fetch_configs(
            repository: Arc<scylla::Session>,
            curr_page: ID,
            environment_id: ID,
            page_size: i32,
        ) -> RepositoryResult<Vec<Config>> {
            let mut configs: Vec<Config> = vec![];

            let rows = repository
                .query_with_values(
                    r#"select * from configs.configs where id > ? and environment_id = ? limit ?;"#,
                    query_values!(curr_page, environment_id, page_size),
                )
                .await
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .response_body()
                .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
                .into_rows()
                .ok_or_else(|| ScyllaRepositoryError::from("Rows not found").into_inner())?;

            for row in rows {
                configs
                    .push(Config::from(ScyllaConfig::try_from_row(row).map_err(
                        |err| ScyllaRepositoryError::from(err).into_inner(),
                    )?))
            }

            Ok(configs)
        }

        async fn fetch_count(
            repository: Arc<scylla::Session>,
            curr_page: ID,
            environment_id: ID,
        ) -> RepositoryResult<i64> {
            let rows = repository
                .query_with_values(
                    r#"select count(*) from configs.configs where id > ? and environment_id = ?;"#,
                    query_values!(curr_page, environment_id),
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

        let (configs, count) = tokio::try_join!(
            fetch_configs(
                self.repository.clone(),
                curr_page,
                environment_id,
                params.page_size() as i32
            ),
            fetch_count(self.repository.clone(), curr_page, environment_id)
        )?;

        let next_page = configs.last().and_then(|last_config| {
            if count > configs.len() as i64 {
                Some(base64_url::encode(last_config.id.to_string().as_str()))
            } else {
                None
            }
        });

        Ok(ResultPaging {
            code: 0,
            items: configs,
            next_page,
        })
    }

    async fn get(&self, environment_id: ID, config_id: ID) -> RepositoryResult<Config> {
        let rows = self
            .repository
            .query_with_values(
                r#"select * from configs.configs where id = ? and environment_id = ?;"#,
                query_values!(config_id, environment_id),
            )
            .await
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
            .response_body()
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
            .into_rows()
            .ok_or_else(|| {
                ScyllaRepositoryError::new("No rows found", "Config not exists", 104).into_inner()
            })?
            .into_iter()
            .last()
            .ok_or_else(|| {
                ScyllaRepositoryError::new("No rows found", "Config not exists", 104).into_inner()
            })?;

        Ok(Config::from(ScyllaConfig::try_from_row(rows).map_err(
            |err| ScyllaRepositoryError::from(err).into_inner(),
        )?))
    }

    async fn delete(&self, config_id: ID) -> RepositoryResult<()> {
        println!("{}", config_id);
        Ok(())
    }
}
