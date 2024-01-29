use crate::backoffice::commands::domain::aggregates::remove_todo::value_objects::todo_status::{
    TodoStatus,
    TodoStatuses,
};
use crate::core::domain::entities::{
    aggregate_root::AggregateRoot,
    date_value_object::DateValueObject,
    entity::Entity,
    identity_object::IdentityObject,
    value_object::ValueObject,
};
use crate::core::domain::events::domain_event::DomainEvent;
use chrono::Local;

pub struct UpdateTodo {
    pub status: TodoStatus,
}

pub struct RecreateTodoProps {
    pub id: IdentityObject,
    pub status: TodoStatus,
    pub updated_at: Option<DateValueObject>,
}

pub struct Todo {
    id: IdentityObject,
    status: TodoStatus,
    updated_at: Option<DateValueObject>,
    domain_events: Vec<DomainEvent>,
}

impl Todo {
    pub fn recreate(props: RecreateTodoProps) -> Self {
        Self {
            id: props.id,
            status: props.status,
            updated_at: props.updated_at,
            domain_events: Vec::new(),
        }
    }

    pub fn get_status(&self) -> &TodoStatus {
        &self.status
    }

    pub fn get_updated_at(&self) -> &Option<DateValueObject> {
        &self.updated_at
    }

    pub fn remove(&mut self) {
        self.status = TodoStatus::new(TodoStatuses::REMOVED);
        self.updated_at = Some(DateValueObject::new(Local::now()));
        // this._status = new TodoStatus(TodoStatuses.REMOVED)
        // this._updatedAt = new DateObject(new Date())
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

/*
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
*/
