use async_trait::async_trait;
use std::error::Error;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::backoffice::plan::application::commands::update_todo_use_case::UpdateTodoResponseModel;

pub struct UpdateTodoPresenter;

#[async_trait]
impl UseCaseOutputPort<UpdateTodoResponseModel> for UpdateTodoPresenter {
    async fn success(&self, response_model: UpdateTodoResponseModel) {
        let id = response_model.id;
        println!("===");
        println!("UpdateTodoPresenter: Todo with ID <{}> successfully updated.", id);
        println!("===");
    }

    async fn failure(&self, error: Box<dyn Error + Send + Sync>) {
        eprintln!("{}", error);
    }
}