use core::fmt;
use std::str::FromStr;
use std::result::Result;
#[derive(PartialEq, Debug)]
pub enum TodoStatus {
    PENDING,
    DONE,
    REMOVED,
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TodoStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "PENDING" => Ok(TodoStatus::PENDING),
            "DONE" => Ok(TodoStatus::DONE),
            "REMOVED" => Ok(TodoStatus::REMOVED),
            _ => panic!("Invalid TodoStatus"),
        }
    }
}
