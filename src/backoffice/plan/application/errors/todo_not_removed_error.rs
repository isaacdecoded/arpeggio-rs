use std::fmt::{ Display, Formatter, Result };
use std::error::Error;

#[derive(Debug)]
pub struct TodoNotRemovedError {
    pub msg: String,
}

impl TodoNotRemovedError {
    pub fn new(msg: String) -> Self {
        Self {
            msg: format!("Unable to remove Todo due to: {msg}"),
        }
    }
}

impl Display for TodoNotRemovedError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TodoNotRemovedError {}
