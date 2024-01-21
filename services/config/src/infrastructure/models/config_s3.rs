use crate::domain::models::config::Config;
use crate::domain::models::id::ID;

pub struct ConfigS3 {
    pub id: ID,
    pub name: String,
    pub config: String,
    pub environment_id: ID,
    pub created_at: i64,
}

// Factory method for creating a new ConfigS3 from a Config
impl From<Config> for ConfigS3 {
    fn from(c: Config) -> Self {
        ConfigS3 {
            id: c.id,
            name: c.name,
            config: c.config,
            environment_id: c.environment_id,
            created_at: c.created_at,
        }
    }
}
