use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::todo::adapters::presenters::find_todos_presenter::FindTodosViewModel,
};

pub struct OnScreenFindTodosView {}

#[async_trait]
impl View<FindTodosViewModel> for OnScreenFindTodosView {
    async fn transform(&self, view_model: FindTodosViewModel) {
        if let Some(todos) = view_model.todos {
            for (idx, todo) in todos.iter().enumerate() {
                let id = todo.id();
                let name = todo.name();
                let position = idx+1;
                println!("{}", format!("{position}. {id} - {name}"));
            }
        }
        if let Some(error) = view_model.error {
            println!("Find Todos failed due to <{}>.", error.to_string())
        }
    }
}
