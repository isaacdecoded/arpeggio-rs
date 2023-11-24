use async_trait::async_trait;
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::entities::{
            value_object::ValueObject,
            string_value_object::StringValueObject,
        },
    },
    backoffice::todo::domain::{
        entities::todo::Todo,
        errors::todo_not_found_error::TodoNotFoundError,
        repositories::todo_repository::TodoRepository,
    },
};

pub struct GetTodoInputData {
  pub id: String
}

pub struct GetTodoOutputData {
    pub todo: Todo
}

pub struct GetTodoUseCase<'a> {
    repository: &'a dyn TodoRepository,
    output_port: Box<dyn UseCaseOutputPort<GetTodoOutputData>>,
}

impl<'a> GetTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn TodoRepository,
        output_port: Box<dyn UseCaseOutputPort<GetTodoOutputData>>,
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<GetTodoInputData> for GetTodoUseCase<'a> {
    async fn interact(&self, input_data: GetTodoInputData) {
        let id = StringValueObject::new(input_data.id);
        let result = self.repository.get_by_id(&id).await;
        match result {
            Ok(todo) => {
                match todo {
                    Some(todo) => {
                        self.output_port.success(GetTodoOutputData { todo }).await
                    },
                    None => {
                        self.output_port.failure(
                            Box::new(TodoNotFoundError::new(&id)),
                        ).await
                    },
                }
            },
            Err(error) => {
                self.output_port.failure(Box::new(error)).await;
            },
        }
    }
}
