mod core;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::borrow::BorrowMut;
    use crate::core::domain::entities::{
        entity::Entity,
        aggregate_root::AggregateRoot,
        value_object::ValueObject,
    };
    use crate::core::domain::events::{
        domain_event::DomainEvent,
        domain_event_bus::DomainEventBus,
        domain_event_subscriber::DomainEventSubscriber,
    };

    struct TestDomainEventSubscriber {
        domain_event: String
    }

    impl DomainEventSubscriber for TestDomainEventSubscriber {
        fn subscribed_to(&self) -> String {
            self.domain_event.to_string()
        }
        fn on(&self, domain_event: &DomainEvent) {
            assert_eq!(domain_event.event_name(), self.domain_event.to_string());
        }
    }

    struct InMemoryEventBus {
        pub subscribers: HashMap<String, Vec<Box<dyn DomainEventSubscriber>>>,
    }

    impl DomainEventBus for InMemoryEventBus {
        fn new() -> Self {
            InMemoryEventBus {
                subscribers: HashMap::new(),
            }
        }

        fn publish(&mut self, domain_events: Vec<DomainEvent>) {
            for domain_event in domain_events {
                if let Some(subscribers) = self.subscribers.borrow_mut().get_mut(&domain_event.event_name()) {
                    for subscriber in subscribers {
                        subscriber.on(&domain_event);
                    }
                }
            }
        }

        fn add_subscriber(&mut self, subscriber: Box<dyn DomainEventSubscriber>) {
            if let Some(subscribers) = self.subscribers.borrow_mut().get_mut(&subscriber.subscribed_to()) {
                subscribers.push(subscriber);
            } else {
                self.subscribers
                    .borrow_mut()
                    .insert(subscriber.subscribed_to().to_owned(), vec![subscriber]);
            }
        }

        fn start(&self) {}
    }


    #[test]
    fn should_work_in_memory_event_bus_implementation() {
        // Given an Aggregate Root
        let id = ValueObject::new("id".to_string());
        let entity = Entity::new(id, None, None);
        let mut ar = AggregateRoot::new(entity);

        // Given an InMemory Event Bus and a Test Subscriber
        let domain_event_name = "Domain event name";
        let domain_event_subscriber = TestDomainEventSubscriber{
            domain_event: "Domain event name".to_string(),
        };
        let mut in_memory_event_bus = InMemoryEventBus::new();
        in_memory_event_bus.add_subscriber(Box::new(domain_event_subscriber));

        // When a Domain Event occurs
        let domain_event = DomainEvent::new(
            ValueObject::new(ar.entity().id()),
            ValueObject::new(domain_event_name.to_string()),
        );
        ar.add_domain_event(domain_event);

        // Then
        in_memory_event_bus.publish(ar.pull_domain_events());
    }
}