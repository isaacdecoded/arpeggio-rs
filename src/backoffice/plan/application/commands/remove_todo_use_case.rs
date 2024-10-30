use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{
        application::use_case_input_port::UseCaseInputPort,
        domain::models::{ identity_object::IdentityObject, value_object::ValueObject },
    },
    backoffice::plan::{
        domain::repositories::plan_repository::PlanRepository,
        application::errors::todo_not_removed_error::TodoNotRemovedError,
    },
};

pub struct RemoveTodoRequestModel {
    pub plan_id: String,
    pub todo_id: String,
}

pub struct RemoveTodoResponseModel {
    pub removed: bool,
}

pub struct RemoveTodoUseCase<'a> {
    repository: &'a dyn PlanRepository,
}

impl<'a> RemoveTodoUseCase<'a> {
    pub fn new(repository: &'a dyn PlanRepository) -> Self {
        Self {
            repository,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<RemoveTodoRequestModel> for RemoveTodoUseCase<'a> {
    async fn interact(&self, request_model: RemoveTodoRequestModel) -> Result<(), Box<dyn Error>> {
        let plan_id = IdentityObject::new(request_model.plan_id.to_owned());
        let result = self.repository.get_by_id(&plan_id).await?;
        match result {
            Some(mut plan) => {
                plan.remove_todo(&IdentityObject::new(request_model.todo_id))?;
                self.repository.save(&plan).await?;
                Ok(())
            }
            None => {
                Err(
                    Box::new(
                        TodoNotRemovedError::new(
                            format!("Plan with ID <{}> do not exist", request_model.plan_id)
                        )
                    )
                )
            }
        }
    }
}
