use std::borrow::Cow;
use serde::Serialize;
use validator::ValidationError;

#[derive(Serialize)]
pub struct Error {
    pub code: String,
    pub message: Option<String>,
}

impl Error {
    pub fn new(validation_error: &ValidationError) -> Error {
        return Error {
            code: validation_error.code.to_string(),
            message: validation_error.message.as_ref().map(|cow_str| cow_str.to_string()),
        };
    }
}


#[derive(Serialize)]
pub struct FieldError {
    pub code: String,
    pub errors: Vec<Error>,
}

#[derive(Serialize)]
pub struct ValidationErrorJsonPayload {
    pub errors: Vec<FieldError>,
}

impl From<&validator::ValidationErrors> for ValidationErrorJsonPayload {
    fn from(error: &validator::ValidationErrors) -> Self {
        let mut field_errors = Vec::new();
        for (field_name, validation_errors) in error
            .field_errors()
            .iter()
        {
            let mut errors = Vec::new();
            for validation_error in validation_errors.iter()
            {
                errors.push(Error::new(validation_error))
            }
            field_errors.push(FieldError { code: field_name.to_string(), errors })
        }
        ValidationErrorJsonPayload {
            errors: field_errors,
        }
    }
}
