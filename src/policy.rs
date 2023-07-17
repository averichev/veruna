use serde::{Deserialize, Serialize};

#[derive(Serialize, Hash, Deserialize)]
pub struct Subject {
    role: String,
}

#[derive(Serialize, Hash, Deserialize)]
pub struct Policy {
    object: String,
    action: String,
    rule: String,
}