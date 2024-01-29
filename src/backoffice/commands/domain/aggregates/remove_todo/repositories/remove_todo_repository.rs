use std::fmt;
use std::error::Error;
use async_trait::async_trait;

use crate::core::domain::entities::identity_object::IdentityObject;
use crate::backoffice::commands::domain::aggregates::remove_todo::entities::todo::Todo;

#[derive(Debug)]
pub struct RemoveTodoRepositoryError {
    pub msg: String,
}

impl fmt::Display for RemoveTodoRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for RemoveTodoRepositoryError {}

#[async_trait]
pub trait RemoveTodoRepository: Sync {
    async fn get_by_id(
        &self,
        id: &IdentityObject
    ) -> Result<Option<Todo>, RemoveTodoRepositoryError>;
    async fn save(&self, todo: &Todo) -> Result<(), RemoveTodoRepositoryError>;
}
