use std::fmt;

#[derive(Debug)]
pub enum InternalError {
    ValidationError(Vec<String>),
    DatabaseError(String),
    Unauthorized,
    Forbidden,
    NotFound(String),
    InternalServerError(String),
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InternalError::ValidationError(errors) => write!(f, "Validation Error: {:?}", errors),
            InternalError::DatabaseError(message) => write!(f, "Database Error: {}", message),
            InternalError::Unauthorized => write!(f, "Unauthorized"),
            InternalError::Forbidden => write!(f, "Forbidden"),
            InternalError::NotFound(entity) => write!(f, "{} Not Found", entity),
            InternalError::InternalServerError(message) => write!(f, "Internal Server Error: {}", message),
        }
    }
}

impl std::error::Error for InternalError {}