use async_trait::async_trait;
use chrono::{ DateTime, Local };
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::entities::{ identity_object::IdentityObject, value_object::ValueObject },
    },
    backoffice::queries::{
        domain::repositories::get_todo_repository::GetTodoRepository,
        application::errors::todo_not_found_error::TodoNotFoundError,
    },
};

pub struct GetTodoRequestModel {
    pub id: String,
}

pub struct GetTodoReadModel {
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

pub struct GetTodoResponseModel {
    pub todo: GetTodoReadModel,
}

pub struct GetTodoUseCase<'a> {
    repository: &'a dyn GetTodoRepository<GetTodoReadModel>,
    output_port: Box<dyn UseCaseOutputPort<GetTodoResponseModel>>,
}

impl<'a> GetTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn GetTodoRepository<GetTodoReadModel>,
        output_port: Box<dyn UseCaseOutputPort<GetTodoResponseModel>>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<GetTodoRequestModel> for GetTodoUseCase<'a> {
    async fn interact(&self, request_model: GetTodoRequestModel) {
        let id = IdentityObject::new(request_model.id);
        let result = self.repository.get_by_id(id).await;
        match result {
            Ok(todo) => {
                match todo {
                    Some(todo) => {
                        self.output_port.success(GetTodoResponseModel {
                            todo,
                        }).await
                    }
                    None => {
                        self.output_port.failure(
                            Box::new(
                                TodoNotFoundError::new(format!("Todo with ID <id> do not exist"))
                            )
                        ).await
                    }
                }
            }
            Err(error) => {
                self.output_port.failure(Box::new(TodoNotFoundError::new(error.msg))).await;
            }
        }
    }
}
