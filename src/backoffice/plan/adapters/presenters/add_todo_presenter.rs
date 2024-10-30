use crate::backoffice::plan::application::commands::add_todo_use_case::AddTodoResponseModel;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use async_trait::async_trait;
use std::error::Error;

pub struct AddTodoPresenter {
    todo_id_catcher: Box<dyn Fn(String) + Sync + Send>,
}

impl AddTodoPresenter {
    pub fn new(todo_id_catcher: impl Fn(String) + 'static + Send + Sync) -> Self {
        Self {
            todo_id_catcher: Box::new(todo_id_catcher),
        }
    }
}

#[async_trait]
impl UseCaseOutputPort<AddTodoResponseModel> for AddTodoPresenter {
    async fn success(&self, response_model: AddTodoResponseModel) {
        let id = response_model.todo_id;
        println!("===");
        println!(
            "AddTodoPresenter: Todo with ID <{}> successfully added.",
            id
        );
        println!("===");
        (self.todo_id_catcher)(id);
    }

    async fn failure(&self, error: Box<dyn Error + Send + Sync>) {
        eprintln!("{}", error);
    }
}
