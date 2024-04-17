use async_trait::async_trait;
use std::{ error::Error, time::SystemTime };
use thiserror::Error;
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::models::{ identity_object::IdentityObject, value_object::ValueObject },
    },
    backoffice::plan::domain::repositories::get_plan_repository::GetPlanRepository,
};

#[derive(Error, Debug)]
pub enum GetPlanUseCaseError {
    #[error("Unable to get Plan: {0}")] PlanNotFoundError(String),
}

pub struct GetPlanRequestModel {
    pub id: String,
}

pub struct PlanTodoReadModel {
    pub id: String,
    pub description: String,
    pub status: String,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

pub struct GetPlanReadModel {
    pub name: String,
    pub todos: Vec<PlanTodoReadModel>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

pub struct GetPlanResponseModel {
    pub plan: GetPlanReadModel,
}

pub struct GetPlanUseCase<'a> {
    repository: &'a dyn GetPlanRepository<GetPlanReadModel>,
    output_port: &'a dyn UseCaseOutputPort<GetPlanResponseModel>,
}

impl<'a> GetPlanUseCase<'a> {
    pub fn new(
        repository: &'a dyn GetPlanRepository<GetPlanReadModel>,
        output_port: &'a dyn UseCaseOutputPort<GetPlanResponseModel>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }

    async fn try_interact(
        &self,
        request_model: GetPlanRequestModel
    ) -> Result<GetPlanResponseModel, Box<dyn Error + Send + Sync>> {
        let plan_id = IdentityObject::new(request_model.id)?;
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(plan) => { Ok(GetPlanResponseModel { plan }) }
            None => {
                Err(
                    GetPlanUseCaseError::PlanNotFoundError(
                        format!("Plan with ID <{}> do not exist", plan_id.get_value())
                    ).into()
                )
            }
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<GetPlanRequestModel> for GetPlanUseCase<'a> {
    async fn interact(&self, request_model: GetPlanRequestModel) {
        match self.try_interact(request_model).await {
            Ok(response_model) => {
                self.output_port.success(response_model).await;
            }
            Err(error) => {
                self.output_port.failure(error).await;
            }
        }
    }
}
