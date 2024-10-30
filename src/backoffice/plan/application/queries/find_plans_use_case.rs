use async_trait::async_trait;
use std::error::Error;
use chrono::{ DateTime, Local };
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

pub struct FindPlansReadModel {
    pub id: String,
    pub name: String,
    pub todo_count: usize,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

pub struct FindPlansResponseModel {
    pub plans: Vec<FindPlansReadModel>,
}

pub struct FindPlansUseCase<'a> {
    repository: &'a dyn FindPlansRepository<FindPlansReadModel>,
    output_port: &'a dyn UseCaseOutputPort<FindPlansResponseModel>,
}

impl<'a> FindPlansUseCase<'a> {
    pub fn new(
        repository: &'a dyn FindPlansRepository<FindPlansReadModel>,
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
    async fn interact(&self, request_model: FindPlansRequestModel) -> Result<(), Box<dyn Error>> {
        let criteria = Criteria {
            // TODO: Implement filters
            // name: request_model.name,
            filters: Vec::new(),
            offset: Some(request_model.offset),
            limit: Some(request_model.limit),
        };
        let plans = self.repository.find(criteria).await?;
        self.output_port.success(FindPlansResponseModel { plans }).await?;
        Ok(())
        /*Err(error) => {
            self.output_port.failure(Box::new(error)).await;
        }*/
    }
}
