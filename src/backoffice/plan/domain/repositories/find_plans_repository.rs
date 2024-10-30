use crate::backoffice::plan::domain::repositories::criteria::find_plans_criteria::FindPlansCriteria;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FindPlansRepositoryError {
    #[error("Unable to find Plans: {0}")]
    FindError(String),
}

#[async_trait]
pub trait FindPlansRepository<ReadModel>: Sync {
    async fn find(
        &self,
        criteria: &FindPlansCriteria,
    ) -> Result<Vec<ReadModel>, FindPlansRepositoryError>;
}
