pub mod core;
pub mod backoffice;

use core::domain::events::domain_event_bus::DomainEventBus;

use std::error::Error;

use crate::core::infrastructure::in_memory_domain_event_bus::InMemoryDomainEventBus;
use crate::backoffice::{
    backoffice_context::BackofficeContext,
    plan::{
        adapters::controllers::{
            create_plan_controller::CreatePlanRequestObject,
            get_plan_controller::GetPlanRequestObject,
            find_plans_controller::FindPlansRequestObject,
            add_todo_controller::AddTodoRequestObject,
            update_todo_controller::UpdateTodoRequestObject,
            check_todo_controller::CheckTodoRequestObject,
            remove_todo_controller::RemoveTodoRequestObject,
        },
        application::subscribers::send_notification_on_todo_added_subscriber::SendNotificationOnTodoAddedSubscriber,
        infrastructure::services::on_screen_notification_service::OnScreenNotificationService,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut domain_event_bus = InMemoryDomainEventBus::new();
    let on_screen_notification_service = OnScreenNotificationService::new();
    let send_notification_on_todo_added_subscriber = SendNotificationOnTodoAddedSubscriber::new(
        Box::new(on_screen_notification_service)
    );
    domain_event_bus.add_subscriber(Box::new(send_notification_on_todo_added_subscriber)).await?;

    let backoffice_context = BackofficeContext::new(&domain_event_bus);
    backoffice_context.plan_aggregate.create_plan(CreatePlanRequestObject {
        name: "My First Plan".to_string(),
    }).await;

    backoffice_context.plan_aggregate.find_plans(FindPlansRequestObject {
        limit: 10,
        offset: 0,
        name: None,
    }).await;

    backoffice_context.plan_aggregate.add_todo(AddTodoRequestObject {
        plan_id: backoffice_context.plan_aggregate.caught_plan_id.lock().unwrap().to_string(),
        description: "My First Todo".to_string(),
    }).await;

    backoffice_context.plan_aggregate.update_todo(UpdateTodoRequestObject {
        plan_id: backoffice_context.plan_aggregate.caught_plan_id.lock().unwrap().to_string(),
        todo_id: backoffice_context.plan_aggregate.caught_todo_id.lock().unwrap().to_string(),
        description: "My First Todo (Updated)".to_string(),
    }).await;

    backoffice_context.plan_aggregate.check_todo(CheckTodoRequestObject {
        plan_id: backoffice_context.plan_aggregate.caught_plan_id.lock().unwrap().to_string(),
        todo_id: backoffice_context.plan_aggregate.caught_todo_id.lock().unwrap().to_string(),
    }).await;

    backoffice_context.plan_aggregate.get_plan(GetPlanRequestObject {
        id: backoffice_context.plan_aggregate.caught_plan_id.lock().unwrap().to_string(),
    }).await;

    backoffice_context.plan_aggregate.remove_todo(RemoveTodoRequestObject {
        plan_id: backoffice_context.plan_aggregate.caught_plan_id.lock().unwrap().to_string(),
        todo_id: backoffice_context.plan_aggregate.caught_todo_id.lock().unwrap().to_string(),
    }).await;
    Ok(())
}
