use async_trait::async_trait;
use crate::{
    core::adapters::view::View,
    backoffice::todo::adapters::presenters::delete_todo_presenter::DeleteTodoViewModel,
};

pub struct OnScreenDeleteTodoView {}

#[async_trait]
impl View<DeleteTodoViewModel> for OnScreenDeleteTodoView {
    async fn transform(&self, view_model: DeleteTodoViewModel) {
        if let Some(total_deleted) = view_model.total_deleted {
            println!("Successfully deleted <{}> todos.", total_deleted);
        }
        if let Some(error) = view_model.error {
            println!("Delete Todos failed due to <{}>.", error.to_string())
        }
    }
}
