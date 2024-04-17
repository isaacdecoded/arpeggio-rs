use async_trait::async_trait;
use std::error::Error;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::backoffice::plan::application::commands::remove_todo_use_case::RemoveTodoResponseModel;

pub struct RemoveTodoPresenter;

#[async_trait]
impl UseCaseOutputPort<RemoveTodoResponseModel> for RemoveTodoPresenter {
    async fn success(&self, response_model: RemoveTodoResponseModel) {
        let id = response_model.id;
        println!("===");
        println!("RemoveTodoPresenter: Todo with ID <{}> successfully removed.", id);
        println!("===");
    }

    async fn failure(&self, error: Box<dyn Error + Send + Sync>) {
        eprintln!("{}", error);
    }
}
