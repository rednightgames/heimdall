use super::id::ID;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Environment {
    pub id: ID,
    pub name: String,
}
