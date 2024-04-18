use async_trait::async_trait;
use std::error::Error;
use crate::{
    backoffice::plan::domain::{
        entities::{ plan::CreatePlanProps, plan::Plan },
        repositories::plan_repository::PlanRepository,
        value_objects::plan_name::PlanName,
    },
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::{
            events::domain_event_bus::DomainEventBus,
            models::{ aggregate_root::AggregateRoot, entity::Entity, value_object::ValueObject },
        },
    },
};

pub struct CreatePlanRequestModel {
    pub name: String,
}

pub struct CreatePlanResponseModel {
    pub plan_id: String,
}

pub struct CreatePlanUseCase<'a> {
    repository: &'a dyn PlanRepository,
    domain_event_bus: &'a dyn DomainEventBus,
    output_port: &'a dyn UseCaseOutputPort<CreatePlanResponseModel>,
}

impl<'a> CreatePlanUseCase<'a> {
    pub fn new(
        repository: &'a dyn PlanRepository,
        domain_event_bus: &'a dyn DomainEventBus,
        output_port: &'a dyn UseCaseOutputPort<CreatePlanResponseModel>
    ) -> Self {
        Self {
            repository,
            output_port,
            domain_event_bus,
        }
    }

    async fn try_interact(
        &self,
        request_model: CreatePlanRequestModel
    ) -> Result<CreatePlanResponseModel, Box<dyn Error + Send + Sync>> {
        let id = self.repository.generate_id().await?;
        let mut plan = Plan::create(CreatePlanProps {
            id,
            name: PlanName::new(request_model.name)?,
            todos: None,
        });
        self.domain_event_bus.publish(plan.pull_domain_events()).await?;
        self.repository.save(&plan).await?;
        Ok(CreatePlanResponseModel {
            plan_id: plan.get_id().to_string(),
        })
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<CreatePlanRequestModel> for CreatePlanUseCase<'a> {
    async fn interact(&self, request_model: CreatePlanRequestModel) {
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
