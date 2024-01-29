use async_trait::async_trait;
use crate::{
    backoffice::commands::application::use_cases::remove_todo_use_case::{
        RemoveTodoUseCase,
        RemoveTodoRequestModel,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct RemoveTodoRequestObject {
    pub id: String,
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
    async fn execute(&self, request_object: RemoveTodoRequestObject) {
        self.use_case.interact(RemoveTodoRequestModel {
            id: request_object.id,
        }).await;
    }
}
