use async_trait::async_trait;
use crate::core::domain::events::{
    domain_event::DomainEvent,
    domain_event_bus::DomainEventBus,
    domain_event_subscriber::DomainEventSubscriber,
};

pub struct InMemoryEventBus {}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl DomainEventBus for InMemoryEventBus {
    async fn publish(&self, domain_events: Vec<DomainEvent>) {}

    async fn add_subscriber(&self, subscriber: Box<dyn DomainEventSubscriber + Send>) {}

    async fn start(&self) {}
}

/*
class InMemoryEventBus(DomainEventBus):
    def __init__(self):
        self.subscribers: OrderedDict[str, list[DomainEventSubscriber]] = OrderedDict(
            []
        )

    def publish(self, domain_events: list[DomainEvent]):
        for domain_event in domain_events:
            if domain_event.event_name.value in self.subscribers:
                subscribers = self.subscribers[domain_event.event_name.value]
                for subscriber in subscribers:
                    subscriber.on(domain_event)

    def add_subscribers(self, subscribers: list[DomainEventSubscriber]):
        for subscriber in subscribers:
            for subscriber_domain_event_name in subscriber.subscribed_to():
                if subscriber_domain_event_name not in self.subscribers:
                    self.subscribers[subscriber_domain_event_name] = []

                self.subscribers[subscriber_domain_event_name].append(subscriber)

    def start(self):
        pass
*/