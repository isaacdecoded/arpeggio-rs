use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{ adapters::view::View, application::use_case_output_port::UseCaseOutputPort },
    backoffice::todo::{ application::find_todos_use_case::{ FindTodosOutputData, TodoViewModel } },
};

pub struct FindTodosViewModel {
    pub todos: Option<Vec<TodoViewModel>>,
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
impl UseCaseOutputPort<FindTodosOutputData> for FindTodosPresenter {
    async fn success(&self, output_data: FindTodosOutputData) {
        self.view.transform(FindTodosViewModel {
            todos: Some(output_data.todos),
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
