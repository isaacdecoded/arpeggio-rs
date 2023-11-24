use chrono::{Local, DateTime};
use crate::core::domain::entities::{
        aggregate_root::AggregateRoot,
        entity::{
            Entity,
            Newable,
            Updatable,
            Recreable,
        },
        string_value_object::StringValueObject,
        value_object::ValueObject,
        date_value_object::DateValueObject};
use crate::core::domain::events::domain_event::DomainEvent;
use crate::backoffice::todo::domain::value_objects::todo_name::TodoName;

pub struct NewTodo {
    pub id: StringValueObject,
    pub name: TodoName,
}

pub struct UpdateTodo {
    pub name: TodoName,
}

pub struct RecreateTodo {
    pub id: StringValueObject,
    pub name: TodoName,
    pub created_at: DateValueObject,
    pub updated_at: Option<DateValueObject>,
}

pub struct Todo {
    id: StringValueObject,
    name: TodoName,
    created_at: DateValueObject,
    updated_at: Option<DateValueObject>,
    domain_events: Vec<DomainEvent>
}

impl Todo {
    pub fn id(&self) -> String {
        self.id.value()
    }

    pub fn name(&self) -> String {
        self.name.value()
    }

    pub fn created_at(&self) -> DateTime<Local> {
        self.created_at.value()
    }

    pub fn updated_at(&self) -> Option<DateTime<Local>> {
        match self.updated_at {
            Some(updated_at) => Some(updated_at.value()),
            None => None
        }
    }
}

impl Entity for Todo {
    type Id = StringValueObject;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn created_at(&self) -> &DateValueObject {
        &self.created_at
    }

    fn updated_at(&self) -> &Option<DateValueObject> {
        &self.updated_at
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

impl Newable<NewTodo> for Todo {
    fn new(new: NewTodo) -> Self {
        let mut todo = Self {
            id: new.id,
            name: new.name,
            created_at: DateValueObject::new(Local::now()),
            updated_at: None,
            domain_events: Vec::new(),
        };
        todo.add_domain_event(DomainEvent::new(
            todo.id.to_owned(),
            StringValueObject::new("event_name".to_string())),
        );
        todo
    }
}

impl Updatable<UpdateTodo> for Todo {
    fn update(&mut self, todo: UpdateTodo) {
        self.name = todo.name;
        self.updated_at = Some(DateValueObject::new(Local::now()));
    }
}

impl Recreable<RecreateTodo> for Todo {
    fn recreate(todo: RecreateTodo) -> Self {
        Self {
            id: todo.id,
            name: todo.name,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
            domain_events: Vec::new(),
        }
    }
}
