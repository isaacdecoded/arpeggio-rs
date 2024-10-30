use crate::{
    backoffice::plan::domain::{
        repositories::plan_repository::PlanRepository,
        value_objects::todo_description::TodoDescription,
    },
    core::{
        application::{
            use_case_input_port::UseCaseInputPort, use_case_output_port::UseCaseOutputPort,
        },
        domain::{
            events::domain_event_bus::DomainEventBus,
            models::{
                aggregate_root::AggregateRoot, identity_object::IdentityObject,
                value_object::ValueObject,
            },
        },
    },
};
use async_trait::async_trait;
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AddTodoUseCaseError {
    #[error("Unable to add Todo: {0}")]
    TodoNotAddedError(String),
}

pub struct AddTodoRequestModel {
    pub plan_id: String,
    pub description: String,
}

pub struct AddTodoResponseModel {
    pub todo_id: String,
}

pub struct AddTodoUseCase<'a> {
    repository: &'a dyn PlanRepository,
    domain_event_bus: &'a dyn DomainEventBus,
    output_port: &'a dyn UseCaseOutputPort<AddTodoResponseModel>,
}

impl<'a> AddTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn PlanRepository,
        domain_event_bus: &'a dyn DomainEventBus,
        output_port: &'a dyn UseCaseOutputPort<AddTodoResponseModel>,
    ) -> Self {
        Self {
            repository,
            output_port,
            domain_event_bus,
        }
    }

    async fn try_interact(
        &self,
        request_model: AddTodoRequestModel,
    ) -> Result<AddTodoResponseModel, Box<dyn Error + Send + Sync>> {
        let plan_id = IdentityObject::new(request_model.plan_id)?;
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(mut plan) => {
                let id = self.repository.generate_id().await?;
                let description = TodoDescription::new(request_model.description)?;
                plan.add_todo(&id, &description)?;
                self.repository.save(&plan).await?;
                self.domain_event_bus
                    .publish(plan.pull_domain_events())
                    .await?;
                Ok(AddTodoResponseModel {
                    todo_id: id.get_value().to_string(),
                })
            }
            None => Err(AddTodoUseCaseError::TodoNotAddedError(format!(
                "Plan with ID <{}> do not exist",
                plan_id.get_value()
            ))
            .into()),
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<AddTodoRequestModel> for AddTodoUseCase<'a> {
    async fn interact(&self, request_model: AddTodoRequestModel) {
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
