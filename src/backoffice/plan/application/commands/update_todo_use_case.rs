use async_trait::async_trait;
use std::error::Error;
use crate::{
    backoffice::plan::{
        application::errors::todo_not_updated_error::TodoNotUpdatedError,
        domain::{
            repositories::plan_repository::PlanRepository,
            value_objects::todo_description::TodoDescription,
        },
    },
    core::{
        application::use_case_input_port::UseCaseInputPort,
        domain::models::{ identity_object::IdentityObject, value_object::ValueObject },
    },
};

pub struct UpdateTodoRequestModel {
    pub plan_id: String,
    pub todo_id: String,
    pub description: String,
}

pub struct UpdateTodoResponseModel {
    pub id: IdentityObject,
}

pub struct UpdateTodoUseCase<'a> {
    repository: &'a dyn PlanRepository,
}

impl<'a> UpdateTodoUseCase<'a> {
    pub fn new(repository: &'a dyn PlanRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<UpdateTodoRequestModel> for UpdateTodoUseCase<'a> {
    async fn interact(&self, request_model: UpdateTodoRequestModel) -> Result<(), Box<dyn Error>> {
        let plan_id = IdentityObject::new(request_model.plan_id);
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(mut plan) => {
                plan.change_todo_description(
                    &IdentityObject::new(request_model.todo_id),
                    &TodoDescription::new(request_model.description)
                )?;
                self.repository.save(&plan).await?;
                Ok(())
            }
            None => {
                Err(
                    Box::new(
                        TodoNotUpdatedError::new(
                            format!("Plan with ID <{}> do not exist", plan_id.get_value())
                        )
                    )
                )
            }
        }
    }
}
