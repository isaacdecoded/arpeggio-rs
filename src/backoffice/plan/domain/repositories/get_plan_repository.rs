use crate::core::domain::models::identity_object::IdentityObject;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetPlanRepositoryError {
    #[error("Unable to get Plan by ID: {0}")]
    GetByIdError(String),
}

#[async_trait]
pub trait GetPlanRepository<ReadModel>: Sync {
    async fn get_by_id(
        &self,
        id: &IdentityObject,
    ) -> Result<Option<ReadModel>, GetPlanRepositoryError>;
}
