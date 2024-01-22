use crate::domain::models::config::{Config, CreateConfig};
use crate::domain::models::id::ID;
use crate::domain::repositories::config::{ConfigQueryParams, ConfigRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::scylla;
use crate::infrastructure::error::{ScyllaRepositoryError, DecodeError};
use crate::infrastructure::models::config::ScyllaConfig;
use crate::infrastructure::queries::{
    CREATE_CONFIGS_KEYSPACE_QUERY, CREATE_CONFIGS_TABLE_QUERY, CREATE_CONFIG_QUERY,
};
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
            .query(CREATE_CONFIGS_KEYSPACE_QUERY)
            .await
            .expect("scylla: initialisation failed: initialize keyspace");

        repository
            .query(CREATE_CONFIGS_TABLE_QUERY)
            .await
            .expect("scylla: initialisation failed: initialize table");

        ConfigScyllaRepository { repository }
    }
}

#[async_trait]
impl ConfigRepository for ConfigScyllaRepository {
    async fn create(&self, id: ID, environment_id: ID, new_config: &CreateConfig) -> RepositoryResult<Config> {
        let cloned = new_config.clone();
        let created_at = Utc::now().timestamp_millis();

        self.repository
            .query_with_values(
                CREATE_CONFIG_QUERY,
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
        let mut configs: Vec<Config> = vec![];
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
                r#"select * from configs.configs where id > ? and environment_id = ? limit ?;"#,
                query_values!(curr_page, environment_id, params.page_size() as i32),
            )
            .await
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
            .response_body()
            .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?
            .into_rows()
            .ok_or_else(|| {
                ScyllaRepositoryError::from(String::from("Rows not found")).into_inner()
            })?;

        for row in rows {
            configs.push(Config::from(
                ScyllaConfig::try_from_row(row)
                    .map_err(|err| ScyllaRepositoryError::from(err).into_inner())?,
            ))
        }

        let next_page = if let Some(last_config) = configs.last() {
            if configs.len() == params.page_size() {
                Some(base64_url::encode(last_config.id.to_string().as_str()))
            } else {
                None
            }
        } else {
            None
        };

        Ok(ResultPaging {
            code: 0,
            items: configs,
            next_page,
        })
    }

    async fn get(&self, environment_id: ID, config_id: ID) -> RepositoryResult<Config> {
        Ok(Config {
            id: todo!(),
            name: todo!(),
            config: todo!(),
            environment_id: todo!(),
            created_at: todo!(),
        })
    }

    async fn delete(&self, config_id: ID) -> RepositoryResult<()> {
        println!("{}", config_id);
        Ok(())
    }
}
