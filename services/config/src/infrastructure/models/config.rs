use crate::domain::models::config::Config;

pub struct ConfigR2 {
    pub config: String,
}

// Factory method for creating a new ConfigR2 from a Config
impl From<Config> for ConfigR2 {
    fn from(value: Config) -> Self {
        Self {
            config: value.config,
        }
    }
}
