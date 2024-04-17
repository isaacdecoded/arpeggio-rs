use std::time::SystemTime;
use crate::core::domain::models::{
    entity::Entity,
    identity_object::IdentityObject,
    date_value_object::DateValueObject,
    value_object::ValueObject,
};
use crate::backoffice::plan::domain::{
    value_objects::todo_description::TodoDescription,
    enums::todo_status::TodoStatus,
};

pub struct CreateTodoProps {
    pub id: IdentityObject,
    pub description: TodoDescription,
    pub status: TodoStatus,
    pub created_at: DateValueObject,
    pub updated_at: Option<DateValueObject>,
}

pub struct Todo {
    id: IdentityObject,
    description: TodoDescription,
    status: TodoStatus,
    created_at: DateValueObject,
    updated_at: Option<DateValueObject>,
}

impl Todo {
    pub fn new(props: CreateTodoProps) -> Self {
        Self {
            id: props.id,
            description: props.description,
            status: props.status,
            created_at: DateValueObject::now(),
            updated_at: None,
        }
    }

    pub fn get_description(&self) -> &String {
        self.description.get_value()
    }

    pub fn get_status(&self) -> &TodoStatus {
        &self.status
    }

    pub fn change_description(&mut self, description: &TodoDescription) {
        self.description = description.to_owned();
        self.update()
    }

    pub fn change_status(&mut self, status: TodoStatus) {
        self.status = status;
        self.update()
    }
}

impl Entity<IdentityObject> for Todo {
    fn get_id(&self) -> &IdentityObject {
        &self.id
    }

    fn update(&mut self) {
        self.updated_at = Some(DateValueObject::now());
    }

    fn get_created_at(&self) -> &SystemTime {
        self.created_at.get_value()
    }

    fn get_updated_at(&self) -> Option<&SystemTime> {
        self.updated_at.as_ref().map(|v| v.get_value())
    }
}
