use std::fmt::{ Display, Formatter, Result };
use std::error::Error;

#[derive(Debug)]
pub struct TodoNotAddedError {
    pub msg: String,
}

impl TodoNotAddedError {
    pub fn new(msg: String) -> Self {
        Self {
            msg: format!("Unable to add Todo due to: {msg}"),
        }
    }
}

impl Display for TodoNotAddedError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TodoNotAddedError {}
