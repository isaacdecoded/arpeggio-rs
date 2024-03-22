use std::fmt::{ Display, Formatter, Result };
use std::error::Error;

#[derive(Debug)]
pub struct TodoNotUpdatedError {
    pub msg: String,
}

impl TodoNotUpdatedError {
    pub fn new(msg: String) -> Self {
        Self {
            msg: format!("Unable to update Todo due to: {msg}"),
        }
    }
}

impl Display for TodoNotUpdatedError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TodoNotUpdatedError {}
