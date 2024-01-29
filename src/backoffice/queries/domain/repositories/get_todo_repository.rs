use std::fmt;
use std::error::Error;
use async_trait::async_trait;

use crate::core::domain::entities::identity_object::IdentityObject;

#[derive(Debug)]
pub struct GetTodoRepositoryError {
    pub msg: String,
}

impl fmt::Display for GetTodoRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for GetTodoRepositoryError {}

#[async_trait]
pub trait GetTodoRepository<ReadModel>: Sync {
    async fn get_by_id(
        &self,
        id: IdentityObject
    ) -> Result<Option<ReadModel>, GetTodoRepositoryError>;
}
