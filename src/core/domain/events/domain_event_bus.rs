use crate::core::domain::events::{
    domain_event::DomainEvent, domain_event_subscriber::DomainEventSubscriber,
};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait DomainEventBus: Send + Sync {
    async fn publish(
        &self,
        domain_events: Vec<Box<dyn DomainEvent>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn add_subscribers(
        &mut self,
        subscribers: Vec<Box<dyn DomainEventSubscriber>>,
    ) -> Result<(), Box<dyn Error>>;
}
