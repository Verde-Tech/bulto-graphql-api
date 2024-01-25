use std::fmt;

#[derive(Debug)]
pub enum InternalError {
    NotFound,
    InternalError,
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InternalError::NotFound => write!(f, "Not Found"),
            InternalError::InternalError => write!(f, "Internal Error"),
        }
    }
}

impl std::error::Error for InternalError {}