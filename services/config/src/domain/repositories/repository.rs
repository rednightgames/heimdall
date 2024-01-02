use crate::domain::error::RepositoryError;
use serde::{Deserialize, Serialize};

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultPaging<T> {
    pub code: i64,
    pub items: Vec<T>,
}

pub const DEFAULT_PAGE: Option<usize> = Some(0);
pub const DEFAULT_PAGE_SIZE: Option<usize> = Some(25);

pub trait QueryParams: Send + Sync {
    fn page(&self) -> usize;
    fn page_size(&self) -> usize;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParamsImpl {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl QueryParams for QueryParamsImpl {
    fn page(&self) -> usize {
        self.page.or(DEFAULT_PAGE).unwrap_or_default()
    }
    fn page_size(&self) -> usize {
        self.page_size.or(DEFAULT_PAGE_SIZE).unwrap_or_default()
    }
}
