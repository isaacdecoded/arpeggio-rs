use async_trait::async_trait;
use crate::core::{
    adapters::controller::Controller,
    application::use_case_input_port::UseCaseInputPort,
};
use crate::backoffice::plan::application::commands::add_todo_use_case::{
    AddTodoUseCase,
    AddTodoRequestModel,
};

pub struct AddTodoRequestObject {
    pub plan_id: String,
    pub description: String,
}

pub struct AddTodoController<'a> {
    use_case: AddTodoUseCase<'a>,
}

impl<'a> AddTodoController<'a> {
    pub fn new(use_case: AddTodoUseCase<'a>) -> Self {
        AddTodoController { use_case }
    }
}

#[async_trait]
impl<'a> Controller<AddTodoRequestObject> for AddTodoController<'a> {
    async fn execute(&self, request_object: AddTodoRequestObject) {
        self.use_case.interact(AddTodoRequestModel {
            plan_id: request_object.plan_id,
            description: request_object.description,
        }).await
    }
}
