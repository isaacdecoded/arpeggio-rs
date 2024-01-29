use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::commands::adapters::presenters::create_todo_presenter::CreateTodoViewModel,
};

pub struct OnScreenCreateTodoView {}

#[async_trait]
impl View<CreateTodoViewModel> for OnScreenCreateTodoView {
    async fn transform(&self, view_model: CreateTodoViewModel) {
        if let Some(id) = view_model.id {
            println!("----------------------");
            println!("OnScreenCreateTodoView:");
            println!("\tTodo with id <{}> successfully created", id);
        }
        if let Some(error) = view_model.error {
            println!("Todo creation failed due to <{}>.", error.to_string())
        }
    }
}
