use crate::backoffice::plan::domain::entities::plan::Plan;
use crate::core::domain::models::identity_object::IdentityObject;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlanRepositoryError {
    #[error("Unable to get Plan by ID: {0}")]
    GetByIdError(String),
    #[error("Unable to save Plan: {0}")]
    SaveError(String),
}

#[async_trait]
pub trait PlanRepository: Sync {
    async fn generate_id(&self) -> Result<IdentityObject, PlanRepositoryError>;
    async fn get_by_id(&self, id: &IdentityObject) -> Result<Option<Plan>, PlanRepositoryError>;
    async fn save(&self, plan: &Plan) -> Result<(), PlanRepositoryError>;
}
