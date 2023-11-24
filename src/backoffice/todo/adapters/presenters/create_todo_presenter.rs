use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::adapters::view::View,
    core::application::use_case_output_port::UseCaseOutputPort,
    backoffice::todo::application::create_todo_use_case::CreateTodoOutputData,
};

pub struct CreateTodoViewModel {
    pub id: Option<String>,
    pub error: Option<Box<dyn Error + Send>>,
}

pub struct CreateTodoPresenter {
    view: Box<dyn View<CreateTodoViewModel>>,
}

impl CreateTodoPresenter {
    pub fn new(view: Box<dyn View<CreateTodoViewModel>>) -> Self {
        Self { view }
    }
}

#[async_trait]
impl UseCaseOutputPort<CreateTodoOutputData> for CreateTodoPresenter {
    async fn success(&self, output_data: CreateTodoOutputData) {
        self.view.transform(CreateTodoViewModel{
            id: Some(output_data.id),
            error: None,
        }).await;
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        self.view.transform(CreateTodoViewModel{
            id: None,
            error: Some(error),
        }).await;
    }
}

/*
class CreateTodoPresenter(UseCaseOutputPort[CreateTodoOutputData]):
    def __init__(self, view: View[CreateTodoViewModel]):
        self.view = view

    def success(self, output_data: CreateTodoOutputData):
        id = output_data.get("id")
        self.view.transform(
            {
                "id": id.value,
                "error": None,
            }
        )

    def failure(self, error: TodoNotSavedError | BaseException):
        self.view.transform(
            {
                "id": None,
                "error": error,
            }
        )
 */