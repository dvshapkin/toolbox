
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone)]
pub struct NotAbsolutePathError {
    pub message: String
}

impl NotAbsolutePathError {
    pub fn from(msg: &str) -> NotAbsolutePathError {
        NotAbsolutePathError{ message: msg.to_string() }
    }
}

impl Display for NotAbsolutePathError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", &self.message)
    }
}

impl std::error::Error for NotAbsolutePathError {
    fn description(&self) -> &str {
        &self.message
    }
}