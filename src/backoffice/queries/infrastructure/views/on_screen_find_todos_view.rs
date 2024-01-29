use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::queries::adapters::presenters::find_todos_presenter::FindTodosViewModel,
};

pub struct OnScreenFindTodosView {}

#[async_trait]
impl View<FindTodosViewModel> for OnScreenFindTodosView {
    async fn transform(&self, view_model: FindTodosViewModel) {
        if let Some(todos) = view_model.todos {
            println!("----------------------");
            println!("OnScreenFindTodosView:");
            for (idx, todo) in todos.iter().enumerate() {
                let id = &todo.id;
                let name = &todo.name;
                let created_at = &todo.created_at;
                println!("\tTodo {idx}. {id} | {name} | {created_at}");
            }
        }
        if let Some(error) = view_model.error {
            println!("Find Todos failed due to <{}>.", error.to_string())
        }
    }
}
