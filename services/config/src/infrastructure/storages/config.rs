use crate::domain::models::id::ID;
use crate::domain::storages::config::ConfigStorage;
use crate::domain::storages::storage::StorageResult;
use crate::infrastructure::error::S3StorageError;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ConfigS3Storage {
    storage: Arc<s3::Bucket>,
}

impl ConfigS3Storage {
    pub async fn new(storage: Arc<s3::Bucket>) -> Self {
        ConfigS3Storage { storage }
    }
}

#[async_trait]
impl ConfigStorage for ConfigS3Storage {
    async fn create(&self, environment_id: ID, config_id: ID, config: &str) -> StorageResult<()> {
        self.storage
            .put_object_with_content_type(
                format!("{}/{}.json", environment_id, config_id),
                config.as_bytes(),
                "application/rednight.config",
            )
            .await
            .map_err(|_err| S3StorageError::new("Cannot create config", "Storage error", 102).into_inner())?;

        Ok(())
    }
    async fn get(&self, environment_id: ID, config_id: ID) -> StorageResult<String> {
        let config_data = self
            .storage
            .get_object(format!("{}/{}.json", environment_id, config_id))
            .await
            .map_err(|_err| S3StorageError::from("Not found").into_inner())?;

        let config = String::from_utf8(config_data.bytes().to_vec())
            .map_err(|_err| S3StorageError::from("Parse error").into_inner())?;

        Ok(config)
    }

    async fn delete(&self, environment_id: ID, config_id: ID) -> StorageResult<()> {
        self.storage
            .delete_object(format!("{}/{}.json", environment_id, config_id))
            .await
            .map_err(|err| S3StorageError::from(err).into_inner())?;

        Ok(())
    }
}
