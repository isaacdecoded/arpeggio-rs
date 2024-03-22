use std::fmt;
use std::error::Error;
use async_trait::async_trait;
use crate::core::domain::models::identity_object::IdentityObject;
use crate::backoffice::plan::domain::entities::plan::Plan;

#[derive(Debug)]
pub struct PlanRepositoryError {
    pub msg: String,
}

impl fmt::Display for PlanRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for PlanRepositoryError {}

#[async_trait]
pub trait PlanRepository: Sync {
    async fn generate_id(&self) -> Result<IdentityObject, PlanRepositoryError>;
    async fn get_by_id(&self, id: &IdentityObject) -> Result<Option<Plan>, PlanRepositoryError>;
    async fn save(&self, plan: &Plan) -> Result<(), PlanRepositoryError>;
}
