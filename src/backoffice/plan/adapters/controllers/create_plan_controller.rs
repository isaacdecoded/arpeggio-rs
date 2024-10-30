use crate::{
    backoffice::plan::application::commands::create_plan_use_case::{
        CreatePlanRequestModel, CreatePlanUseCase,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};
use async_trait::async_trait;

pub struct CreatePlanRequestObject {
    pub name: String,
}

pub struct CreatePlanController<'a> {
    use_case: CreatePlanUseCase<'a>,
}

impl<'a> CreatePlanController<'a> {
    pub fn new(use_case: CreatePlanUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<CreatePlanRequestObject> for CreatePlanController<'a> {
    async fn execute(&self, request_object: CreatePlanRequestObject) {
        self.use_case
            .interact(CreatePlanRequestModel {
                name: request_object.name,
            })
            .await
    }
}
