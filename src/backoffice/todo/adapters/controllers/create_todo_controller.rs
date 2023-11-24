use async_trait::async_trait;
use crate::{
    backoffice::todo::application::create_todo_use_case::{
        CreateTodoUseCase,
        CreateTodoInputData,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct CreateTodoRequestModel {
    pub name: String,
}

pub struct CreateTodoController<'a> {
    use_case: CreateTodoUseCase<'a>,
}

impl<'a> CreateTodoController<'a> {
    pub fn new(use_case: CreateTodoUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<CreateTodoRequestModel> for CreateTodoController<'a> {
    async fn execute(&self, request_model: CreateTodoRequestModel) {
        self.use_case.interact(CreateTodoInputData {
            name: request_model.name,
        }).await;
    }
}

