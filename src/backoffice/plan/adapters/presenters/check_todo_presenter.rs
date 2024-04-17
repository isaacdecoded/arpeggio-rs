use async_trait::async_trait;
use std::error::Error;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::backoffice::plan::application::commands::check_todo_use_case::CheckTodoResponseModel;

pub struct CheckTodoPresenter;

#[async_trait]
impl UseCaseOutputPort<CheckTodoResponseModel> for CheckTodoPresenter {
    async fn success(&self, response_model: CheckTodoResponseModel) {
        let id = response_model.id;
        println!("===");
        println!("CheckTodoPresenter: Todo with ID <{}> successfully checked.", id);
        println!("===");
    }

    async fn failure(&self, error: Box<dyn Error + Send + Sync>) {
        eprintln!("{}", error);
    }
}
