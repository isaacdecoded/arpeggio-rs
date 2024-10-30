use crate::backoffice::plan::adapters::{
    controllers::{
        add_todo_controller::{AddTodoController, AddTodoRequestObject},
        check_todo_controller::{CheckTodoController, CheckTodoRequestObject},
        create_plan_controller::{CreatePlanController, CreatePlanRequestObject},
        find_plans_controller::{FindPlansController, FindPlansRequestObject},
        get_plan_controller::{GetPlanController, GetPlanRequestObject},
        remove_todo_controller::{RemoveTodoController, RemoveTodoRequestObject},
        update_todo_controller::{UpdateTodoController, UpdateTodoRequestObject},
    },
    presenters::{
        add_todo_presenter::AddTodoPresenter, check_todo_presenter::CheckTodoPresenter,
        create_plan_presenter::CreatePlanPresenter, find_plans_presenter::FindPlansPresenter,
        get_plan_presenter::GetPlanPresenter, remove_todo_presenter::RemoveTodoPresenter,
        update_todo_presenter::UpdateTodoPresenter,
    },
};
use crate::backoffice::plan::application::{
    commands::{
        add_todo_use_case::AddTodoUseCase, check_todo_use_case::CheckTodoUseCase,
        create_plan_use_case::CreatePlanUseCase, remove_todo_use_case::RemoveTodoUseCase,
        update_todo_use_case::UpdateTodoUseCase,
    },
    queries::{find_plans_use_case::FindPlansUseCase, get_plan_use_case::GetPlanUseCase},
};
use crate::backoffice::plan::infrastructure::repositories::{
    in_memory_find_plans_repository::InMemoryFindPlansRepository,
    in_memory_get_plan_repository::InMemoryGetPlanRepository,
    in_memory_plan_repository::InMemoryPlanRepository, in_memory_repository::InMemoryRepository,
};
use crate::core::adapters::controller::Controller;
use crate::core::domain::events::domain_event_bus::DomainEventBus;
use std::sync::{Arc, Mutex};

pub struct PlanAggregate<'a> {
    pub caught_plan_id: Arc<Mutex<String>>,
    pub caught_todo_id: Arc<Mutex<String>>,

    domain_event_bus: &'a dyn DomainEventBus,
    find_plans_repository: InMemoryFindPlansRepository,
    get_plan_repository: InMemoryGetPlanRepository,
    plan_repository: InMemoryPlanRepository,
}

impl<'a> PlanAggregate<'a> {
    pub fn new(domain_event_bus: &'a dyn DomainEventBus) -> Self {
        let caught_plan_id: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
        let caught_todo_id: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

        let in_memory_repository = InMemoryRepository::new();
        let find_plans_repository =
            InMemoryFindPlansRepository::new(Arc::clone(&in_memory_repository));
        let get_plan_repository = InMemoryGetPlanRepository::new(Arc::clone(&in_memory_repository));
        let plan_repository = InMemoryPlanRepository::new(Arc::clone(&in_memory_repository));

        PlanAggregate {
            domain_event_bus,
            find_plans_repository,
            get_plan_repository,
            plan_repository,
            caught_plan_id,
            caught_todo_id,
        }
    }

    pub async fn get_plan(&self, request_object: GetPlanRequestObject) {
        let presenter = GetPlanPresenter;
        let use_case = GetPlanUseCase::new(&self.get_plan_repository, &presenter);
        let controller = GetPlanController::new(&use_case);
        controller.execute(request_object).await;
    }

    pub async fn find_plans(&self, request_object: FindPlansRequestObject) {
        let presenter = FindPlansPresenter;
        let use_case = FindPlansUseCase::new(&self.find_plans_repository, &presenter);
        let controller = FindPlansController::new(use_case);
        controller.execute(request_object).await;
    }

    pub async fn create_plan(&self, request_object: CreatePlanRequestObject) {
        let presenter = CreatePlanPresenter::new({
            let caught_plan_id = Arc::clone(&self.caught_plan_id);
            move |id| {
                *caught_plan_id.lock().unwrap() = id;
            }
        });
        let use_case =
            CreatePlanUseCase::new(&self.plan_repository, self.domain_event_bus, &presenter);
        let controller = CreatePlanController::new(use_case);
        controller.execute(request_object).await;
    }

    pub async fn add_todo(&self, request_object: AddTodoRequestObject) {
        let presenter = AddTodoPresenter::new({
            let caught_todo_id = Arc::clone(&self.caught_todo_id);
            move |id| {
                *caught_todo_id.lock().unwrap() = id;
            }
        });
        let use_case =
            AddTodoUseCase::new(&self.plan_repository, self.domain_event_bus, &presenter);
        let controller = AddTodoController::new(use_case);
        controller.execute(request_object).await;
    }

    pub async fn update_todo(&self, request_object: UpdateTodoRequestObject) {
        let presenter = UpdateTodoPresenter;
        let use_case = UpdateTodoUseCase::new(&self.plan_repository, &presenter);
        let controller = UpdateTodoController::new(use_case);
        controller.execute(request_object).await;
    }

    pub async fn remove_todo(&self, request_object: RemoveTodoRequestObject) {
        let presenter = RemoveTodoPresenter;
        let use_case = RemoveTodoUseCase::new(&self.plan_repository, &presenter);
        let controller = RemoveTodoController::new(use_case);
        controller.execute(request_object).await;
    }

    pub async fn check_todo(&self, request_object: CheckTodoRequestObject) {
        let presenter = CheckTodoPresenter;
        let use_case =
            CheckTodoUseCase::new(&self.plan_repository, self.domain_event_bus, &presenter);
        let controller = CheckTodoController::new(use_case);
        let _ = controller.execute(request_object).await;
    }
}
