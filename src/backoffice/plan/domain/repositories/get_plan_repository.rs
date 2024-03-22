use std::fmt;
use std::error::Error;
use async_trait::async_trait;
use crate::core::domain::models::identity_object::IdentityObject;

#[derive(Debug)]
pub struct GetPlanRepositoryError {
    pub msg: String,
}

impl fmt::Display for GetPlanRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for GetPlanRepositoryError {}

#[async_trait]
pub trait GetPlanRepository<ReadModel>: Sync {
    async fn get_by_id(
        &self,
        id: &IdentityObject
    ) -> Result<Option<ReadModel>, GetPlanRepositoryError>;
}
