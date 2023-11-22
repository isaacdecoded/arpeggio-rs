use async_trait::async_trait;
use crate::{
    backoffice::todo::application::delete_todo_use_case::{
        DeleteTodoUseCase,
        DeleteTodoInputData
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct DeleteTodoRequestModel {
    pub id: Option<String>
}

pub struct DeleteTodoController<'a> {
    use_case: DeleteTodoUseCase<'a>
}

impl<'a> DeleteTodoController<'a> {
    pub fn new(use_case: DeleteTodoUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<DeleteTodoRequestModel> for DeleteTodoController<'a> {
    async fn execute(&self, request_model: DeleteTodoRequestModel) {
        let id = request_model.id;
        self.use_case.interact(DeleteTodoInputData {
            id,
        }).await;
    }
}

