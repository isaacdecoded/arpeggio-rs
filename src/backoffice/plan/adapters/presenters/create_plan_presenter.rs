use async_trait::async_trait;
use std::error::Error;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::backoffice::plan::application::commands::create_plan_use_case::CreatePlanResponseModel;

pub struct CreatePlanPresenter {
    plan_id_catcher: Box<dyn Fn(String) + Sync + Send>,
}

impl CreatePlanPresenter {
    pub fn new(plan_id_catcher: impl Fn(String) + 'static + Send + Sync) -> Self {
        Self {
            plan_id_catcher: Box::new(plan_id_catcher),
        }
    }
}

#[async_trait]
impl UseCaseOutputPort<CreatePlanResponseModel> for CreatePlanPresenter {
    async fn success(&self, response_model: CreatePlanResponseModel) -> Result<(), Box<dyn Error>> {
        let id = response_model.id;
        println!("===");
        println!("Plan with ID <{}> successfully created.", id);
        println!("===");
        (self.plan_id_catcher)(id);
        Ok(())
    }

    async fn failure(&self, error: Box<dyn Error + Send>) -> Result<(), Box<dyn Error>> {
        eprintln!("{}", error.to_string());
        Ok(())
    }
}
