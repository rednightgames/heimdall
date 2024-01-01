use serde::{Deserialize, Serialize};
use crate::domain::models::config::CreateConfig;
use crate::domain::models::id::ID;

#[derive(Deserialize, Serialize)]
pub struct CreateConfigDTO {
    pub config: String,
    pub environment: String,
}

#[derive(Debug, Serialize)]
pub struct ConfigDTO {
    pub id: ID,
    pub config: String,
    pub environment: String,
    pub created_at: i64,
}

impl From<CreateConfigDTO> for CreateConfig {
    fn from(c: CreateConfigDTO) -> Self {
        CreateConfig {
            config: c.config,
            environment: c.environment,
        }
    }
}
