use async_trait::async_trait;
use std::error::Error;
use crate::core::{
    adapters::controller::Controller,
    application::use_case_input_port::UseCaseInputPort,
};
use crate::backoffice::plan::application::commands::check_todo_use_case::{
    CheckTodoUseCase,
    CheckTodoRequestModel,
};

pub struct CheckTodoRequestObject {
    pub plan_id: String,
    pub todo_id: String,
}

pub struct CheckTodoController<'a> {
    use_case: CheckTodoUseCase<'a>,
}

impl<'a> CheckTodoController<'a> {
    pub fn new(use_case: CheckTodoUseCase<'a>) -> Self {
        CheckTodoController { use_case }
    }
}

#[async_trait]
impl<'a> Controller<CheckTodoRequestObject> for CheckTodoController<'a> {
    async fn execute(&self, request_object: CheckTodoRequestObject) -> Result<(), Box<dyn Error>> {
        self.use_case.interact(CheckTodoRequestModel {
            plan_id: request_object.plan_id,
            todo_id: request_object.todo_id,
        }).await
    }
}
