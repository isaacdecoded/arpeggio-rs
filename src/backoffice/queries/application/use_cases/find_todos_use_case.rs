use async_trait::async_trait;
use chrono::{ DateTime, Local };
use crate::{
    core::application::{
        use_case_input_port::UseCaseInputPort,
        use_case_output_port::UseCaseOutputPort,
    },
    backoffice::queries::domain::repositories::find_todos_repository::FindTodosRepository,
};

pub struct FindTodosRequestModel {
    pub name: Option<String>,
    pub offset: u16,
    pub limit: u16,
}

pub struct FindTodosReadCriteria {
    name: Option<String>,
}

pub struct FindTodosReadModel {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Local>,
}

pub struct FindTodosResponseModel {
    pub todos: Vec<FindTodosReadModel>,
}

pub struct FindTodosUseCase<'a> {
    repository: &'a dyn FindTodosRepository<FindTodosReadCriteria, Vec<FindTodosReadModel>>,
    output_port: Box<dyn UseCaseOutputPort<FindTodosResponseModel>>,
}

impl<'a> FindTodosUseCase<'a> {
    pub fn new(
        repository: &'a dyn FindTodosRepository<FindTodosReadCriteria, Vec<FindTodosReadModel>>,
        output_port: Box<dyn UseCaseOutputPort<FindTodosResponseModel>>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<FindTodosRequestModel> for FindTodosUseCase<'a> {
    async fn interact(&self, request_model: FindTodosRequestModel) {
        let mut criteria = FindTodosReadCriteria {
            name: request_model.name,
        };
        let result = self.repository.find(criteria).await;
        match result {
            Ok(todos) => {
                self.output_port.success(FindTodosResponseModel { todos }).await;
            }
            Err(error) => {
                self.output_port.failure(Box::new(error)).await;
            }
        }
    }
}
