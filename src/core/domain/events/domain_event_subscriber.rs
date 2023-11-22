use async_trait::async_trait;
use crate::core::domain::events::domain_event::DomainEvent;

#[async_trait]
pub trait DomainEventSubscriber {
    async fn subscribed_to(&self) -> String;
    async fn on(&self, domain_event: &DomainEvent);
}
