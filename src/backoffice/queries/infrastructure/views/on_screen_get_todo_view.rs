use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::queries::adapters::presenters::get_todo_presenter::GetTodoViewModel,
};

pub struct OnScreenGetTodoView {}

#[async_trait]
impl View<GetTodoViewModel> for OnScreenGetTodoView {
    async fn transform(&self, view_model: GetTodoViewModel) {
        if let Some(todo) = view_model.todo {
            let name = todo.name;
            let created_at = todo.created_at;
            let updated_at = todo.updated_at.is_some().to_string();
            println!("----------------------");
            println!("OnScreenGetTodoView:");
            println!("\tName: {name}");
            println!("\tCreatedAt: {created_at}");
            println!("\tUpdatedAt: {updated_at}");
        }
        if let Some(error) = view_model.error {
            println!("Get Todo failed due to <{}>.", error.to_string())
        }
    }
}
