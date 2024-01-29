use async_trait::async_trait;
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::entities::{ value_object::ValueObject, identity_object::IdentityObject },
    },
    backoffice::commands::application::errors::todo_not_updated_error::TodoNotUpdatedError,
    backoffice::commands::domain::aggregates::update_todo::{
        entities::todo::UpdateTodoProps,
        value_objects::todo_name::TodoName,
        repositories::update_todo_repository::UpdateTodoRepository,
    },
};

pub struct UpdateTodoRequestModel {
    pub id: String,
    pub name: String,
}

pub struct UpdateTodoResponseModel {
    pub id: IdentityObject,
}

pub struct UpdateTodoUseCase<'a> {
    repository: &'a dyn UpdateTodoRepository,
    output_port: Box<dyn UseCaseOutputPort<UpdateTodoResponseModel>>,
}

impl<'a> UpdateTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn UpdateTodoRepository,
        output_port: Box<dyn UseCaseOutputPort<UpdateTodoResponseModel>>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<UpdateTodoRequestModel> for UpdateTodoUseCase<'a> {
    async fn interact(&self, request_model: UpdateTodoRequestModel) {
        let id = IdentityObject::new(request_model.id);
        let result = self.repository.get_by_id(&id).await;
        match result {
            Ok(todo) => {
                match todo {
                    Some(mut todo) => {
                        todo.update(UpdateTodoProps {
                            name: TodoName::new(request_model.name),
                        });
                        let result = self.repository.save(&todo).await;
                        match result {
                            Ok(()) =>
                                self.output_port.success(UpdateTodoResponseModel { id }).await,
                            Err(error) =>
                                self.output_port.failure(
                                    Box::new(TodoNotUpdatedError::new(error.msg))
                                ).await,
                        }
                    }
                    None => {
                        self.output_port.failure(
                            Box::new(
                                TodoNotUpdatedError::new(format!("Todo with ID <id> do not exist"))
                            )
                        ).await
                    }
                }
            }
            Err(error) => {
                self.output_port.failure(Box::new(TodoNotUpdatedError::new(error.msg))).await;
            }
        }
    }
}
