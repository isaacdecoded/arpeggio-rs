use crate::backoffice::commands::domain::aggregates::update_todo::value_objects::todo_name::TodoName;
use crate::core::domain::entities::{
    aggregate_root::AggregateRoot,
    date_value_object::DateValueObject,
    entity::Entity,
    identity_object::IdentityObject,
    value_object::ValueObject,
};
use crate::core::domain::events::domain_event::DomainEvent;
use chrono::Local;

pub struct UpdateTodoProps {
    pub name: TodoName,
}

pub struct RecreateTodoProps {
    pub id: IdentityObject,
    pub name: TodoName,
    pub updated_at: Option<DateValueObject>,
}

pub struct Todo {
    id: IdentityObject,
    name: TodoName,
    updated_at: Option<DateValueObject>,
    domain_events: Vec<DomainEvent>,
}

impl Todo {
    pub fn recreate(props: RecreateTodoProps) -> Self {
        Self {
            id: props.id,
            name: props.name,
            updated_at: props.updated_at,
            domain_events: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &TodoName {
        &self.name
    }

    pub fn get_updated_at(&self) -> &Option<DateValueObject> {
        &self.updated_at
    }

    pub fn update(&mut self, props: UpdateTodoProps) {
        self.name = props.name;
        self.updated_at = Some(DateValueObject::new(Local::now()));
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
