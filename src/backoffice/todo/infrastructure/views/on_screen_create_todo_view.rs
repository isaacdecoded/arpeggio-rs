use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::todo::adapters::presenters::create_todo_presenter::CreateTodoViewModel,
};

pub struct OnScreenCreateTodoView {}

#[async_trait]
impl View<CreateTodoViewModel> for OnScreenCreateTodoView {
    async fn transform(&self, view_model: CreateTodoViewModel) {
        if let Some(id) = view_model.id {
            println!("Todo with id <{}> successfully created.", id);
        }
        if let Some(error) = view_model.error {
            println!("Todo creation failed due to <{}>.", error.to_string())
        }
    }
}

/*
from core.adapters import View
from backoffice.todo.adapters.presenters import CreateTodoViewModel


class OnScreenCreateTodoView(View[CreateTodoViewModel]):
    def transform(self, view_model: CreateTodoViewModel):
        id = view_model.get("id")
        error = view_model.get("error")
        if error:
            print(error)

        if id:
            print(f"Todo with id <${id}> successfully created.")
*/