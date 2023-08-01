use crate::DomainError;

#[derive(Clone)]
pub struct RegisterUserError {
    pub(crate) message: String,
}

impl RegisterUserError {
    pub fn new(message: String) -> RegisterUserError {
        RegisterUserError {
            message
        }
    }
}

impl DomainError for RegisterUserError {
    fn message(&self) -> String {
        self.message.clone()
    }
}