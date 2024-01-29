use async_trait::async_trait;
use crate::{
    backoffice::commands::application::use_cases::update_todo_use_case::{
        UpdateTodoUseCase,
        UpdateTodoRequestModel,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct UpdateTodoRequestObject {
    pub id: String,
    pub name: String,
}

pub struct UpdateTodoController<'a> {
    use_case: UpdateTodoUseCase<'a>,
}

impl<'a> UpdateTodoController<'a> {
    pub fn new(use_case: UpdateTodoUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<UpdateTodoRequestObject> for UpdateTodoController<'a> {
    async fn execute(&self, request_object: UpdateTodoRequestObject) {
        self.use_case.interact(UpdateTodoRequestModel {
            id: request_object.id,
            name: request_object.name,
        }).await;
    }
}
