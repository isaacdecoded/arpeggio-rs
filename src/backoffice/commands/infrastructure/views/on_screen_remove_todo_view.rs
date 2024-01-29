use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::commands::adapters::presenters::remove_todo_presenter::RemoveTodoViewModel,
};

pub struct OnScreenRemoveTodoView {}

#[async_trait]
impl View<RemoveTodoViewModel> for OnScreenRemoveTodoView {
    async fn transform(&self, view_model: RemoveTodoViewModel) {
        if let Some(_) = view_model.removed {
            println!("----------------------");
            println!("OnScreenRemoveTodoView:");
            println!("\tTodo successfully removed");
        }
        if let Some(error) = view_model.error {
            println!("Remove Todo failed due to <{}>.", error.to_string())
        }
    }
}
