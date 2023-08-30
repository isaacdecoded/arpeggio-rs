use crate::core::domain::entities::entity::Entity;
use crate::core::domain::events::domain_event::DomainEvent;

pub struct AggregateRoot {
    entity: Entity,
    domain_events: Vec<DomainEvent>,
}

impl AggregateRoot {
    pub fn new(entity: Entity) -> AggregateRoot {
        AggregateRoot {
            entity,
            domain_events: Vec::new(),
        }
    }

    pub fn add_domain_event(&mut self, domain_event: DomainEvent) {
        self.domain_events.push(domain_event)
    }

    pub fn pull_domain_events(&mut self) -> Vec<DomainEvent> {
        let domain_events = self.domain_events.to_vec();
        self.domain_events = Vec::new();
        domain_events
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::entities::entity::Entity;
    use crate::core::domain::entities::{
        aggregate_root::AggregateRoot,
        value_object::ValueObject,
        string_value_object::StringValueObject,
    };
    use crate::core::domain::events::domain_event::DomainEvent;

    #[test]
    fn should_initialize_valid_instance() {
        let id = ValueObject::new("id".to_string());
        let entity = Entity::new(id, None, None);
        let ar = AggregateRoot::new(entity);

        assert_eq!(ar.entity.id(), "id".to_string());
        assert_eq!(ar.domain_events.len(), 0);
    }

    #[test]
    fn should_mutate_domain_events() {
        let id = StringValueObject::new("id".to_string());
        let entity = Entity::new(id, None, None);
        let mut ar = AggregateRoot::new(entity);

        let event_name = "domain event name";
        let domain_event = DomainEvent::new(
            StringValueObject::new(ar.entity.id()),
            StringValueObject::new(event_name.to_string()),
        );

        ar.add_domain_event(domain_event);
        assert_eq!(ar.domain_events.len(), 1);

        let ar_domain_events = ar.pull_domain_events();
        assert_eq!(ar.domain_events.len(), 0);
        assert_eq!(ar_domain_events.len(), 1);
    }
}
