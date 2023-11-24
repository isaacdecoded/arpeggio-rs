use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::todo::adapters::presenters::update_todo_presenter::UpdateTodoViewModel,
};

pub struct OnScreenUpdateTodoView {}

#[async_trait]
impl View<UpdateTodoViewModel> for OnScreenUpdateTodoView {
    async fn transform(&self, view_model: UpdateTodoViewModel) {
        if let Some(id) = view_model.id {
            println!("Todo with id <{}> successfully updated.", id);
        }
        if let Some(error) = view_model.error {
            println!("Update Todo failed due to <{}>.", error.to_string())
        }
    }
}
