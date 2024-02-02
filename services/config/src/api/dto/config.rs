use crate::domain::models::config::Config;
use crate::domain::models::id::ID;
use crate::domain::{models::config::CreateConfig, repositories::repository::ResultPaging};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref UPPER_LOWER_DASH: Regex = Regex::new(r"^[a-zA-Z0-9\-]+$").unwrap();
}

#[derive(Debug, Serialize)]
pub struct ConfigDTO {
    pub id: ID,
    pub name: String,
    pub config: String,
    pub environment_id: ID,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateConfigDTO {
    #[validate(
        required,
        length(min = 1),
        regex(path = "UPPER_LOWER_DASH", code = "Invalid service name")
    )]
    pub name: Option<String>,
    #[validate(required, length(min = 1))]
    pub config: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListConfigDTO {
    pub id: ID,
    pub name: String,
    pub created_at: i64,
}

impl From<CreateConfigDTO> for CreateConfig {
    fn from(config: CreateConfigDTO) -> Self {
        CreateConfig {
            name: config.name.unwrap(),
            config: config.config.unwrap(),
        }
    }
}

impl From<Config> for ConfigDTO {
    fn from(config: Config) -> Self {
        ConfigDTO {
            id: config.id,
            name: config.name,
            config: config.config,
            environment_id: config.environment_id,
            created_at: config.created_at,
        }
    }
}

impl From<Config> for ListConfigDTO {
    fn from(config: Config) -> Self {
        ListConfigDTO {
            id: config.id,
            name: config.name,
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
            next_page: configs.next_page,
        }
    }
}
