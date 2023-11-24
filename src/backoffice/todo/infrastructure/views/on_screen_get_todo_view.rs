use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::todo::adapters::presenters::get_todo_presenter::GetTodoViewModel,
};

pub struct OnScreenGetTodoView {}

#[async_trait]
impl View<GetTodoViewModel> for OnScreenGetTodoView {
    async fn transform(&self, view_model: GetTodoViewModel) {
        if let Some(todo) = view_model.todo {
            let id = todo.id();
            let name = todo.name();
            println!("{}", format!("1. {id} - {name}"));
        }
        if let Some(error) = view_model.error {
            println!("Get Todo failed due to <{}>.", error.to_string())
        }
    }
}
