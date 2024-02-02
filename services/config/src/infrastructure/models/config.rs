use cdrs_tokio::frame::TryFromRow;

use crate::domain::models::config::Config;
use crate::domain::models::id::ID;

#[derive(Clone, Debug, PartialEq)]
pub struct ScyllaConfig {
    pub id: ID,
    pub name: String,
    pub environment_id: ID,
    pub created_at: i64,
}

impl From<ScyllaConfig> for Config {
    fn from(config: ScyllaConfig) -> Self {
        Config {
            id: config.id,
            name: config.name,
            config: String::default(),
            environment_id: config.environment_id,
            created_at: config.created_at,
        }
    }
}

impl TryFromRow for ScyllaConfig {
    fn try_from_row(row: cdrs_tokio::types::rows::Row) -> cdrs_tokio::error::Result<ScyllaConfig> {
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

        let environment_id =
            match cdrs_tokio::types::IntoRustByName::<i64>::get_by_name(&row, "environment_id") {
                Ok(Some(val)) => val,
                _ => {
                    return Err(cdrs_tokio::error::Error::from(
                        "Failed to get 'environment_id' from row",
                    ))
                }
            };

        Ok(ScyllaConfig {
            id,
            name,
            environment_id,
            created_at,
        })
    }
}
