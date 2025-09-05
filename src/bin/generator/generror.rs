use std::{io, num::ParseIntError};

pub type GenResult<T> = Result<T, GenError>;

#[derive(Debug, Clone)]
pub struct GenError {
    text: String,
}

impl GenError {
    pub fn new<T: AsRef<str>>(message: T) -> Self {
        Self {
            text: message.as_ref().to_string(),
        }
    }

    pub fn message(&self) -> &str {
        &self.text
    }
}

impl From<io::Error> for GenError {
    fn from(value: io::Error) -> Self {
        Self {
            text: value.to_string(),
        }
    }
}

impl From<ParseIntError> for GenError {
    fn from(value: ParseIntError) -> Self {
        Self {
            text: value.to_string(),
        }
    }
}
