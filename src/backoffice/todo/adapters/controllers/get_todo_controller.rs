use async_trait::async_trait;
use crate::{
    backoffice::todo::application::get_todo_use_case::{
        GetTodoUseCase,
        GetTodoInputData,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct GetTodoRequestModel {
    pub id: String,
}

pub struct GetTodoController<'a> {
    use_case: GetTodoUseCase<'a>,
}

impl<'a> GetTodoController<'a> {
    pub fn new(use_case: GetTodoUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<GetTodoRequestModel> for GetTodoController<'a> {
    async fn execute(&self, request_model: GetTodoRequestModel) {
        self.use_case.interact(GetTodoInputData {
            id: request_model.id,
        }).await;
    }
}

