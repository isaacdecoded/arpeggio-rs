use async_trait::async_trait;
use std::error::Error;
use crate::{
    core::{ adapters::view::View, domain::entities::value_object::ValueObject },
    core::application::use_case_output_port::UseCaseOutputPort,
    backoffice::commands::application::use_cases::create_todo_use_case::CreateTodoResponseModel,
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
impl<'a> UseCaseOutputPort<CreateTodoResponseModel> for CreateTodoPresenter {
    async fn success(&self, response_model: CreateTodoResponseModel) {
        self.view.transform(CreateTodoViewModel {
            id: Some(response_model.id.get_value().to_string()),
            error: None,
        }).await;
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        self.view.transform(CreateTodoViewModel {
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
