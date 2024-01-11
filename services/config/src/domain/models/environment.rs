use super::id::ID;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct Environment {
    pub id: ID,
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateEnvironment {
    pub name: String,
}
