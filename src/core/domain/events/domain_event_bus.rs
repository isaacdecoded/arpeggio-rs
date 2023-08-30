use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::events::domain_event_subscriber::DomainEventSubscriber;

pub trait DomainEventBus{
    fn new() -> Self;
    fn publish(&mut self, domain_events: Vec<DomainEvent>);
    fn add_subscriber(&mut self, subscriber: Box<dyn DomainEventSubscriber>);
    fn start(&self);
}
