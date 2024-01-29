use async_trait::async_trait;
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::{
            entities::{
                aggregate_root::AggregateRoot,
                entity::Entity,
                identity_object::IdentityObject,
                value_object::ValueObject,
            },
            events::domain_event_bus::DomainEventBus,
        },
    },
    backoffice::commands::application::errors::todo_not_created_error::TodoNotCreatedError,
    backoffice::commands::domain::aggregates::create_todo::{
        entities::todo::Todo,
        repositories::create_todo_repository::CreateTodoRepository,
        value_objects::todo_name::TodoName,
        entities::todo::CreateTodoProps,
    },
};

pub struct CreateTodoRequestModel {
    pub name: String,
}

pub struct CreateTodoResponseModel {
    pub id: IdentityObject,
}

pub struct CreateTodoUseCase<'a> {
    repository: &'a dyn CreateTodoRepository,
    domain_event_bus: &'a dyn DomainEventBus<'a>,
    output_port: Box<dyn UseCaseOutputPort<CreateTodoResponseModel>>,
}

impl<'a> CreateTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn CreateTodoRepository,
        domain_event_bus: &'a dyn DomainEventBus<'a>,
        output_port: Box<dyn UseCaseOutputPort<CreateTodoResponseModel>>
    ) -> Self {
        Self {
            repository,
            output_port,
            domain_event_bus,
        }
    }
}

/*
impl From<TodoRepositoryError> for CreateTodoUseCase {
    fn from(value: TodoRepositoryError) -> Self {
        todo!()
    }
}

impl Into<UseCaseOutputPortError> for CreateTodoUseCase {
    fn into(self) -> UseCaseOutputPortError {
        self.output_port.failure(UseCaseOutputPortError {});
        todo!()
    }
}
*/

#[async_trait]
impl<'a> UseCaseInputPort<CreateTodoRequestModel> for CreateTodoUseCase<'a> {
    async fn interact(&self, request_model: CreateTodoRequestModel) {
        let result = self.repository.generate_id().await;
        match result {
            Ok(id) => {
                let mut todo = Todo::create(CreateTodoProps {
                    id: IdentityObject::new(id),
                    name: TodoName::new(request_model.name),
                });
                let saved = self.repository.save(&todo).await;
                match saved {
                    Ok(()) => {
                        self.domain_event_bus.publish(todo.pull_domain_events()).await;
                        self.output_port.success(CreateTodoResponseModel {
                            id: todo.get_id().to_owned(),
                        }).await;
                    }
                    Err(error) => {
                        self.output_port.failure(
                            Box::new(TodoNotCreatedError::new(error.msg))
                        ).await;
                    }
                }
            }
            Err(error) => {
                self.output_port.failure(Box::new(TodoNotCreatedError::new(error.msg))).await;
            }
        }
    }
}
