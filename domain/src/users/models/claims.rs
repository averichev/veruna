use serde::{Deserialize, Serialize};

#[typetag::serde]
pub trait ClaimsTrait {
    fn username(&self) -> String;
    fn id(&self) -> String;
}


#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    username: String,
    id: String,
}

impl Claims {
    pub(crate) fn new(username: String, id: String) -> Box<dyn ClaimsTrait> {
        Box::new(Claims { username, id })
    }
}

#[typetag::serde]
impl ClaimsTrait for Claims {
    fn username(&self) -> String {
        self.username.clone()
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}