use crate::domain::models::environment::Environment;
use crate::domain::models::id::ID;
use cdrs_tokio::frame::TryFromRow;

#[derive(Clone, Debug, PartialEq)]
pub struct ScyllaEnvironment {
    pub id: ID,
    pub name: String,
    pub created_at: i64,
}

impl From<ScyllaEnvironment> for Environment {
    fn from(env: ScyllaEnvironment) -> Self {
        Environment {
            id: env.id,
            name: env.name,
            created_at: env.created_at,
        }
    }
}

impl TryFromRow for ScyllaEnvironment {
    fn try_from_row(
        row: cdrs_tokio::types::rows::Row,
    ) -> cdrs_tokio::error::Result<ScyllaEnvironment> {
        let id = match cdrs_tokio::types::IntoRustByName::<i64>::get_by_name(&row, "id") {
            Ok(Some(val)) => val,
            _ => {
                return Err(cdrs_tokio::error::Error::from(
                    "Failed to get 'id' from row",
                ))
            }
        };

        let name = match cdrs_tokio::types::IntoRustByName::<String>::get_by_name(&row, "name") {
            Ok(Some(val)) => val,
            _ => {
                return Err(cdrs_tokio::error::Error::from(
                    "Failed to get 'name' from row",
                ))
            }
        };

        let created_at =
            match cdrs_tokio::types::IntoRustByName::<i64>::get_by_name(&row, "created_at") {
                Ok(Some(val)) => val,
                _ => {
                    return Err(cdrs_tokio::error::Error::from(
                        "Failed to get 'created_at' from row",
                    ))
                }
            };

        Ok(ScyllaEnvironment {
            id,
            name,
            created_at,
        })
    }
}
