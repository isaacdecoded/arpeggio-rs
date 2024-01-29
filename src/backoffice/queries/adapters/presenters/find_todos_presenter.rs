use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{ adapters::view::View, application::use_case_output_port::UseCaseOutputPort },
    backoffice::queries::application::use_cases::find_todos_use_case::{
        FindTodosResponseModel,
        FindTodosReadModel,
    },
};

pub struct FindTodosViewModel {
    pub todos: Option<Vec<FindTodosReadModel>>,
    pub error: Option<Box<dyn Error + Send>>,
}

pub struct FindTodosPresenter {
    view: Box<dyn View<FindTodosViewModel>>,
}

impl FindTodosPresenter {
    pub fn new(view: Box<dyn View<FindTodosViewModel>>) -> Self {
        Self { view }
    }
}

#[async_trait]
impl UseCaseOutputPort<FindTodosResponseModel> for FindTodosPresenter {
    async fn success(&self, response_model: FindTodosResponseModel) {
        self.view.transform(FindTodosViewModel {
            todos: Some(response_model.todos),
            error: None,
        }).await;
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        self.view.transform(FindTodosViewModel {
            todos: None,
            error: Some(error),
        }).await;
    }
}
