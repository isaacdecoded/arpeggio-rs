use std::fmt;
use std::error::Error;
use async_trait::async_trait;

use crate::backoffice::commands::domain::aggregates::create_todo::entities::todo::Todo;

#[derive(Debug)]
pub struct CreateTodoRepositoryError {
    pub msg: String,
}

impl fmt::Display for CreateTodoRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for CreateTodoRepositoryError {}

#[async_trait]
pub trait CreateTodoRepository: Sync {
    async fn generate_id(&self) -> Result<String, CreateTodoRepositoryError>;
    async fn save(&self, todo: &Todo) -> Result<(), CreateTodoRepositoryError>;
}
