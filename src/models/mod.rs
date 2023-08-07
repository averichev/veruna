use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims{
    pub(crate) username: String,
    pub(crate) id: String
}