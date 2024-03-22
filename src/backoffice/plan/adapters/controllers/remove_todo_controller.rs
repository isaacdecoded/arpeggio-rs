use async_trait::async_trait;
use std::error::Error;
use crate::{
    backoffice::plan::application::commands::remove_todo_use_case::{
        RemoveTodoUseCase,
        RemoveTodoRequestModel,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct RemoveTodoRequestObject {
    pub plan_id: String,
    pub todo_id: String,
}

pub struct RemoveTodoController<'a> {
    use_case: RemoveTodoUseCase<'a>,
}

impl<'a> RemoveTodoController<'a> {
    pub fn new(use_case: RemoveTodoUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<RemoveTodoRequestObject> for RemoveTodoController<'a> {
    async fn execute(&self, request_object: RemoveTodoRequestObject) -> Result<(), Box<dyn Error>> {
        self.use_case.interact(RemoveTodoRequestModel {
            plan_id: request_object.plan_id,
            todo_id: request_object.todo_id,
        }).await
    }
}
