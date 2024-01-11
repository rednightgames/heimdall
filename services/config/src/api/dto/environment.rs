use crate::domain::models::environment::{CreateEnvironment, Environment};
use crate::domain::models::id::ID;
use crate::domain::repositories::repository::ResultPaging;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref UPPER_LOWER_DASH: Regex = Regex::new(r"^[a-zA-Z0-9\-]+$").unwrap();
}

#[derive(Debug, Serialize)]
pub struct EnvironmentDTO {
    pub id: ID,
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateEnvironmentDTO {
    #[validate(
        required,
        length(min = 1),
        regex(path = "UPPER_LOWER_DASH", code = "Invalid service name")
    )]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListEnvironmentDTO {
    pub id: ID,
    pub name: String,
}

impl From<Environment> for EnvironmentDTO {
    fn from(env: Environment) -> Self {
        EnvironmentDTO {
            id: env.id,
            name: env.name,
        }
    }
}

impl From<Environment> for ListEnvironmentDTO {
    fn from(env: Environment) -> Self {
        ListEnvironmentDTO {
            id: env.id,
            name: env.name,
        }
    }
}

impl From<CreateEnvironmentDTO> for CreateEnvironment {
    fn from(config: CreateEnvironmentDTO) -> Self {
        CreateEnvironment {
            name: config.name.unwrap(),
        }
    }
}

impl From<ResultPaging<Environment>> for ResultPaging<ListEnvironmentDTO> {
    fn from(envs: ResultPaging<Environment>) -> Self {
        ResultPaging {
            items: envs
                .items
                .into_iter()
                .map(|env: Environment| ListEnvironmentDTO::from(env))
                .collect(),
            code: envs.code,
            next_page: envs.next_page,
        }
    }
}
