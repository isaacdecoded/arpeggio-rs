use async_trait::async_trait;
use crate::{
    backoffice::commands::application::use_cases::create_todo_use_case::{
        CreateTodoUseCase,
        CreateTodoRequestModel,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct CreateTodoRequestObject {
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
impl<'a> Controller<CreateTodoRequestObject> for CreateTodoController<'a> {
    async fn execute(&self, request_object: CreateTodoRequestObject) {
        self.use_case.interact(CreateTodoRequestModel {
            name: request_object.name,
        }).await;
    }
}
