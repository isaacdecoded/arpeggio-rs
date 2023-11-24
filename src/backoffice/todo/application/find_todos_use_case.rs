use async_trait::async_trait;
use std::mem::ManuallyDrop;
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::repositories::criteria::{
            Criteria,
            Filter,
            FilterOperator,
            FilterValue,
        },
    },
    backoffice::todo::domain::{
        entities::todo::Todo,
        repositories::todo_repository::TodoRepository,
    },
};

pub struct FindTodosInputData {
    pub name: Option<String>,
    pub offset: u16,
    pub limit: u16,
}

pub struct FindTodosOutputData {
    pub todos: Vec<Todo>
}

pub struct FindTodosUseCase<'a> {
    repository: &'a dyn TodoRepository,
    output_port: Box<dyn UseCaseOutputPort<FindTodosOutputData>>,
}

impl<'a> FindTodosUseCase<'a> {
    pub fn new(
        repository: &'a dyn TodoRepository,
        output_port: Box<dyn UseCaseOutputPort<FindTodosOutputData>>,
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<FindTodosInputData> for FindTodosUseCase<'a> {
    async fn interact(&self, input_data: FindTodosInputData) {
        let mut filters = Vec::new();
        if let Some(name) = input_data.name {
            filters.push(Filter {
                field: "name".to_string(),
                operator: FilterOperator::Equal,
                value: FilterValue { s: ManuallyDrop::new(name) },
            })
        }
        let criteria = Criteria::new(
            filters,
            Some(input_data.limit),
            Some(input_data.offset),
        );
        let result = self.repository.find(
            Some(criteria),
        ).await;
        match result {
            Ok(todos) => {
                self.output_port.success(FindTodosOutputData { todos }).await;
            },
            Err(error) => {
                self.output_port.failure(Box::new(error)).await;
            },
        }
    }
}
