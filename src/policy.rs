use serde::{Deserialize, Serialize};

#[derive(Serialize, Hash, Deserialize)]
pub struct Subject {
    role: String,
}

#[derive(Serialize, Hash, Deserialize)]
pub struct Policy {
    pub object: String,
    pub action: String,
    pub rule: String,
}