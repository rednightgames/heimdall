use super::id::ID;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub config: String,
    pub id: ID,
}
