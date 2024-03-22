use async_trait::async_trait;
use std::error::Error;
use crate::core::domain::events::{
    domain_event::DomainEvent,
    domain_event_bus::DomainEventBus,
    domain_event_subscriber::DomainEventSubscriber,
};
use std::collections::HashMap;

pub struct InMemoryDomainEventBus {
    subscribers: HashMap<String, Vec<Box<dyn DomainEventSubscriber>>>,
}

impl InMemoryDomainEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }
}

#[async_trait]
impl DomainEventBus for InMemoryDomainEventBus {
    async fn publish(
        &self,
        domain_events: Vec<Box<dyn DomainEvent>>
    ) -> Result<(), Box<dyn Error>> {
        for domain_event in domain_events.iter() {
            if let Some(subscribers) = self.subscribers.get(&domain_event.get_name()) {
                for subscriber in subscribers.iter() {
                    subscriber.on(domain_event.as_ref()).await?;
                }
            }
        }
        Ok(())
    }

    async fn add_subscriber(
        &mut self,
        subscriber: Box<dyn DomainEventSubscriber>
    ) -> Result<(), Box<dyn Error>> {
        let subscriber_domain_event_name = subscriber.subscribed_to();
        if let Some(subscribers) = self.subscribers.get_mut(&subscriber_domain_event_name) {
            subscribers.push(subscriber);
        } else {
            let mut subscribers = Vec::new();
            subscribers.push(subscriber);
            self.subscribers.insert(subscriber_domain_event_name, subscribers);
        }
        Ok(())
    }
}
