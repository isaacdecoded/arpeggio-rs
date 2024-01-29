use async_trait::async_trait;
use crate::core::domain::events::{
    domain_event::DomainEvent,
    domain_event_bus::DomainEventBus,
    domain_event_subscriber::DomainEventSubscriber,
};
use std::collections::HashMap;

pub struct InMemoryDomainEventBus<'a> {
    subscribers: HashMap<String, Vec<&'a dyn DomainEventSubscriber>>,
}

impl<'a> InMemoryDomainEventBus<'a> {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }
}

#[async_trait]
impl<'a> DomainEventBus<'a> for InMemoryDomainEventBus<'a> {
    async fn publish(&self, domain_events: Vec<DomainEvent>) {
        for domain_event in domain_events.iter() {
            if let Some(subscribers) = self.subscribers.get(&domain_event.get_name()) {
                for subscriber in subscribers.iter() {
                    subscriber.on(domain_event).await;
                }
            }
        }
    }

    async fn add_subscriber(&mut self, subscriber: &'a dyn DomainEventSubscriber) {
        let subscriber_domain_event_name = subscriber.subscribed_to();
        if let Some(subscribers) = self.subscribers.get_mut(&subscriber_domain_event_name) {
            subscribers.push(subscriber);
        } else {
            let mut subscribers = Vec::new();
            subscribers.push(subscriber);
            self.subscribers.insert(subscriber_domain_event_name, subscribers);
        }
    }
}
