use async_trait::async_trait;
use std::mem::ManuallyDrop;
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
                aggregate_root::AggregateRoot
            },
            repositories::criteria::{
                Criteria,
                Filter,
                FilterOperator,
                FilterValue,
            },
        }
    },
    backoffice::todo::domain::{
        entities::todo::Todo,
        repositories::todo_repository::TodoRepository,
        value_objects::todo_name::TodoName,
        entities::todo::NewTodo,
    },
};

pub struct DeleteTodoInputData {
  pub id: Option<String>
}

pub struct DeleteTodoOutputData {
    pub total_deleted: u32
}

pub struct DeleteTodoUseCase<'a> {
    repository: &'a dyn TodoRepository,
    output_port: Box<dyn UseCaseOutputPort<DeleteTodoOutputData>>,
}

impl<'a> DeleteTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn TodoRepository,
        output_port: Box<dyn UseCaseOutputPort<DeleteTodoOutputData>>,
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<DeleteTodoInputData> for DeleteTodoUseCase<'a> {
    async fn interact(&self, input_data: DeleteTodoInputData) {
        let mut filters = Vec::new();
        if let Some(id) = input_data.id {
            filters.push(Filter {
                field: "id".to_string(),
                operator: FilterOperator::Equal,
                value: FilterValue { s: ManuallyDrop::new(id) },
            })
        }
        let criteria = Criteria::new_with_filters_only(filters);
        let result = self.repository.delete(criteria).await;
        match result {
            Ok(total_deleted) => {
                self.output_port.success(DeleteTodoOutputData { total_deleted }).await;
            },
            Err(error) => {
                self.output_port.failure(Box::new(error)).await;
            },
        }
    }
}
