use async_trait::async_trait;
use std::error::Error;
use thiserror::Error;
use crate::core::domain::models::{ identity_object::IdentityObject, aggregate_root::AggregateRoot };
use crate::core::domain::events::domain_event_bus::DomainEventBus;
use crate::core::application::{
    use_case_input_port::UseCaseInputPort,
    use_case_output_port::UseCaseOutputPort,
};
use crate::backoffice::plan::domain::repositories::plan_repository::PlanRepository;
use crate::core::domain::models::value_object::ValueObject;

#[derive(Error, Debug)]
pub enum CheckTodoUseCaseError {
    #[error("Unable to check Todo: {0}")] TodoNotCheckedError(String),
}

pub struct CheckTodoRequestModel {
    pub plan_id: String,
    pub todo_id: String,
}

pub struct CheckTodoResponseModel {
    pub todo_id: String,
}

pub struct CheckTodoUseCase<'a> {
    repository: &'a dyn PlanRepository,
    domain_event_bus: &'a dyn DomainEventBus,
    output_port: &'a dyn UseCaseOutputPort<CheckTodoResponseModel>,
}

impl<'a> CheckTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn PlanRepository,
        domain_event_bus: &'a dyn DomainEventBus,
        output_port: &'a dyn UseCaseOutputPort<CheckTodoResponseModel>
    ) -> Self {
        Self {
            repository,
            domain_event_bus,
            output_port,
        }
    }

    async fn try_interact(
        &self,
        request_model: CheckTodoRequestModel
    ) -> Result<CheckTodoResponseModel, Box<dyn Error + Send + Sync>> {
        let plan_id = IdentityObject::new(request_model.plan_id)?;
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(mut plan) => {
                let todo_id = IdentityObject::new(request_model.todo_id)?;
                plan.mark_todo_as_done(&todo_id)?;
                self.repository.save(&plan).await?;
                self.domain_event_bus.publish(plan.pull_domain_events()).await?;
                Ok(CheckTodoResponseModel { todo_id: todo_id.get_value().to_string() })
            }
            None => {
                Err(
                    CheckTodoUseCaseError::TodoNotCheckedError(
                        format!("Plan with ID <{}> do not exist", plan_id.get_value())
                    ).into()
                )
            }
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<CheckTodoRequestModel> for CheckTodoUseCase<'a> {
    async fn interact(&self, request_model: CheckTodoRequestModel) {
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
