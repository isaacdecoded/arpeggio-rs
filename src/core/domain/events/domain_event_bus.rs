use async_trait::async_trait;
use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::events::domain_event_subscriber::DomainEventSubscriber;

#[async_trait]
pub trait DomainEventBus: Send + Sync {
    async fn publish(&self, domain_events: Vec<DomainEvent>);
    async fn add_subscriber(&self, subscriber: Box<dyn DomainEventSubscriber + Send>);
    async fn start(&self);
}
