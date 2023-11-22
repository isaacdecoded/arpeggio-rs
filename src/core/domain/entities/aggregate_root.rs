use crate::core::domain::entities::entity::Entity;
use crate::core::domain::events::domain_event::DomainEvent;

pub trait AggregateRoot: Entity {
    fn add_domain_event(&mut self, domain_event: DomainEvent);
    fn pull_domain_events(&mut self) -> Vec<DomainEvent>;
}
