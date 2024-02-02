use super::storage::StorageResult;
use crate::domain::models::id::ID;
use async_trait::async_trait;

#[async_trait]
pub trait ConfigStorage: Send + Sync {
    async fn create(&self, environment_id: ID, config_id: ID, config: &str) -> StorageResult<()>;
    async fn get(&self, environment_id: ID, config_id: ID) -> StorageResult<String>;
    async fn delete(&self, environment_id: ID, config_id: ID) -> StorageResult<()>;
}
