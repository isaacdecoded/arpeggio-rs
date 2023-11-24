use async_trait::async_trait;
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::entities::{
            entity::Updatable,
            value_object::ValueObject,
            string_value_object::StringValueObject,
        },
    },
    backoffice::todo::domain::{
        entities::todo::UpdateTodo,
        value_objects::todo_name::TodoName,
        errors::todo_not_found_error::TodoNotFoundError,
        repositories::todo_repository::TodoRepository,
    },
};

pub struct UpdateTodoInputData {
  pub id: String,
  pub name: String,
}

pub struct UpdateTodoOutputData {
    pub id: StringValueObject
}

pub struct UpdateTodoUseCase<'a> {
    repository: &'a dyn TodoRepository,
    output_port: Box<dyn UseCaseOutputPort<UpdateTodoOutputData>>,
}

impl<'a> UpdateTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn TodoRepository,
        output_port: Box<dyn UseCaseOutputPort<UpdateTodoOutputData>>,
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<UpdateTodoInputData> for UpdateTodoUseCase<'a> {
    async fn interact(&self, input_data: UpdateTodoInputData) {
        let id = StringValueObject::new(input_data.id);
        let result = self.repository.get_by_id(&id).await;
        match result {
            Ok(todo) => {
                match todo {
                    Some(mut todo) => {
                        todo.update(UpdateTodo{
                            name: TodoName::new(input_data.name),
                        });
                        let result = self.repository.save(&todo).await;
                        match result {
                            Ok(()) => self.output_port.success(UpdateTodoOutputData { id }).await,
                            Err(error) => self.output_port.failure(Box::new(error)).await,
                        }
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
