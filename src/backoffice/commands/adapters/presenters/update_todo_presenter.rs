use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{
        adapters::view::View,
        application::use_case_output_port::UseCaseOutputPort,
        domain::entities::value_object::ValueObject,
    },
    backoffice::commands::application::use_cases::update_todo_use_case::UpdateTodoResponseModel,
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
impl UseCaseOutputPort<UpdateTodoResponseModel> for UpdateTodoPresenter {
    async fn success(&self, response_model: UpdateTodoResponseModel) {
        self.view.transform(UpdateTodoViewModel {
            id: Some(response_model.id.get_value().to_string()),
            error: None,
        }).await;
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        self.view.transform(UpdateTodoViewModel {
            id: None,
            error: Some(error),
        }).await;
    }
}
