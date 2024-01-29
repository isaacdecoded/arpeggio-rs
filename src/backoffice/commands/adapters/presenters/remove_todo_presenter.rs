use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{ adapters::view::View, application::use_case_output_port::UseCaseOutputPort },
    backoffice::commands::application::use_cases::remove_todo_use_case::RemoveTodoResponseModel,
};

pub struct RemoveTodoViewModel {
    pub removed: Option<bool>,
    pub error: Option<Box<dyn Error + Send>>,
}

pub struct RemoveTodoPresenter {
    view: Box<dyn View<RemoveTodoViewModel>>,
}

impl RemoveTodoPresenter {
    pub fn new(view: Box<dyn View<RemoveTodoViewModel>>) -> Self {
        Self { view }
    }
}

#[async_trait]
impl UseCaseOutputPort<RemoveTodoResponseModel> for RemoveTodoPresenter {
    async fn success(&self, response_model: RemoveTodoResponseModel) {
        self.view.transform(RemoveTodoViewModel {
            removed: Some(response_model.removed),
            error: None,
        }).await;
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        self.view.transform(RemoveTodoViewModel {
            removed: None,
            error: Some(error),
        }).await;
    }
}
