use crate::domain::models::config::Config;
use crate::domain::models::id::ID;
use crate::domain::{models::config::CreateConfig, repositories::repository::ResultPaging};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateConfigDTO {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    #[validate(required, length(min = 1))]
    pub config: Option<String>,
    #[validate(required, length(min = 1))]
    pub environment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConfigDTO {
    pub id: ID,
    pub name: String,
    pub config: String,
    pub environment: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct ListConfigDTO {
    pub id: ID,
    pub name: String,
    pub environment: String,
    pub created_at: i64,
}

impl From<CreateConfigDTO> for CreateConfig {
    fn from(config: CreateConfigDTO) -> Self {
        CreateConfig {
            name: config.name.unwrap(),
            config: config.config.unwrap(),
            environment: config.environment.unwrap(),
        }
    }
}

impl From<Config> for ConfigDTO {
    fn from(config: Config) -> Self {
        ConfigDTO {
            id: config.id,
            name: config.name,
            config: config.config,
            environment: config.environment,
            created_at: config.created_at,
        }
    }
}

impl From<Config> for ListConfigDTO {
    fn from(config: Config) -> Self {
        ListConfigDTO {
            id: config.id,
            name: config.name,
            environment: config.environment,
            created_at: config.created_at,
        }
    }
}

impl From<ResultPaging<Config>> for ResultPaging<ListConfigDTO> {
    fn from(configs: ResultPaging<Config>) -> Self {
        ResultPaging {
            items: configs
                .items
                .into_iter()
                .map(|config: Config| ListConfigDTO::from(config))
                .collect(),
            code: configs.code,
        }
    }
}
