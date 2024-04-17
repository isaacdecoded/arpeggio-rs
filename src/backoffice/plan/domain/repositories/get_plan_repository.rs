use thiserror::Error;
use async_trait::async_trait;
use crate::core::domain::models::identity_object::IdentityObject;

#[derive(Error, Debug)]
pub enum GetPlanRepositoryError {
    #[error("Unable to get Plan by ID: {0}")] GetByIdError(String),
}

#[async_trait]
pub trait GetPlanRepository<ReadModel>: Sync {
    async fn get_by_id(
        &self,
        id: &IdentityObject
    ) -> Result<Option<ReadModel>, GetPlanRepositoryError>;
}
