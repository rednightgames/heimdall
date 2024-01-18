use crate::domain::error::CommonError;
use crate::domain::models::config::{Config, CreateConfig};
use crate::domain::models::id::ID;
use crate::domain::repositories::config::ConfigQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use async_trait::async_trait;

#[async_trait]
pub trait ConfigService: Sync + Send {
    async fn create(&self, config: CreateConfig) -> Result<Config, CommonError>;
    async fn list(
        &self,
        environment_id: ID,
        params: ConfigQueryParams,
    ) -> Result<ResultPaging<Config>, CommonError>;
    async fn get(&self, environment_id: ID, config_id: ID) -> Result<Config, CommonError>;
    async fn delete(&self, config_id: ID) -> Result<(), CommonError>;
}
