use crate::core::domain::events::domain_event::DomainEvent;

pub trait DomainEventSubscriber {
    fn subscribed_to(&self) -> String;
    fn on(&self, domain_event: &DomainEvent) -> ();
}
