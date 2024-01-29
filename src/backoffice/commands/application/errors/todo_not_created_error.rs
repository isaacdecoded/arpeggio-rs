use std::fmt::{ Display, Formatter, Result };
use std::error::Error;

#[derive(Debug)]
pub struct TodoNotCreatedError {
    pub msg: String,
}

impl TodoNotCreatedError {
    pub fn new(msg: String) -> Self {
        Self {
            msg: format!("Unable to create Todo due to: {msg}"),
        }
    }
}

impl Display for TodoNotCreatedError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TodoNotCreatedError {}
