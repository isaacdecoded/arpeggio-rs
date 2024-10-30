use crate::{
    backoffice::plan::application::queries::get_plan_use_case::{
        GetPlanResponseModel, PlanTodoReadModel,
    },
    core::application::use_case_output_port::UseCaseOutputPort,
};
use async_trait::async_trait;
use chrono::{DateTime, Local};
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde::Serializer;
use serde_json;
use std::error::Error;

#[derive(Default)]
pub struct GetPlanPresenter;

impl Serialize for PlanTodoReadModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PlanTodoReadModel", 2)?;
        let created_at: DateTime<Local> = self.created_at.into();
        state.serialize_field("id", &self.id)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field(
            "created_at",
            &created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        )?;
        if let Some(updated_at) = self.updated_at {
            let datetime: DateTime<Local> = updated_at.into();
            state.serialize_field(
                "updated_at",
                &datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            )?;
        }
        state.end()
    }
}

#[async_trait]
impl UseCaseOutputPort<GetPlanResponseModel> for GetPlanPresenter {
    async fn success(&self, response_model: GetPlanResponseModel) {
        let created_at: DateTime<Local> = response_model.plan.created_at.into();
        println!("===");
        println!("Plan details:");
        println!("Name: {}", response_model.plan.name);
        println!(
            "Todos: {}",
            serde_json::to_string(&response_model.plan.todos).unwrap()
        );
        println!("CreatedAt: {}", created_at.format("%Y-%m-%d %H:%M:%S"));
        if let Some(updated_at) = response_model.plan.updated_at {
            let datetime: DateTime<Local> = updated_at.into();
            println!("UpdatedAt: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
        }
        println!("===");
    }

    async fn failure(&self, error: Box<dyn Error + Send + Sync>) {
        eprintln!("{}", error);
    }
}
