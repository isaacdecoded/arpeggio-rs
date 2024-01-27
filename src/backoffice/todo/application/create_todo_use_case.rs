use async_trait::async_trait;
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::{
            entities::{
                entity::Newable,
                value_object::ValueObject,
                string_value_object::StringValueObject,
                aggregate_root::AggregateRoot,
            },
            events::domain_event_bus::DomainEventBus,
        },
    },
    backoffice::todo::domain::{
        entities::todo::Todo,
        repositories::todo_repository::TodoRepository,
        value_objects::todo_name::TodoName,
        entities::todo::NewTodo,
    },
};

pub struct CreateTodoInputData {
    pub name: String,
}

pub struct CreateTodoOutputData {
    pub id: String,
}

pub struct CreateTodoUseCase<'a> {
    repository: &'a dyn TodoRepository,
    domain_event_bus: &'a dyn DomainEventBus<'a>,
    output_port: Box<dyn UseCaseOutputPort<CreateTodoOutputData>>,
}

impl<'a> CreateTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn TodoRepository,
        domain_event_bus: &'a dyn DomainEventBus<'a>,
        output_port: Box<dyn UseCaseOutputPort<CreateTodoOutputData>>
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
impl<'a> UseCaseInputPort<CreateTodoInputData> for CreateTodoUseCase<'a> {
    async fn interact(&self, input_data: CreateTodoInputData) {
        let result = self.repository.generate_id().await;
        match result {
            Ok(id) => {
                let mut todo = Todo::new(NewTodo {
                    id: StringValueObject::new(id),
                    name: TodoName::new(input_data.name),
                });
                let saved = self.repository.save(&todo).await;
                match saved {
                    Ok(()) => {
                        self.domain_event_bus.publish(todo.pull_domain_events()).await;
                        self.output_port.success(CreateTodoOutputData { id: todo.id() }).await;
                    }
                    Err(error) => {
                        self.output_port.failure(Box::new(error)).await;
                    }
                }
            }
            Err(error) => {
                self.output_port.failure(Box::new(error)).await;
            }
        }
    }
}
