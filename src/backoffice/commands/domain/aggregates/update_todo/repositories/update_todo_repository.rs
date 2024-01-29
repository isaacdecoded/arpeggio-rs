use std::fmt;
use std::error::Error;
use async_trait::async_trait;

use crate::backoffice::commands::domain::aggregates::update_todo::entities::todo::Todo;
use crate::core::domain::entities::identity_object::IdentityObject;

#[derive(Debug)]
pub struct UpdateTodoRepositoryError {
    pub msg: String,
}

impl fmt::Display for UpdateTodoRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for UpdateTodoRepositoryError {}

#[async_trait]
pub trait UpdateTodoRepository: Sync {
    async fn get_by_id(
        &self,
        id: &IdentityObject
    ) -> Result<Option<Todo>, UpdateTodoRepositoryError>;
    async fn save(&self, todo: &Todo) -> Result<(), UpdateTodoRepositoryError>;
}
