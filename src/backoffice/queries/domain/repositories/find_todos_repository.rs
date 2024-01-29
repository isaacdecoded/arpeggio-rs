use std::fmt;
use std::error::Error;
use async_trait::async_trait;

#[derive(Debug)]
pub struct FindTodosRepositoryError {
    pub msg: String,
}

impl fmt::Display for FindTodosRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for FindTodosRepositoryError {}

#[async_trait]
pub trait FindTodosRepository<ReadCriteria, ReadModel>: Sync {
    async fn find(&self, criteria: ReadCriteria) -> Result<ReadModel, FindTodosRepositoryError>;
}
