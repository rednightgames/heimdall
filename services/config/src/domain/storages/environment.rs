use super::storage::StorageResult;
use crate::domain::models::id::ID;
use async_trait::async_trait;

#[async_trait]
pub trait EnvironmentStorage: Send + Sync {
    async fn delete(&self, environment_id: ID) -> StorageResult<()>;
}
