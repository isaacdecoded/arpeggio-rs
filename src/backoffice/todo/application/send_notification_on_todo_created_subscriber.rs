use async_trait::async_trait;
use crate::core::domain::events::domain_event_subscriber::DomainEventSubscriber;
use crate::core::domain::events::domain_event::{ DomainEvent, DEvent };
use crate::backoffice::todo::domain::events::todo_created_domain_event::TodoCreatedDomainEvent;

pub struct SendNotificationOnTodoCreatedSubscriber {}

impl SendNotificationOnTodoCreatedSubscriber {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<'a> DomainEventSubscriber for SendNotificationOnTodoCreatedSubscriber {
    fn subscribed_to(&self) -> String {
        TodoCreatedDomainEvent::name()
    }

    async fn on(&self, domain_event: &DomainEvent) {
        println!("QUELOQUEEEE {}", domain_event.get_name())
    }
}
