use async_trait::async_trait;
use std::error::Error;
use chrono::{ DateTime, Local };
use crate::{
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::models::{ identity_object::IdentityObject, value_object::ValueObject },
    },
    backoffice::plan::{
        domain::repositories::get_plan_repository::GetPlanRepository,
        application::errors::plan_not_found_error::PlanNotFoundError,
    },
};

pub struct GetPlanRequestModel {
    pub id: String,
}

pub struct PlanTodoReadModel {
    pub id: String,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

pub struct GetPlanReadModel {
    pub name: String,
    pub todos: Vec<PlanTodoReadModel>,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
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
}

#[async_trait]
impl<'a> UseCaseInputPort<GetPlanRequestModel> for GetPlanUseCase<'a> {
    async fn interact(&self, request_model: GetPlanRequestModel) -> Result<(), Box<dyn Error>> {
        let plan_id = IdentityObject::new(request_model.id);
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(plan) => {
                self.output_port.success(GetPlanResponseModel {
                    plan,
                }).await?;
                Ok(())
            }
            None => {
                self.output_port.failure(
                    Box::new(
                        PlanNotFoundError::new(
                            format!(
                                "Plan with ID <{}> do not exist",
                                plan_id.get_value().to_string()
                            )
                        )
                    )
                ).await?;
                Ok(())
            }
        }
    }
}
