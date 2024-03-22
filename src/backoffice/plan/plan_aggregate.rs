use std::sync::{ Arc, Mutex };
use crate::core::adapters::controller::Controller;
use crate::core::domain::events::domain_event_bus::DomainEventBus;
use crate::backoffice::plan::application::{
    commands::{
        create_plan_use_case::CreatePlanUseCase,
        add_todo_use_case::AddTodoUseCase,
        update_todo_use_case::UpdateTodoUseCase,
        remove_todo_use_case::RemoveTodoUseCase,
        check_todo_use_case::CheckTodoUseCase,
    },
    queries::{ find_plans_use_case::FindPlansUseCase, get_plan_use_case::GetPlanUseCase },
};
use crate::backoffice::plan::adapters::controllers::{
    create_plan_controller::{ CreatePlanController, CreatePlanRequestObject },
    find_plans_controller::{ FindPlansController, FindPlansRequestObject },
    get_plan_controller::{ GetPlanController, GetPlanRequestObject },
    add_todo_controller::{ AddTodoController, AddTodoRequestObject },
    update_todo_controller::{ UpdateTodoController, UpdateTodoRequestObject },
    remove_todo_controller::{ RemoveTodoController, RemoveTodoRequestObject },
    check_todo_controller::{ CheckTodoController, CheckTodoRequestObject },
};
use crate::backoffice::plan::adapters::presenters::{
    create_plan_presenter::CreatePlanPresenter,
    find_plans_presenter::FindPlansPresenter,
    get_plan_presenter::GetPlanPresenter,
    add_todo_presenter::AddTodoPresenter,
};
use crate::backoffice::plan::infrastructure::repositories::{
    in_memory_find_plans_repository::InMemoryFindPlansRepository,
    in_memory_get_plan_repository::InMemoryGetPlanRepository,
    in_memory_plan_repository::InMemoryPlanRepository,
    in_memory_repository::InMemoryRepository,
};

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
        let find_plans_repository = InMemoryFindPlansRepository::new(
            Arc::clone(&in_memory_repository)
        );
        let get_plan_repository = InMemoryGetPlanRepository::new(Arc::clone(&in_memory_repository));
        let plan_repository = InMemoryPlanRepository::new(Arc::clone(&in_memory_repository));

        PlanAggregate {
            domain_event_bus,
            find_plans_repository,
            get_plan_repository,
            plan_repository,
            caught_plan_id: caught_plan_id,
            caught_todo_id: caught_todo_id,
        }
    }

    pub async fn get_plan(&self, request_object: GetPlanRequestObject) {
        let presenter = GetPlanPresenter::new();
        let use_case = GetPlanUseCase::new(&self.get_plan_repository, &presenter);
        let controller = GetPlanController::new(&use_case);
        let _ = controller.execute(request_object).await;
    }

    pub async fn find_plans(&self, request_object: FindPlansRequestObject) {
        let presenter = FindPlansPresenter::new();
        let use_case = FindPlansUseCase::new(&self.find_plans_repository, &presenter);
        let controller = FindPlansController::new(use_case);
        let _ = controller.execute(request_object).await;
    }

    pub async fn create_plan(&self, request_object: CreatePlanRequestObject) {
        let presenter = CreatePlanPresenter::new({
            let caught_plan_id = Arc::clone(&self.caught_plan_id);
            move |id| {
                *caught_plan_id.lock().unwrap() = id;
            }
        });
        let use_case = CreatePlanUseCase::new(
            &self.plan_repository,
            self.domain_event_bus,
            &presenter
        );
        let controller = CreatePlanController::new(use_case);
        let _ = controller.execute(request_object).await;
    }

    pub async fn add_todo(&self, request_object: AddTodoRequestObject) {
        let presenter = AddTodoPresenter::new({
            let caught_todo_id = Arc::clone(&self.caught_todo_id);
            move |id| {
                *caught_todo_id.lock().unwrap() = id;
            }
        });
        let use_case = AddTodoUseCase::new(
            &self.plan_repository,
            self.domain_event_bus,
            &presenter
        );
        let controller = AddTodoController::new(use_case);
        let _ = controller.execute(request_object).await;
    }

    pub async fn update_todo(&self, request_object: UpdateTodoRequestObject) {
        let use_case = UpdateTodoUseCase::new(&self.plan_repository);
        let controller = UpdateTodoController::new(use_case);
        let _ = controller.execute(request_object).await;
    }

    pub async fn remove_todo(&self, request_object: RemoveTodoRequestObject) {
        let use_case = RemoveTodoUseCase::new(&self.plan_repository);
        let controller = RemoveTodoController::new(use_case);
        let result = controller.execute(request_object).await;
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("===");
                eprintln!("Error: {:?}", e);
                println!("(Error caused on purpose to test Plan Aggregate's consistency policies)");
                println!("===");
            }
        }
    }

    pub async fn check_todo(&self, request_object: CheckTodoRequestObject) {
        let use_case = CheckTodoUseCase::new(&self.plan_repository, self.domain_event_bus);
        let controller = CheckTodoController::new(use_case);
        let _ = controller.execute(request_object).await;
    }
}
