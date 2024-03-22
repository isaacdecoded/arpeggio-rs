use std::fmt::{ Display, Formatter, Result };
use std::error::Error;

#[derive(Debug)]
pub struct TodoNotCheckedError {
    pub msg: String,
}

impl TodoNotCheckedError {
    pub fn new(msg: String) -> Self {
        Self {
            msg: format!("Unable to check Todo due to: {msg}"),
        }
    }
}

impl Display for TodoNotCheckedError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TodoNotCheckedError {}
