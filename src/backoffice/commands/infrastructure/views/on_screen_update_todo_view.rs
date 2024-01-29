use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::commands::adapters::presenters::update_todo_presenter::UpdateTodoViewModel,
};

pub struct OnScreenUpdateTodoView {}

#[async_trait]
impl View<UpdateTodoViewModel> for OnScreenUpdateTodoView {
    async fn transform(&self, view_model: UpdateTodoViewModel) {
        if let Some(id) = view_model.id {
            println!("----------------------");
            println!("OnScreenUpdateTodoView:");
            println!("\tTodo with id <{}> successfully updated", id);
        }
        if let Some(error) = view_model.error {
            println!("Update Todo failed due to <{}>.", error.to_string())
        }
    }
}
