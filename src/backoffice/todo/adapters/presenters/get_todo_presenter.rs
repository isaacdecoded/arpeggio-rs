use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{
        adapters::view::View,
        application::use_case_output_port::UseCaseOutputPort,
    },
    backoffice::todo::{
        domain::entities::todo::Todo,
        application::get_todo_use_case::GetTodoOutputData,
    },
};

pub struct GetTodoViewModel {
    pub todo: Option<Todo>,
    pub error: Option<Box<dyn Error + Send>>,
}

pub struct GetTodoPresenter {
    view: Box<dyn View<GetTodoViewModel>>,
}

impl GetTodoPresenter {
    pub fn new(view: Box<dyn View<GetTodoViewModel>>) -> Self {
        Self { view }
    }
}

#[async_trait]
impl UseCaseOutputPort<GetTodoOutputData> for GetTodoPresenter {
    async fn success(&self, output_data: GetTodoOutputData) {
        self.view.transform(GetTodoViewModel{
            todo: Some(output_data.todo),
            error: None,
        }).await;
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        self.view.transform(GetTodoViewModel{
            todo: None,
            error: Some(error),
        }).await;
    }
}