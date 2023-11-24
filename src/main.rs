pub mod core;
pub mod backoffice;

use crate::core::{
    adapters::controller::Controller,
    infrastructure::in_memory_event_bus::InMemoryEventBus
};
use crate::backoffice::todo::{
    adapters::{
        controllers::{
            find_todos_controller::{FindTodosController, FindTodosRequestModel},
            get_todo_controller::{GetTodoController, GetTodoRequestModel},
            create_todo_controller::{CreateTodoController, CreateTodoRequestModel},
            update_todo_controller::{UpdateTodoController, UpdateTodoRequestModel},
            delete_todo_controller::{DeleteTodoController, DeleteTodoRequestModel},
        },
        presenters::{
            find_todos_presenter::FindTodosPresenter,
            get_todo_presenter::GetTodoPresenter,
            create_todo_presenter::CreateTodoPresenter,
            update_todo_presenter::UpdateTodoPresenter,
            delete_todo_presenter::DeleteTodoPresenter,
        },
    },
    application::{
        find_todos_use_case::FindTodosUseCase,
        get_todo_use_case::GetTodoUseCase,
        create_todo_use_case::CreateTodoUseCase,
        update_todo_use_case::UpdateTodoUseCase,
        delete_todo_use_case::DeleteTodoUseCase,
    },
    infrastructure::{
        repositories::in_memory_todo_repository::InMemoryTodoRepository,
        views::{
            on_screen_find_todos_view::OnScreenFindTodosView,
            on_screen_get_todo_view::OnScreenGetTodoView,
            on_screen_create_todo_view::OnScreenCreateTodoView,
            on_screen_update_todo_view::OnScreenUpdateTodoView,
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

    let find_todos_presenter = FindTodosPresenter::new(
        Box::new(OnScreenFindTodosView{}),
    );
    let find_todos_use_case = FindTodosUseCase::new(
        &todo_repository,
        Box::new(find_todos_presenter),
    );

    let update_todo_presenter = UpdateTodoPresenter::new(
        Box::new(OnScreenUpdateTodoView{}),
    );
    let update_todo_use_case = UpdateTodoUseCase::new(
        &todo_repository,
        Box::new(update_todo_presenter),
    );

    let get_todo_presenter = GetTodoPresenter::new(
        Box::new(OnScreenGetTodoView{}),
    );
    let get_todo_use_case = GetTodoUseCase::new(
        &todo_repository,
        Box::new(get_todo_presenter),
    );

    let delete_todo_presenter = DeleteTodoPresenter::new(
        Box::new(OnScreenDeleteTodoView{}),
    );
    let delete_todo_use_case = DeleteTodoUseCase::new(
        &todo_repository,
        Box::new(delete_todo_presenter),
    );

    // Run controllers
    let create_todo_controller = CreateTodoController::new(create_todo_use_case);
    create_todo_controller.execute(CreateTodoRequestModel {
        name: "Todo1".to_string()
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
        id: Some("MyFirstTodoID".to_string())
    }).await;
}
