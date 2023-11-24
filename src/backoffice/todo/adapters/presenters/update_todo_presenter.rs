use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{
        domain::entities::value_object::ValueObject,
        application::use_case_output_port::UseCaseOutputPort,
        adapters::view::View,
    },
    backoffice::todo::application::update_todo_use_case::UpdateTodoOutputData,
};

pub struct UpdateTodoViewModel {
    pub id: Option<String>,
    pub error: Option<Box<dyn Error + Send>>,
}

pub struct UpdateTodoPresenter {
    view: Box<dyn View<UpdateTodoViewModel>>,
}

impl UpdateTodoPresenter {
    pub fn new(view: Box<dyn View<UpdateTodoViewModel>>) -> Self {
        Self { view }
    }
}

#[async_trait]
impl UseCaseOutputPort<UpdateTodoOutputData> for UpdateTodoPresenter {
    async fn success(&self, output_data: UpdateTodoOutputData) {
        self.view.transform(UpdateTodoViewModel{
            id: Some(output_data.id.value()),
            error: None,
        }).await;
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        self.view.transform(UpdateTodoViewModel{
            id: None,
            error: Some(error),
        }).await;
    }
}