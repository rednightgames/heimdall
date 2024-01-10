use crate::domain::models::environment::Environment;
use crate::domain::models::id::ID;
use crate::domain::repositories::repository::ResultPaging;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListEnvironmentDTO {
    pub id: ID,
    pub name: String,
}

impl From<Environment> for ListEnvironmentDTO {
    fn from(env: Environment) -> Self {
        ListEnvironmentDTO {
            id: env.id,
            name: env.name,
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
        }
    }
}
