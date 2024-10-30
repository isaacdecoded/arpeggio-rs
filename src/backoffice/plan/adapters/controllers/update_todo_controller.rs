use crate::{
    backoffice::plan::application::commands::update_todo_use_case::{
        UpdateTodoRequestModel, UpdateTodoUseCase,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};
use async_trait::async_trait;

pub struct UpdateTodoRequestObject {
    pub plan_id: String,
    pub todo_id: String,
    pub description: String,
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
        self.use_case
            .interact(UpdateTodoRequestModel {
                plan_id: request_object.plan_id,
                todo_id: request_object.todo_id,
                description: request_object.description,
            })
            .await
    }
}
