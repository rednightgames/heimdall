use crate::domain::error::CommonError;
use crate::domain::models::environment::Environment;
use crate::domain::repositories::environment::EnvironmentQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use async_trait::async_trait;

#[async_trait]
pub trait EnvironmentService: Sync + Send {
    async fn list(
        &self,
        params: EnvironmentQueryParams,
    ) -> Result<ResultPaging<Environment>, CommonError>;
}
