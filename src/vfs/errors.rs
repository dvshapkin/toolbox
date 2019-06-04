use std::fmt::{Display, Error, Formatter};

//////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct NotAbsolutePathError {
    pub message: String,
}

impl NotAbsolutePathError {
    pub fn new() -> NotAbsolutePathError {
        NotAbsolutePathError {
            message: "Argument is not absolute path.".to_string(),
        }
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

//////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct NotRelativePathError {
    pub message: String,
}

impl NotRelativePathError {
    pub fn new() -> NotRelativePathError {
        NotRelativePathError {
            message: "Argument is not relative path.".to_string(),
        }
    }
}

impl Display for NotRelativePathError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", &self.message)
    }
}

impl std::error::Error for NotRelativePathError {
    fn description(&self) -> &str {
        &self.message
    }
}

//////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct PathNotBelongsError {
    pub message: String,
}

impl PathNotBelongsError {
    pub fn new(path: &str) -> PathNotBelongsError {
        PathNotBelongsError {
            message: format!("File system don't contain this path: {}", path),
        }
    }
}

impl Display for PathNotBelongsError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", &self.message)
    }
}

impl std::error::Error for PathNotBelongsError {
    fn description(&self) -> &str {
        &self.message
    }
}
