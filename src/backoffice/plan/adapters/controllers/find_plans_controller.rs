use async_trait::async_trait;
use std::error::Error;
use crate::{
    backoffice::plan::application::queries::find_plans_use_case::{
        FindPlansUseCase,
        FindPlansRequestModel,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};

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
    async fn execute(&self, request_object: FindPlansRequestObject) -> Result<(), Box<dyn Error>> {
        self.use_case.interact(FindPlansRequestModel {
            name: request_object.name,
            offset: request_object.offset,
            limit: request_object.limit,
        }).await
    }
}
