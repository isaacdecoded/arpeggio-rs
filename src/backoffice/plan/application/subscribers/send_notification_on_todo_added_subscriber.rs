use crate::backoffice::plan::domain::{
    events::todo_added_domain_event::TodoAddedDomainEvent,
    services::notification_service::{NotificationService, TodoAddedNotificationRequest},
};
use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::events::domain_event_subscriber::DomainEventSubscriber;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

pub struct EmailRecipientData {
    pub address: String,
    pub name: String,
}

pub struct SendNotificationOnTodoAddedSubscriber {
    notification_service: Arc<dyn NotificationService>,
}

impl SendNotificationOnTodoAddedSubscriber {
    pub fn new(notification_service: Arc<dyn NotificationService>) -> Box<Self> {
        Box::new(Self {
            notification_service,
        })
    }
}

#[async_trait]
impl DomainEventSubscriber for SendNotificationOnTodoAddedSubscriber {
    fn subscribed_to(&self) -> String {
        TodoAddedDomainEvent::name()
    }

    async fn on(&self, domain_event: &dyn DomainEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        let todo_created_option = domain_event.as_any().downcast_ref::<TodoAddedDomainEvent>();
        if let Some(todo_created) = todo_created_option {
            let request = TodoAddedNotificationRequest {
                todo_id: todo_created.get_aggregate_root_id().to_string(),
                todo_description: todo_created.get_todo_description().to_string(),
                todo_created_at: todo_created.get_todo_added_at().to_owned(),
            };
            self.notification_service
                .send_new_todo_details(request)
                .await?;
            return Ok(());
        }
        Err(format!(
            "Invalid domain event type with name {}",
            domain_event.get_name()
        )
        .into())
    }
}
