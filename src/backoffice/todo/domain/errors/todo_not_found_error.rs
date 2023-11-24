use std::error::Error;
use std::fmt;

use crate::core::domain::entities::string_value_object::StringValueObject;
use crate::core::domain::entities::value_object::ValueObject;

#[derive(Debug)]
pub struct TodoNotFoundError {
    pub msg: String,
}

impl TodoNotFoundError {
    pub fn new(id: &StringValueObject) -> Self {
        let id_value = id.value();
        Self {
            msg: format!("Todo with id <{id_value}> do not exist.")
        }
    }
}

impl fmt::Display for TodoNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TodoNotFoundError {}

/*
    def __init__(self, id: str, msg: str | None = None):
        super().__init__(
            f"Todo with id <{id}> {f'not found due to: {msg}' if msg else 'not exist.'}"
        )
 */