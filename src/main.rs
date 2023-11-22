pub mod core;
pub mod backoffice;

use crate::core::{
    adapters::controller::Controller,
    infrastructure::in_memory_event_bus::InMemoryEventBus
};
use crate::backoffice::todo::{
    adapters::{
        controllers::{
            create_todo_controller::{CreateTodoController, CreateTodoRequestModel},
            delete_todo_controller::{DeleteTodoController, DeleteTodoRequestModel},
        },
        presenters::{
            create_todo_presenter::CreateTodoPresenter,
            delete_todo_presenter::DeleteTodoPresenter,
        },
    },
    application::{
        create_todo_use_case::CreateTodoUseCase,
        delete_todo_use_case::DeleteTodoUseCase,
    },
    infrastructure::{
        repositories::in_memory_todo_repository::InMemoryTodoRepository,
        views::{
            on_screen_create_todo_view::OnScreenCreateTodoView,
            on_screen_delete_todo_view::OnScreenDeleteTodoView,
        },
    }
};

#[tokio::main]
async fn main() {
    // Setup implementation components
    let domain_event_bus = InMemoryEventBus::new();
    let todo_repository = InMemoryTodoRepository::new();

    // Prepare Use Cases
    let create_todo_presenter = CreateTodoPresenter::new(
        Box::new(OnScreenCreateTodoView{}),
    );
    let create_todo_use_case = CreateTodoUseCase::new(
        &todo_repository,
        &domain_event_bus,
        Box::new(create_todo_presenter),
    );

    let delete_todo_presenter = DeleteTodoPresenter::new(
        Box::new(OnScreenDeleteTodoView{}),
    );
    let delete_todo_use_case = DeleteTodoUseCase::new(
        &todo_repository,
        Box::new(delete_todo_presenter),
    );

    // Run controllers
    let mut create_todo_controller = CreateTodoController::new(create_todo_use_case);
    create_todo_controller.execute(CreateTodoRequestModel {
        name: "Todo1".to_string()
    }).await;

    let mut delete_todo_controller = DeleteTodoController::new(delete_todo_use_case);
    delete_todo_controller.execute(DeleteTodoRequestModel {
        id: Some("MyFirstTodoID".to_string())
    }).await;
}
