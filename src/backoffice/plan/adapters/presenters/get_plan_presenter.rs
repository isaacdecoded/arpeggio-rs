use serde_json;
use async_trait::async_trait;
use std::error::Error;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use crate::{
    backoffice::plan::application::queries::get_plan_use_case::{
        GetPlanResponseModel,
        PlanTodoReadModel,
    },
    core::application::use_case_output_port::UseCaseOutputPort,
};

pub struct GetPlanPresenter;

impl GetPlanPresenter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Serialize for PlanTodoReadModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("PlanTodoReadModel", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("created_at", &self.created_at.to_string())?;
        if let Some(updated_at) = self.updated_at {
            state.serialize_field("updated_at", &updated_at.to_string())?;
        }
        state.end()
    }
}

#[async_trait]
impl UseCaseOutputPort<GetPlanResponseModel> for GetPlanPresenter {
    async fn success(&self, response_model: GetPlanResponseModel) -> Result<(), Box<dyn Error>> {
        println!("===");
        println!("Plan details:");
        println!("Name: {}", response_model.plan.name);
        println!("Todos: {}", serde_json::to_string(&response_model.plan.todos).unwrap());
        println!("CreatedAt: {}", response_model.plan.created_at);
        if let Some(updated_at) = response_model.plan.updated_at {
            println!("UpdatedAt: {}", updated_at.to_string());
        }
        println!("===");
        Ok(())
    }

    async fn failure(&self, error: Box<dyn Error + Send>) -> Result<(), Box<dyn Error>> {
        eprintln!("{}", error.to_string());
        Ok(())
    }
}
