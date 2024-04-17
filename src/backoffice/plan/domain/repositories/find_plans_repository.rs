use thiserror::Error;
use async_trait::async_trait;
use crate::core::domain::repositories::criteria::Criteria;

#[derive(Error, Debug)]
pub enum FindPlansRepositoryError {
    #[error("Unable to find Plans: {0}")] FindError(String),
}

#[async_trait]
pub trait FindPlansRepository<ReadModel>: Sync {
    async fn find(&self, criteria: Criteria) -> Result<Vec<ReadModel>, FindPlansRepositoryError>;
}
