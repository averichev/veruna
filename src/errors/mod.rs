use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct InternalServerError{
    pub(crate) message: String
}