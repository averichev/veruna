use crate::DomainErrorTrait as DomainErrorTrait;

#[derive(Clone, Debug)]
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

impl DomainErrorTrait for RegisterUserError {
    fn message(&self) -> String {
        self.message.clone()
    }
}


#[derive(Clone, Debug)]
pub struct LoginError {
    message: String,
}


impl LoginError {
    pub fn new(message: String) -> LoginError {
        LoginError {
            message
        }
    }
}

impl DomainErrorTrait for LoginError {
    fn message(&self) -> String {
        self.message.clone()
    }
}