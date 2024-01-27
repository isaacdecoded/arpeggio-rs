pub mod backoffice;
pub mod core;
use core::domain::events::domain_event_subscriber::DomainEventSubscriber;

use crate::backoffice::todo::{
    adapters::{
        controllers::{
            create_todo_controller::{ CreateTodoController, CreateTodoRequestModel },
            delete_todo_controller::{ DeleteTodoController, DeleteTodoRequestModel },
            find_todos_controller::{ FindTodosController, FindTodosRequestModel },
            get_todo_controller::{ GetTodoController, GetTodoRequestModel },
            update_todo_controller::{ UpdateTodoController, UpdateTodoRequestModel },
        },
        presenters::{
            create_todo_presenter::CreateTodoPresenter,
            delete_todo_presenter::DeleteTodoPresenter,
            find_todos_presenter::FindTodosPresenter,
            get_todo_presenter::GetTodoPresenter,
            update_todo_presenter::UpdateTodoPresenter,
        },
    },
    application::{
        create_todo_use_case::CreateTodoUseCase,
        delete_todo_use_case::DeleteTodoUseCase,
        find_todos_use_case::FindTodosUseCase,
        get_todo_use_case::GetTodoUseCase,
        update_todo_use_case::UpdateTodoUseCase,
        send_notification_on_todo_created_subscriber::SendNotificationOnTodoCreatedSubscriber,
    },
    infrastructure::{
        repositories::in_memory_todo_repository::InMemoryTodoRepository,
        views::{
            on_screen_create_todo_view::OnScreenCreateTodoView,
            on_screen_delete_todo_view::OnScreenDeleteTodoView,
            on_screen_find_todos_view::OnScreenFindTodosView,
            on_screen_get_todo_view::OnScreenGetTodoView,
            on_screen_update_todo_view::OnScreenUpdateTodoView,
        },
    },
};
use crate::core::{
    domain::events::domain_event_bus::DomainEventBus,
    adapters::controller::Controller,
    infrastructure::in_memory_domain_event_bus::InMemoryDomainEventBus,
};
// use async_trait::async_trait;

#[tokio::main]
async fn main() {
    // Setup implementation components
    let mut domain_event_bus = InMemoryDomainEventBus::new();
    let send_notification_on_todo_created_subscriber =
        SendNotificationOnTodoCreatedSubscriber::new();
    domain_event_bus.add_subscriber(&send_notification_on_todo_created_subscriber).await;
    let todo_repository = InMemoryTodoRepository::new();

    // Prepare Use Cases
    let create_todo_presenter = CreateTodoPresenter::new(Box::new(OnScreenCreateTodoView {}));
    let create_todo_use_case = CreateTodoUseCase::new(
        &todo_repository,
        &domain_event_bus,
        Box::new(create_todo_presenter)
    );

    let find_todos_presenter = FindTodosPresenter::new(Box::new(OnScreenFindTodosView {}));
    let find_todos_use_case = FindTodosUseCase::new(
        &todo_repository,
        Box::new(find_todos_presenter)
    );

    let update_todo_presenter = UpdateTodoPresenter::new(Box::new(OnScreenUpdateTodoView {}));
    let update_todo_use_case = UpdateTodoUseCase::new(
        &todo_repository,
        Box::new(update_todo_presenter)
    );

    let get_todo_presenter = GetTodoPresenter::new(Box::new(OnScreenGetTodoView {}));
    let get_todo_use_case = GetTodoUseCase::new(&todo_repository, Box::new(get_todo_presenter));

    let delete_todo_presenter = DeleteTodoPresenter::new(Box::new(OnScreenDeleteTodoView {}));
    let delete_todo_use_case = DeleteTodoUseCase::new(
        &todo_repository,
        Box::new(delete_todo_presenter)
    );

    // Run controllers
    let create_todo_controller = CreateTodoController::new(create_todo_use_case);
    create_todo_controller.execute(CreateTodoRequestModel {
        name: "Todo1".to_string(),
    }).await;

    let find_todos_controller = FindTodosController::new(find_todos_use_case);
    find_todos_controller.execute(FindTodosRequestModel {
        name: None,
        limit: 10,
        offset: 0,
    }).await;

    let update_todo_controller = UpdateTodoController::new(update_todo_use_case);
    update_todo_controller.execute(UpdateTodoRequestModel {
        id: "MyFirstTodoID".to_string(),
        name: "My First Todo (Updated)".to_string(),
    }).await;

    let get_todo_controller = GetTodoController::new(get_todo_use_case);
    get_todo_controller.execute(GetTodoRequestModel {
        id: "MyFirstTodoID".to_string(),
    }).await;

    let delete_todo_controller = DeleteTodoController::new(delete_todo_use_case);
    delete_todo_controller.execute(DeleteTodoRequestModel {
        id: Some("MyFirstTodoID".to_string()),
    }).await;
}
