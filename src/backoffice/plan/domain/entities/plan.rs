use crate::backoffice::plan::domain::entities::todo::{CreateTodoProps, Todo};
use crate::backoffice::plan::domain::enums::todo_status::TodoStatus;
use crate::backoffice::plan::domain::events::{
    plan_completed_domain_event::PlanCompletedDomainEvent,
    plan_created_domain_event::PlanCreatedDomainEvent,
    todo_added_domain_event::TodoAddedDomainEvent,
};
use crate::backoffice::plan::domain::value_objects::plan_name::PlanName;
use crate::backoffice::plan::domain::value_objects::todo_description::TodoDescription;
use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::models::{
    aggregate_root::AggregateRoot, date_value_object::DateValueObject, entity::Entity,
    identity_object::IdentityObject, value_object::ValueObject,
};
use std::time::SystemTime;
use thiserror::Error;

pub struct CreatePlanProps {
    pub id: IdentityObject,
    pub name: PlanName,
    pub todos: Option<Vec<Todo>>,
}

pub struct RecreatePlanProps {
    pub id: IdentityObject,
    pub name: PlanName,
    pub todos: Option<Vec<Todo>>,
    pub created_at: DateValueObject,
    pub updated_at: Option<DateValueObject>,
}

pub struct Plan {
    id: IdentityObject,
    name: PlanName,
    todos: Vec<Todo>,
    created_at: DateValueObject,
    updated_at: Option<DateValueObject>,
    domain_events: Vec<Box<dyn DomainEvent>>,
}

#[derive(Error, Debug)]
pub enum PlanError {
    #[error("Unable to get Todo: {0}")]
    GetTodo(String),
    #[error("Unable to process Todo: {0}")]
    ValidateDescriptionError(String),
    #[error("Unable to add Todo: {0}")]
    AddTodoError(String),
    #[error("Unable to remove Todo: {0}")]
    RemoveTodoError(String),
    #[error("Unable to mark Todo as done: {0}")]
    MarkTodoAsDoneError(String),
}

impl Plan {
    pub fn get_name(&self) -> &String {
        self.name.get_value()
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn create(props: CreatePlanProps) -> Self {
        let mut plan = Self {
            id: props.id,
            name: props.name,
            todos: props.todos.unwrap_or_default(),
            created_at: DateValueObject::now(),
            updated_at: None,
            domain_events: Vec::new(),
        };
        plan.add_domain_event(Box::new(PlanCreatedDomainEvent::new(&plan)));
        plan
    }

    pub fn recreate(props: RecreatePlanProps) -> Self {
        Self {
            id: props.id,
            name: props.name,
            todos: props.todos.unwrap_or_default(),
            created_at: props.created_at,
            updated_at: props.updated_at,
            domain_events: Vec::new(),
        }
    }

    pub fn change_name(&mut self, name: PlanName) {
        self.name = name;
        self.update()
    }

    pub fn add_todo(
        &mut self,
        id: &IdentityObject,
        description: &TodoDescription,
    ) -> Result<(), PlanError> {
        self.validate_description_duplication(description)?;
        let todo = Todo::new(CreateTodoProps {
            id: id.to_owned(),
            description: description.to_owned(),
            status: TodoStatus::PENDING,
            created_at: DateValueObject::now(),
            updated_at: None,
        });
        self.add_domain_event(TodoAddedDomainEvent::new(&todo));
        self.todos.push(todo);
        self.update();
        Ok(())
    }

    pub fn remove_todo(&mut self, id: &IdentityObject) -> Result<(), PlanError> {
        if self.is_completed() {
            return Err(PlanError::RemoveTodoError(
                "This Plan aggregation's lifecycle is completed".into(),
            ));
        }
        let todo = self.get_todo(id)?;
        todo.change_status(TodoStatus::REMOVED);
        self.update();
        Ok(())
    }

    pub fn change_todo_description(
        &mut self,
        id: &IdentityObject,
        description: &TodoDescription,
    ) -> Result<(), PlanError> {
        self.validate_description_duplication(description)?;
        let todo = self.get_todo(id)?;
        todo.change_description(description);
        self.update();
        Ok(())
    }

    pub fn mark_todo_as_done(&mut self, id: &IdentityObject) -> Result<(), PlanError> {
        let todo = self.get_todo(id)?;
        todo.change_status(TodoStatus::DONE);
        self.update();
        self.check_completeness();
        Ok(())
    }

    pub fn is_completed(&self) -> bool {
        self.todos
            .iter()
            .all(|t| t.get_status() == &TodoStatus::DONE)
    }

    fn check_completeness(&mut self) {
        if self.is_completed() {
            self.add_domain_event(Box::new(PlanCompletedDomainEvent::new(self)));
        }
    }

    fn validate_description_duplication(
        &self,
        description: &TodoDescription,
    ) -> Result<(), PlanError> {
        let description_already_exist = self
            .todos
            .iter()
            .any(|t| t.get_description() == description.get_value());
        if description_already_exist {
            return Err(PlanError::ValidateDescriptionError(format!(
                "Todo with the same description already exists: {}",
                description.get_value()
            )));
        }
        Ok(())
    }

    fn get_todo(&mut self, id: &IdentityObject) -> Result<&mut Todo, PlanError> {
        let result = self.todos.iter_mut().find(|t| t.get_id().is_equal(id));
        match result {
            Some(todo) => Ok(todo),
            None => Err(PlanError::GetTodo(
                "Todo not found in current Plan aggregation".into(),
            )),
        }
    }
}

impl AggregateRoot<IdentityObject> for Plan {
    fn add_domain_event(&mut self, domain_event: Box<dyn DomainEvent>) {
        self.domain_events.push(domain_event);
    }

    fn pull_domain_events(&mut self) -> Vec<Box<dyn DomainEvent>> {
        self.domain_events.drain(..).collect()
    }
}

impl Entity<IdentityObject> for Plan {
    fn get_id(&self) -> &IdentityObject {
        &self.id
    }

    fn get_created_at(&self) -> &SystemTime {
        self.created_at.get_value()
    }

    fn get_updated_at(&self) -> Option<&SystemTime> {
        self.updated_at.as_ref().map(|v| v.get_value())
    }

    fn update(&mut self) {
        self.updated_at = Some(DateValueObject::now());
    }
}
