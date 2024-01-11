use crate::domain::models::environment::Environment;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_NEXT_PAGE, DEFAULT_PAGE_SIZE,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentQueryParams {
    pub next_page: Option<String>,
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
    async fn list(
        &self,
        params: EnvironmentQueryParams,
    ) -> RepositoryResult<ResultPaging<Environment>>;
}
