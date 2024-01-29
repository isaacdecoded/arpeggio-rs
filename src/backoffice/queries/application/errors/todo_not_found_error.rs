use std::fmt::{ Display, Formatter, Result };
use std::error::Error;

#[derive(Debug)]
pub struct TodoNotFoundError {
    pub msg: String,
}

impl TodoNotFoundError {
    pub fn new(msg: String) -> Self {
        Self {
            msg: format!("Unable to get Todo due to: {msg}"),
        }
    }
}

impl Display for TodoNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TodoNotFoundError {}
