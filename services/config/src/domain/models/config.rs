use super::id::ID;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub id: ID,
    pub config: String,
    pub environment: String,
    pub created_at: i64,
}

pub struct CreateConfig {
    pub config: String,
    pub environment: String,
}