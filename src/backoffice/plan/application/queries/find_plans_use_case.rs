use std::time::SystemTime;
use async_trait::async_trait;
use crate::{
    backoffice::plan::domain::repositories::find_plans_repository::FindPlansRepository,
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::repositories::criteria::Criteria,
    },
};

pub struct FindPlansRequestModel {
    pub name: Option<String>,
    pub offset: u16,
    pub limit: u16,
}

pub struct PlanReadModel {
    pub id: String,
    pub name: String,
    pub todo_count: usize,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

pub struct FindPlansResponseModel {
    pub plans: Vec<PlanReadModel>,
}

pub struct FindPlansUseCase<'a> {
    repository: &'a dyn FindPlansRepository<PlanReadModel>,
    output_port: &'a dyn UseCaseOutputPort<FindPlansResponseModel>,
}

impl<'a> FindPlansUseCase<'a> {
    pub fn new(
        repository: &'a dyn FindPlansRepository<PlanReadModel>,
        output_port: &'a dyn UseCaseOutputPort<FindPlansResponseModel>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<FindPlansRequestModel> for FindPlansUseCase<'a> {
    async fn interact(&self, request_model: FindPlansRequestModel) {
        let criteria = Criteria {
            filters: Vec::new(),
            offset: Some(request_model.offset),
            limit: Some(request_model.limit),
        };
        match self.repository.find(criteria).await {
            Ok(plans) => {
                self.output_port.success(FindPlansResponseModel { plans }).await;
            }
            Err(error) => {
                self.output_port.failure(error.into()).await;
            }
        }
    }
}
