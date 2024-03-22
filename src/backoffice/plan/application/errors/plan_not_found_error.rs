use std::fmt::{ Display, Formatter, Result };
use std::error::Error;

#[derive(Debug)]
pub struct PlanNotFoundError {
    pub msg: String,
}

impl PlanNotFoundError {
    pub fn new(msg: String) -> Self {
        Self {
            msg: format!("Unable to get Plan due to: {msg}"),
        }
    }
}

impl Display for PlanNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for PlanNotFoundError {}
