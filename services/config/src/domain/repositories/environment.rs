use super::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_NEXT_PAGE, DEFAULT_PAGE_SIZE,
};
use crate::domain::models::environment::{CreateEnvironment, Environment};
use crate::domain::models::id::ID;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EnvironmentQueryParams {
    #[validate(custom = "crate::api::validator::validate_next_page")]
    pub next_page: Option<String>,
    #[validate(range(min = 5, max = 50))]
    pub page_size: Option<usize>,
}

impl QueryParams for EnvironmentQueryParams {
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
pub trait EnvironmentRepository: Send + Sync {
    async fn create(&self, id: ID, new_env: &CreateEnvironment) -> RepositoryResult<Environment>;
    async fn list(
        &self,
        params: EnvironmentQueryParams,
    ) -> RepositoryResult<ResultPaging<Environment>>;
}
