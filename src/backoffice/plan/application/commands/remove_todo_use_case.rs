use crate::{
    backoffice::plan::domain::repositories::plan_repository::PlanRepository,
    core::{
        application::{
            use_case_input_port::UseCaseInputPort, use_case_output_port::UseCaseOutputPort,
        },
        domain::models::{identity_object::IdentityObject, value_object::ValueObject},
    },
};
use async_trait::async_trait;
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RemoveTodoUseCaseError {
    #[error("Unable to remove Todo: {0}")]
    TodoNotRemovedError(String),
}

pub struct RemoveTodoRequestModel {
    pub plan_id: String,
    pub todo_id: String,
}

pub struct RemoveTodoResponseModel {
    pub todo_id: String,
}

pub struct RemoveTodoUseCase<'a> {
    repository: &'a dyn PlanRepository,
    output_port: &'a dyn UseCaseOutputPort<RemoveTodoResponseModel>,
}

impl<'a> RemoveTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn PlanRepository,
        output_port: &'a dyn UseCaseOutputPort<RemoveTodoResponseModel>,
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }

    async fn try_interact(
        &self,
        request_model: RemoveTodoRequestModel,
    ) -> Result<RemoveTodoResponseModel, Box<dyn Error + Send + Sync>> {
        let plan_id = IdentityObject::new(request_model.plan_id.to_owned())?;
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(mut plan) => {
                let todo_id = IdentityObject::new(request_model.todo_id)?;
                plan.remove_todo(&todo_id)?;
                self.repository.save(&plan).await?;
                Ok(RemoveTodoResponseModel {
                    todo_id: todo_id.get_value().to_string(),
                })
            }
            None => Err(RemoveTodoUseCaseError::TodoNotRemovedError(format!(
                "Plan with ID <{}> do not exist",
                request_model.plan_id
            ))
            .into()),
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<RemoveTodoRequestModel> for RemoveTodoUseCase<'a> {
    async fn interact(&self, request_model: RemoveTodoRequestModel) {
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
