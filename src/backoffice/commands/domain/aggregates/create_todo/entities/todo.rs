use crate::backoffice::commands::domain::aggregates::create_todo::events::todo_created_domain_event::TodoCreatedDomainEvent;
use crate::backoffice::commands::domain::aggregates::create_todo::value_objects::todo_name::TodoName;
use crate::core::domain::entities::{
    aggregate_root::AggregateRoot,
    date_value_object::DateValueObject,
    entity::Entity,
    identity_object::IdentityObject,
    value_object::ValueObject,
};
use crate::core::domain::events::domain_event::DomainEvent;
use chrono::{ DateTime, Local };

pub struct CreateTodoProps {
    pub id: IdentityObject,
    pub name: TodoName,
}

pub struct Todo {
    id: IdentityObject,
    name: TodoName,
    created_at: DateValueObject,
    domain_events: Vec<DomainEvent>,
}

impl Todo {
    pub fn create(props: CreateTodoProps) -> Self {
        let mut todo = Self {
            id: props.id,
            name: props.name,
            created_at: DateValueObject::new(Local::now()),
            domain_events: Vec::new(),
        };
        todo.add_domain_event(TodoCreatedDomainEvent::new(todo.get_id().get_value().to_string()));
        todo
    }

    pub fn get_name(&self) -> &String {
        self.name.get_value()
    }

    pub fn get_created_at(&self) -> &DateTime<Local> {
        self.created_at.get_value()
    }
}

impl AggregateRoot for Todo {
    fn add_domain_event(&mut self, domain_event: DomainEvent) {
        self.domain_events.push(domain_event);
    }

    fn pull_domain_events(&mut self) -> Vec<DomainEvent> {
        let domain_events = self.domain_events.to_vec();
        self.domain_events = Vec::new();
        domain_events
    }
}

impl Entity for Todo {
    fn get_id(&self) -> &IdentityObject {
        &self.id
    }
}
