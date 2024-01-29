pub mod core;
pub mod backoffice;
use crate::backoffice::commands::{
    adapters::{
        controllers::{
            create_todo_controller::{ CreateTodoController, CreateTodoRequestObject },
            remove_todo_controller::{ RemoveTodoController, RemoveTodoRequestObject },
            update_todo_controller::{ UpdateTodoController, UpdateTodoRequestObject },
        },
        presenters::{
            create_todo_presenter::CreateTodoPresenter,
            remove_todo_presenter::RemoveTodoPresenter,
            update_todo_presenter::UpdateTodoPresenter,
        },
    },
    application::{
        use_cases::{
            create_todo_use_case::CreateTodoUseCase,
            remove_todo_use_case::RemoveTodoUseCase,
            update_todo_use_case::UpdateTodoUseCase,
        },
        subscribers::send_notification_on_todo_created_subscriber::SendNotificationOnTodoCreatedSubscriber,
    },
    infrastructure::{
        repositories::{
            in_memory_create_todo_repository::InMemoryCreateTodoRepository,
            in_memory_remove_todo_repository::InMemoryRemoveTodoRepository,
            in_memory_update_todo_repository::InMemoryUpdateTodoRepository,
        },
        services::on_screen_notification_service::OnScreenNotificationService,
        views::{
            on_screen_create_todo_view::OnScreenCreateTodoView,
            on_screen_remove_todo_view::OnScreenRemoveTodoView,
            on_screen_update_todo_view::OnScreenUpdateTodoView,
        },
    },
};
use crate::backoffice::queries::{
    adapters::{
        controllers::{
            find_todos_controller::{ FindTodosController, FindTodosRequestObject },
            get_todo_controller::{ GetTodoController, GetTodoRequestObject },
        },
        presenters::{
            find_todos_presenter::FindTodosPresenter,
            get_todo_presenter::GetTodoPresenter,
        },
    },
    application::use_cases::{
        find_todos_use_case::FindTodosUseCase,
        get_todo_use_case::GetTodoUseCase,
    },
    infrastructure::{
        repositories::{
            in_memory_find_todos_repository::InMemoryFindTodosRepository,
            in_memory_get_todo_repository::InMemoryGetTodoRepository,
        },
        views::{
            on_screen_find_todos_view::OnScreenFindTodosView,
            on_screen_get_todo_view::OnScreenGetTodoView,
        },
    },
};
use crate::core::{
    domain::events::domain_event_bus::DomainEventBus,
    adapters::controller::Controller,
    infrastructure::in_memory_domain_event_bus::InMemoryDomainEventBus,
};

#[tokio::main]
async fn main() {
    // Setup DomainEventBus and Subscribers
    let mut domain_event_bus = InMemoryDomainEventBus::new();
    let notification_service = OnScreenNotificationService::new();
    let send_notification_on_todo_created_subscriber = SendNotificationOnTodoCreatedSubscriber::new(
        &notification_service
    );
    domain_event_bus.add_subscriber(&send_notification_on_todo_created_subscriber).await;

    // Prepare Use Cases
    let create_todo_presenter = CreateTodoPresenter::new(Box::new(OnScreenCreateTodoView {}));
    let create_todo_repository = InMemoryCreateTodoRepository::new();
    let create_todo_use_case = CreateTodoUseCase::new(
        &create_todo_repository,
        &domain_event_bus,
        Box::new(create_todo_presenter)
    );

    let find_todos_presenter = FindTodosPresenter::new(Box::new(OnScreenFindTodosView {}));
    let find_todos_repository = InMemoryFindTodosRepository::new();
    let find_todos_use_case = FindTodosUseCase::new(
        &find_todos_repository,
        Box::new(find_todos_presenter)
    );

    let update_todo_presenter = UpdateTodoPresenter::new(Box::new(OnScreenUpdateTodoView {}));
    let update_todo_reposiroty = InMemoryUpdateTodoRepository::new();
    let update_todo_use_case = UpdateTodoUseCase::new(
        &update_todo_reposiroty,
        Box::new(update_todo_presenter)
    );

    let get_todo_presenter = GetTodoPresenter::new(Box::new(OnScreenGetTodoView {}));
    let get_todo_repository = InMemoryGetTodoRepository::new();
    let get_todo_use_case = GetTodoUseCase::new(&get_todo_repository, Box::new(get_todo_presenter));

    let remove_todo_presenter = RemoveTodoPresenter::new(Box::new(OnScreenRemoveTodoView {}));
    let remove_todo_repository = InMemoryRemoveTodoRepository::new();
    let remove_todo_use_case = RemoveTodoUseCase::new(
        &remove_todo_repository,
        Box::new(remove_todo_presenter)
    );

    // Run controllers
    let find_todos_controller = FindTodosController::new(find_todos_use_case);
    find_todos_controller.execute(FindTodosRequestObject {
        name: None,
        limit: 10,
        offset: 0,
    }).await;

    let create_todo_controller = CreateTodoController::new(create_todo_use_case);
    create_todo_controller.execute(CreateTodoRequestObject {
        name: "My First Todo".to_string(),
    }).await;

    let update_todo_controller = UpdateTodoController::new(update_todo_use_case);
    update_todo_controller.execute(UpdateTodoRequestObject {
        id: "MyFirstTodoID".to_string(),
        name: "My First Todo (Updated)".to_string(),
    }).await;

    let get_todo_controller = GetTodoController::new(get_todo_use_case);
    get_todo_controller.execute(GetTodoRequestObject {
        id: "MyFirstTodoID".to_string(),
    }).await;

    let remove_todo_controller = RemoveTodoController::new(remove_todo_use_case);
    remove_todo_controller.execute(RemoveTodoRequestObject {
        id: "MyFirstTodoID".to_string(),
    }).await;
}
