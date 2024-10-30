use crate::core::domain::events::domain_event::DomainEvent;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait DomainEventSubscriber: Send + Sync {
    fn subscribed_to(&self) -> String;
    async fn on(&self, domain_event: &dyn DomainEvent) -> Result<(), Box<dyn Error + Send + Sync>>;
}
