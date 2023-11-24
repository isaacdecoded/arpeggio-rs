use async_trait::async_trait;
use crate::{
    backoffice::todo::application::find_todos_use_case::{
        FindTodosUseCase,
        FindTodosInputData,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct FindTodosRequestModel {
    pub name: Option<String>,
    pub limit: u16,
    pub offset: u16,
}

pub struct FindTodosController<'a> {
    use_case: FindTodosUseCase<'a>,
}

impl<'a> FindTodosController<'a> {
    pub fn new(use_case: FindTodosUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<FindTodosRequestModel> for FindTodosController<'a> {
    async fn execute(&self, request_model: FindTodosRequestModel) {
        self.use_case.interact(FindTodosInputData {
            name: request_model.name,
            offset: request_model.offset,
            limit: request_model.limit,
        }).await;
    }
}

