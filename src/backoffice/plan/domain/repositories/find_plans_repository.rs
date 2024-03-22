use std::fmt;
use std::error::Error;
use async_trait::async_trait;
use crate::core::domain::repositories::criteria::Criteria;

#[derive(Debug)]
pub struct FindPlansRepositoryError {
    pub msg: String,
}

impl fmt::Display for FindPlansRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for FindPlansRepositoryError {}

#[async_trait]
pub trait FindPlansRepository<ReadModel>: Sync {
    async fn find(&self, criteria: Criteria) -> Result<Vec<ReadModel>, FindPlansRepositoryError>;
}
