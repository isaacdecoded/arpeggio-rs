use crate::{
    backoffice::plan::application::queries::find_plans_use_case::{
        FindPlansRequestModel, FindPlansUseCase,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};
use async_trait::async_trait;

pub struct FindPlansRequestObject {
    pub name: Option<String>,
    pub limit: u16,
    pub offset: u16,
}

pub struct FindPlansController<'a> {
    use_case: FindPlansUseCase<'a>,
}

impl<'a> FindPlansController<'a> {
    pub fn new(use_case: FindPlansUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<FindPlansRequestObject> for FindPlansController<'a> {
    async fn execute(&self, request_object: FindPlansRequestObject) {
        self.use_case
            .interact(FindPlansRequestModel {
                name: request_object.name,
                offset: request_object.offset,
                limit: request_object.limit,
            })
            .await
    }
}
