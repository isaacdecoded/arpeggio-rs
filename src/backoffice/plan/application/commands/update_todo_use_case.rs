use async_trait::async_trait;
use std::error::Error;
use thiserror::Error;
use crate::{
    backoffice::plan::domain::{
        repositories::plan_repository::PlanRepository,
        value_objects::todo_description::TodoDescription,
    },
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::models::{ identity_object::IdentityObject, value_object::ValueObject },
    },
};

#[derive(Error, Debug)]
pub enum UpdateTodoUseCaseError {
    #[error("Unable to update Todo: {0}")] TodoNotUpdatedError(String),
}

pub struct UpdateTodoRequestModel {
    pub plan_id: String,
    pub todo_id: String,
    pub description: String,
}

pub struct UpdateTodoResponseModel {
    pub todo_id: String,
}

pub struct UpdateTodoUseCase<'a> {
    repository: &'a dyn PlanRepository,
    output_port: &'a dyn UseCaseOutputPort<UpdateTodoResponseModel>,
}

impl<'a> UpdateTodoUseCase<'a> {
    pub fn new(
        repository: &'a dyn PlanRepository,
        output_port: &'a dyn UseCaseOutputPort<UpdateTodoResponseModel>
    ) -> Self {
        Self { repository, output_port }
    }

    async fn try_interact(
        &self,
        request_model: UpdateTodoRequestModel
    ) -> Result<UpdateTodoResponseModel, Box<dyn Error + Send + Sync>> {
        let plan_id = IdentityObject::new(request_model.plan_id)?;
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(mut plan) => {
                let todo_id = IdentityObject::new(request_model.todo_id)?;
                plan.change_todo_description(
                    &todo_id,
                    &TodoDescription::new(request_model.description)?
                )?;
                self.repository.save(&plan).await?;
                Ok(UpdateTodoResponseModel { todo_id: todo_id.get_value().to_string() })
            }
            None => {
                Err(
                    UpdateTodoUseCaseError::TodoNotUpdatedError(
                        format!("Plan with ID <{}> do not exist", plan_id.get_value())
                    ).into()
                )
            }
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<UpdateTodoRequestModel> for UpdateTodoUseCase<'a> {
    async fn interact(&self, request_model: UpdateTodoRequestModel) {
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
