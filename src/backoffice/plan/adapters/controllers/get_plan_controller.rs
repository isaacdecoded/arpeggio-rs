use crate::{
    backoffice::plan::application::queries::get_plan_use_case::{
        GetPlanRequestModel, GetPlanUseCase,
    },
    core::adapters::controller::Controller,
    core::application::use_case_input_port::UseCaseInputPort,
};
use async_trait::async_trait;

pub struct GetPlanRequestObject {
    pub id: String,
}

pub struct GetPlanController<'a> {
    use_case: &'a GetPlanUseCase<'a>,
}

impl<'a> GetPlanController<'a> {
    pub fn new(use_case: &'a GetPlanUseCase<'a>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<'a> Controller<GetPlanRequestObject> for GetPlanController<'a> {
    async fn execute(&self, request_object: GetPlanRequestObject) {
        self.use_case
            .interact(GetPlanRequestModel {
                id: request_object.id,
            })
            .await
    }
}
