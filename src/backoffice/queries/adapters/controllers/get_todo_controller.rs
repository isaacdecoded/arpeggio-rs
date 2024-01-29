use async_trait::async_trait;
use crate::{
    backoffice::queries::application::use_cases::get_todo_use_case::{
        GetTodoUseCase,
        GetTodoRequestModel,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct GetTodoRequestObject {
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
impl<'a> Controller<GetTodoRequestObject> for GetTodoController<'a> {
    async fn execute(&self, request_object: GetTodoRequestObject) {
        self.use_case.interact(GetTodoRequestModel {
            id: request_object.id,
        }).await;
    }
}
