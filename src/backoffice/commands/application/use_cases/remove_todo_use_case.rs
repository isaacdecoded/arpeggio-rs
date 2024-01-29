use async_trait::async_trait;
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::entities::{ identity_object::IdentityObject, value_object::ValueObject },
    },
    backoffice::commands::application::errors::todo_not_removed_error::TodoNotRemovedError,
    backoffice::commands::domain::aggregates::remove_todo::repositories::remove_todo_repository::RemoveTodoRepository,
};

pub struct RemoveTodoRequestModel {
    pub id: String,
}

pub struct RemoveTodoResponseModel {
    pub removed: bool,
}

pub struct RemoveTodoUseCase<'a> {
    repository: &'a dyn RemoveTodoRepository,
    output_port: Box<dyn UseCaseOutputPort<RemoveTodoResponseModel>>,
}

impl<'a> RemoveTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn RemoveTodoRepository,
        output_port: Box<dyn UseCaseOutputPort<RemoveTodoResponseModel>>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<RemoveTodoRequestModel> for RemoveTodoUseCase<'a> {
    async fn interact(&self, request_model: RemoveTodoRequestModel) {
        let id = IdentityObject::new(request_model.id);
        let result = self.repository.get_by_id(&id).await;
        match result {
            Ok(todo) => {
                match todo {
                    Some(mut todo) => {
                        todo.remove();
                        let result = self.repository.save(&todo).await;
                        match result {
                            Ok(()) =>
                                self.output_port.success(RemoveTodoResponseModel {
                                    removed: true,
                                }).await,
                            Err(error) =>
                                self.output_port.failure(
                                    Box::new(TodoNotRemovedError::new(error.msg))
                                ).await,
                        }
                    }
                    None => {
                        self.output_port.failure(
                            Box::new(
                                TodoNotRemovedError::new(format!("Todo with ID <id> do not exist"))
                            )
                        ).await
                    }
                }
            }
            Err(error) => {
                self.output_port.failure(Box::new(TodoNotRemovedError::new(error.msg))).await;
            }
        }
    }
}
