use crate::{
    backoffice::plan::application::queries::find_plans_use_case::FindPlansResponseModel,
    core::application::use_case_output_port::UseCaseOutputPort,
};
use async_trait::async_trait;
use std::error::Error;

#[derive(Default)]
pub struct FindPlansPresenter;

#[async_trait]
impl UseCaseOutputPort<FindPlansResponseModel> for FindPlansPresenter {
    async fn success(&self, response_model: FindPlansResponseModel) {
        let mut info: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        for (idx, plan) in response_model.plans.iter().enumerate() {
            info.insert(
                format!("Plan {}", idx + 1),
                format!("{} | {} | {} todos", plan.id, plan.name, plan.todo_count),
            );
        }
        println!("===");
        println!("Plan list:");
        for (key, value) in info {
            println!("{}: {}", key, value);
        }
        println!("===");
    }

    async fn failure(&self, error: Box<dyn Error + Send + Sync>) {
        eprintln!("{}", error)
    }
}
