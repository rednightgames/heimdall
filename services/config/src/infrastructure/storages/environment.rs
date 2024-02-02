use crate::domain::models::id::ID;
use crate::domain::storages::environment::EnvironmentStorage;
use crate::domain::storages::storage::StorageResult;
use crate::infrastructure::error::S3StorageError;
use async_trait::async_trait;
use std::sync::Arc;

pub struct EnvironmentS3Storage {
    storage: Arc<s3::Bucket>,
}

impl EnvironmentS3Storage {
    pub async fn new(storage: Arc<s3::Bucket>) -> Self {
        EnvironmentS3Storage { storage }
    }
}

#[async_trait]
impl EnvironmentStorage for EnvironmentS3Storage {
    async fn delete(&self, environment_id: ID) -> StorageResult<()> {
        let rows = self.storage.list(format!("{}/", environment_id), Some("/".to_string())).await.map_err(|_err| 
            S3StorageError::new("Cannot delete environment", "Storage error", 102).into_inner()
        )?;

        for row in rows {
            for obj in row.contents {
                self.storage.delete_object(obj.key.to_string()).await.map_err(|_err| {
                    S3StorageError::new("Cannot delete environment", "Storage error", 102).into_inner()
                 })?;
            }
        }

        Ok(())
    }
}
