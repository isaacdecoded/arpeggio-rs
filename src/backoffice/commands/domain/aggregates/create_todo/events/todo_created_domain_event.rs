use chrono::{ Local, DateTime };
use crate::core::domain::events::domain_event::{ DomainEvent, DEvent };

#[derive(Clone)]
pub struct TodoCreatedDomainEvent {
    aggregate_root_id: String,
    occurring_time: DateTime<Local>,
    // todo_name: String,
    // todo_created_at: String,
}

impl TodoCreatedDomainEvent {
    pub fn new(aggregate_root_id: String) -> DomainEvent {
        DomainEvent::new(TodoCreatedDomainEvent::name(), aggregate_root_id)
    }

    pub fn name() -> String {
        "TodoCreated".to_string()
    }
}

impl DEvent for TodoCreatedDomainEvent {
    fn get_name(&self) -> String {
        "TodoCreated".to_string()
    }

    fn get_aggregate_root_id(&self) -> String {
        self.aggregate_root_id.to_string()
    }

    fn get_occurring_time(&self) -> DateTime<Local> {
        self.occurring_time
    }
}
