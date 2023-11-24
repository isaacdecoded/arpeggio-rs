use async_trait::async_trait;
use crate::{
    backoffice::todo::application::update_todo_use_case::{
        UpdateTodoUseCase,
        UpdateTodoInputData,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct UpdateTodoRequestModel {
    pub id: String,
    pub name: String,
}

pub struct UpdateTodoController<'a> {
    use_case: UpdateTodoUseCase<'a>
}

impl<'a> UpdateTodoController<'a> {
    pub fn new(use_case: UpdateTodoUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<UpdateTodoRequestModel> for UpdateTodoController<'a> {
    async fn execute(&self, request_model: UpdateTodoRequestModel) {
        self.use_case.interact(UpdateTodoInputData {
            id: request_model.id,
            name: request_model.name,
        }).await;
    }
}

