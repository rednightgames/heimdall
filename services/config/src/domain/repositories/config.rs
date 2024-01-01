use super::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_PAGE, DEFAULT_PAGE_SIZE,
};
use crate::domain::models::config::{Config, CreateConfig};
use crate::domain::models::id::ID;
use async_trait::async_trait;

pub struct ConfigQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl QueryParams for ConfigQueryParams {
    fn page(&self) -> i32 {
        self.page.or(DEFAULT_PAGE).unwrap_or_default()
    }

    fn page_size(&self) -> i32 {
        self.page_size.or(DEFAULT_PAGE_SIZE).unwrap_or_default()
    }
}

#[async_trait]
pub trait ConfigRepository: Send + Sync {
    async fn create(&self, id: ID, new_config: &CreateConfig) -> RepositoryResult<Config>;
    async fn list(&self, params: ConfigQueryParams) -> RepositoryResult<ResultPaging<Config>>;
    async fn get(&self, config_id: ID) -> RepositoryResult<Config>;
    async fn delete(&self, config_id: ID) -> RepositoryResult<()>;
}
