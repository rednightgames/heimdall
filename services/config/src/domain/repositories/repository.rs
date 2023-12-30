use crate::domain::error::RepositoryError;
use serde::{Deserialize, Serialize};

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultPaging<T> {
    pub code: i64,
    pub items: Vec<T>,
}

pub const DEFAULT_PAGE: Option<i32> = Some(0);
pub const DEFAULT_PAGE_SIZE: Option<i32> = Some(10);

pub trait QueryParams: Send + Sync {
    fn page(&self) -> i32;
    fn page_size(&self) -> i32;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParamsImpl {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl QueryParams for QueryParamsImpl {
    fn page(&self) -> i32 {
        self.page.or(DEFAULT_PAGE).unwrap_or_default()
    }
    fn page_size(&self) -> i32 {
        self.page_size.or(DEFAULT_PAGE_SIZE).unwrap_or_default()
    }
}
