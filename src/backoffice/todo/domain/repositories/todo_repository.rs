use std::error::Error;
use std::fmt;

use async_trait::async_trait;

use crate::backoffice::todo::domain::entities::todo::Todo;
use crate::core::domain::{
    entities::string_value_object::StringValueObject,
    repositories::criteria::Criteria,
};

pub struct TodoCriteria {
    pub id: StringValueObject,
}

#[derive(Debug)]
pub struct TodoRepositoryError {
    pub msg: String,
}

impl fmt::Display for TodoRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TodoRepositoryError {}

#[async_trait]
pub trait TodoRepository: Sync {
    async fn generate_id(&self) -> Result<String, TodoRepositoryError>;
    async fn find(&self, criteria: Option<Criteria>) -> Result<Vec<Todo>, TodoRepositoryError>;
    async fn get_by_id(&self, id: &StringValueObject) -> Result<Option<Todo>, TodoRepositoryError>;
    async fn save(&self, todo: &Todo) -> Result<(), TodoRepositoryError>;
    async fn delete(&self, criteria: Criteria) -> Result<u32, TodoRepositoryError>;
}
