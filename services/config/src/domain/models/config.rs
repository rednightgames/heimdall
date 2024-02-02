use super::id::ID;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub id: ID,
    pub name: String,
    pub config: String,
    pub environment_id: ID,
    pub created_at: i64,
}

#[derive(Clone)]
pub struct CreateConfig {
    pub name: String,
    pub config: String,
}
