use async_trait::async_trait;
use std::error::Error;
use crate::core::domain::models::{ identity_object::IdentityObject, aggregate_root::AggregateRoot };
use crate::core::domain::events::domain_event_bus::DomainEventBus;
use crate::core::application::use_case_input_port::UseCaseInputPort;
use crate::backoffice::plan::domain::repositories::plan_repository::PlanRepository;
use crate::backoffice::plan::application::errors::todo_not_checked_error::TodoNotCheckedError;
use crate::core::domain::models::value_object::ValueObject;

pub struct CheckTodoRequestModel {
    pub plan_id: String,
    pub todo_id: String,
}

pub struct CheckTodoUseCase<'a> {
    repository: &'a dyn PlanRepository,
    domain_event_bus: &'a dyn DomainEventBus,
}

impl<'a> CheckTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn PlanRepository,
        domain_event_bus: &'a dyn DomainEventBus
    ) -> Self {
        Self {
            repository,
            domain_event_bus,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<CheckTodoRequestModel> for CheckTodoUseCase<'a> {
    async fn interact(&self, request_model: CheckTodoRequestModel) -> Result<(), Box<dyn Error>> {
        let plan_id = IdentityObject::new(request_model.plan_id);
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(mut plan) => {
                let todo_id = IdentityObject::new(request_model.todo_id);
                plan.mark_todo_as_done(&todo_id)?;
                self.repository.save(&plan).await?;
                self.domain_event_bus.publish(plan.pull_domain_events()).await?;
                Ok(())
            }
            None => {
                Err(
                    Box::new(
                        TodoNotCheckedError::new(
                            format!("Plan with ID <{}> do not exist", plan_id.get_value())
                        )
                    )
                )
            }
        }
    }
}
