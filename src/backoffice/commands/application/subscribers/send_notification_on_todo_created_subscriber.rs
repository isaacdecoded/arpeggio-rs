use async_trait::async_trait;
use chrono::Local;
use crate::core::domain::events::domain_event_subscriber::DomainEventSubscriber;
use crate::core::domain::events::domain_event::DomainEvent;
use crate::backoffice::commands::domain::aggregates::create_todo::{
    services::notification_service::{ NotificationService, NotificationRequest },
    events::todo_created_domain_event::TodoCreatedDomainEvent,
};

pub struct EmailRecipientData {
    pub address: String,
    pub name: String,
}

pub struct SendNotificationOnTodoCreatedSubscriber<'a> {
    notification_service: &'a dyn NotificationService<EmailRecipientData>,
}

impl<'a> SendNotificationOnTodoCreatedSubscriber<'a> {
    pub fn new(notification_service: &'a dyn NotificationService<EmailRecipientData>) -> Self {
        Self {
            notification_service,
        }
    }
}

#[async_trait]
impl<'a> DomainEventSubscriber for SendNotificationOnTodoCreatedSubscriber<'a> {
    fn subscribed_to(&self) -> String {
        TodoCreatedDomainEvent::name()
    }

    async fn on(&self, domain_event: &DomainEvent) {
        let id = domain_event.get_aggregate_root_id();
        let request = NotificationRequest {
            recipient_data: EmailRecipientData {
                address: "arpeggio@arpeggio".to_string(),
                name: "Arpeggio".to_string(),
            },
            todo_id: id,
            todo_name: "".to_string(),
            todo_created_at: Local::now(),
        };
        let _ = self.notification_service.send_new_todo_details(request).await;
    }
}
