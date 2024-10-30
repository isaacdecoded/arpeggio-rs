use crate::{
    backoffice::plan::domain::entities::todo::Todo,
    core::domain::{events::domain_event::DomainEvent, models::entity::Entity},
};
use std::any::Any;
use std::time::SystemTime;

#[derive(Clone)]
pub struct TodoAddedDomainEvent {
    aggregate_root_id: String,
    occurring_time: SystemTime,
    todo_description: String,
    todo_added_at: SystemTime,
}

impl TodoAddedDomainEvent {
    pub fn new(todo: &Todo) -> Box<Self> {
        Box::new(Self {
            aggregate_root_id: todo.get_id().to_string(),
            occurring_time: SystemTime::now(),
            todo_description: todo.get_description().to_string(),
            todo_added_at: todo.get_created_at().to_owned(),
        })
    }

    pub fn name() -> String {
        "TodoAdded".to_string()
    }

    pub fn get_todo_description(&self) -> &String {
        &self.todo_description
    }

    pub fn get_todo_added_at(&self) -> &SystemTime {
        &self.todo_added_at
    }
}

impl DomainEvent for TodoAddedDomainEvent {
    fn get_name(&self) -> String {
        TodoAddedDomainEvent::name()
    }

    fn get_aggregate_root_id(&self) -> &String {
        &self.aggregate_root_id
    }

    fn get_occurring_time(&self) -> &SystemTime {
        &self.occurring_time
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
