use async_trait::async_trait;
use crate::{
    backoffice::queries::application::use_cases::find_todos_use_case::{
        FindTodosUseCase,
        FindTodosRequestModel,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

pub struct FindTodosRequestObject {
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
impl<'a> Controller<FindTodosRequestObject> for FindTodosController<'a> {
    async fn execute(&self, request_object: FindTodosRequestObject) {
        self.use_case.interact(FindTodosRequestModel {
            name: request_object.name,
            offset: request_object.offset,
            limit: request_object.limit,
        }).await;
    }
}
