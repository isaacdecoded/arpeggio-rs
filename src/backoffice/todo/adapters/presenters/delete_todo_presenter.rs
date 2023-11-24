use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{
        adapters::view::View,
        application::use_case_output_port::UseCaseOutputPort,
    },
    backoffice::todo::application::delete_todo_use_case::DeleteTodoOutputData,
};

pub struct DeleteTodoViewModel {
    pub total_deleted: Option<u32>,
    pub error: Option<Box<dyn Error + Send>>,
}

pub struct DeleteTodoPresenter {
    view: Box<dyn View<DeleteTodoViewModel>>,
}

impl DeleteTodoPresenter {
    pub fn new(view: Box<dyn View<DeleteTodoViewModel>>) -> Self {
        Self { view }
    }
}

#[async_trait]
impl UseCaseOutputPort<DeleteTodoOutputData> for DeleteTodoPresenter {
    async fn success(&self, output_data: DeleteTodoOutputData) {
        self.view.transform(DeleteTodoViewModel{
            total_deleted: Some(output_data.total_deleted),
            error: None,
        }).await;
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        self.view.transform(DeleteTodoViewModel{
            total_deleted: None,
            error: Some(error),
        }).await;
    }
}