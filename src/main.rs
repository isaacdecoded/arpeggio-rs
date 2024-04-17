pub mod core;
pub mod backoffice;

use core::domain::events::domain_event_bus::DomainEventBus;

use core::domain::events::domain_event_subscriber::DomainEventSubscriber;
use std::error::Error;
use std::sync::Arc;

use backoffice::plan::domain::services::notification_service::NotificationService;

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
        application::subscribers::{
            send_notification_on_plan_completed_subscriber::SendNotificationOnPlanCompletedSubscriber,
            send_notification_on_plan_created_subscriber::SendNotificationOnPlanCreatedSubscriber,
            send_notification_on_todo_added_subscriber::SendNotificationOnTodoAddedSubscriber,
        },
        infrastructure::services::on_screen_notification_service::OnScreenNotificationService,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Prepare the Domain Event Bus
    let mut domain_event_bus = InMemoryDomainEventBus::default();
    let notification_service: Arc<dyn NotificationService> = Arc::new(OnScreenNotificationService);
    let send_notification_on_plan_created_subscriber: Box<
        dyn DomainEventSubscriber
    > = SendNotificationOnPlanCreatedSubscriber::new(Arc::clone(&notification_service));
    let send_notification_on_plan_completed_subscriber: Box<
        dyn DomainEventSubscriber
    > = SendNotificationOnPlanCompletedSubscriber::new(Arc::clone(&notification_service));
    let send_notification_on_todo_added_subscriber: Box<
        dyn DomainEventSubscriber
    > = SendNotificationOnTodoAddedSubscriber::new(Arc::clone(&notification_service));
    domain_event_bus.add_subscribers(
        vec![
            send_notification_on_plan_created_subscriber,
            send_notification_on_plan_completed_subscriber,
            send_notification_on_todo_added_subscriber
        ]
    ).await?;

    // Initialize and interact with the Backoffice Bounded Context
    let backoffice_context = BackofficeContext::new(&domain_event_bus);
    backoffice_context.plan_aggregate.create_plan(CreatePlanRequestObject {
        name: "My First Plan".to_string(),
    }).await;

    backoffice_context.plan_aggregate.find_plans(FindPlansRequestObject {
        limit: 10,
        offset: 0,
        name: None,
    }).await;

    let plan_id = backoffice_context.plan_aggregate.caught_plan_id.lock().unwrap().to_string();
    backoffice_context.plan_aggregate.add_todo(AddTodoRequestObject {
        plan_id: plan_id.clone(),
        description: "My First Todo".to_string(),
    }).await;

    let todo_id = backoffice_context.plan_aggregate.caught_todo_id.lock().unwrap().to_string();
    backoffice_context.plan_aggregate.update_todo(UpdateTodoRequestObject {
        plan_id: plan_id.clone(),
        todo_id: todo_id.clone(),
        description: "My First Todo (Updated)".to_string(),
    }).await;

    let plan_id = backoffice_context.plan_aggregate.caught_plan_id.lock().unwrap().to_string();
    let todo_id = backoffice_context.plan_aggregate.caught_todo_id.lock().unwrap().to_string();

    backoffice_context.plan_aggregate.check_todo(CheckTodoRequestObject {
        plan_id: plan_id.clone(),
        todo_id: todo_id.clone(),
    }).await;

    backoffice_context.plan_aggregate.get_plan(GetPlanRequestObject {
        id: plan_id.clone(),
    }).await;

    backoffice_context.plan_aggregate.remove_todo(RemoveTodoRequestObject {
        plan_id: plan_id.clone(),
        todo_id: todo_id.clone(),
    }).await;
    Ok(())
}
