use crate::domain::models::{config::Config, id::ID};

pub struct ConfigS3 {
    pub id: ID,
    pub config: String,
    pub created_at: i64,
}

// Factory method for creating a new ConfigR2 from a Config
impl From<Config> for ConfigS3 {
    fn from(c: Config) -> Self {
        ConfigS3 {
            id: c.id,
            config: c.config,
            created_at: c.created_at,
        }
    }
}
