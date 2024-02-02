use super::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_NEXT_PAGE, DEFAULT_PAGE_SIZE,
};
use crate::domain::models::config::{Config, CreateConfig};
use crate::domain::models::id::ID;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct ConfigQueryParams {
    #[validate(custom = "crate::api::validator::validate_next_page")]
    pub next_page: Option<String>,
    #[validate(range(min = 5, max = 50))]
    pub page_size: Option<usize>,
    pub query: Option<String>,
}

impl QueryParams for ConfigQueryParams {
    fn next_page(&self) -> String {
        self.next_page
            .clone()
            .or(DEFAULT_NEXT_PAGE)
            .unwrap_or_default()
    }
    fn page_size(&self) -> usize {
        self.page_size.or(DEFAULT_PAGE_SIZE).unwrap_or_default()
    }
}

#[async_trait]
pub trait ConfigRepository: Send + Sync {
    async fn create(
        &self,
        id: ID,
        environment_id: ID,
        new_config: &CreateConfig,
    ) -> RepositoryResult<Config>;
    async fn list(
        &self,
        environment_id: ID,
        params: ConfigQueryParams,
    ) -> RepositoryResult<ResultPaging<Config>>;
    async fn get(&self, environment_id: ID, config_id: ID) -> RepositoryResult<Config>;
    async fn delete(&self, environment_id: ID, config_id: ID) -> RepositoryResult<()>;
}
