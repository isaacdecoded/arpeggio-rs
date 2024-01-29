use async_trait::async_trait;
use chrono::{ DateTime, Local };
use std::{ collections::HashMap, sync::RwLock };
use crate::{
    core::domain::entities::{
        date_value_object::DateValueObject,
        entity::Entity,
        identity_object::IdentityObject,
        value_object::ValueObject,
    },
    backoffice::commands::domain::aggregates::remove_todo::{
        entities::todo::{ Todo, RecreateTodoProps },
        value_objects::todo_status::{ TodoStatus, TodoStatuses },
    },
    backoffice::commands::domain::aggregates::remove_todo::repositories::remove_todo_repository::{
        RemoveTodoRepository,
        RemoveTodoRepositoryError,
    },
};

struct TodoModel {
    pub status: TodoStatuses,
    pub updated_at: Option<DateTime<Local>>,
}

pub struct InMemoryRemoveTodoRepository {
    todos: RwLock<HashMap<String, TodoModel>>,
}

impl InMemoryRemoveTodoRepository {
    pub fn new() -> Self {
        let todos = RwLock::new(HashMap::new());
        todos
            .write()
            .unwrap()
            .insert("MyFirstTodoID".to_string(), TodoModel {
                status: TodoStatuses::ARCHIVED,
                updated_at: Some(Local::now()),
            });
        Self { todos }
    }

    fn to_entity(&self, id: &String, model: &TodoModel) -> Todo {
        Todo::recreate(RecreateTodoProps {
            id: IdentityObject::new(id.to_string()),
            status: TodoStatus::new(model.status),
            updated_at: match model.updated_at {
                Some(updated_at) => Some(DateValueObject::new(updated_at)),
                None => None,
            },
        })
    }
}

#[async_trait]
impl RemoveTodoRepository for InMemoryRemoveTodoRepository {
    async fn get_by_id(
        &self,
        id: &IdentityObject
    ) -> Result<Option<Todo>, RemoveTodoRepositoryError> {
        let id_value = id.get_value();
        let result = self.todos.read().unwrap();
        match result.get(id_value) {
            Some(todo_model) => Ok(Some(self.to_entity(&id_value, todo_model))),
            None => Ok(None),
        }
    }

    async fn save(&self, todo: &Todo) -> Result<(), RemoveTodoRepositoryError> {
        let mut todo_updated_at = None;
        match todo.get_updated_at() {
            Some(updated_at) => {
                todo_updated_at = Some(updated_at.get_value());
            }
            _ => (),
        }
        self.todos.write().unwrap().insert(todo.get_id().get_value().to_string(), TodoModel {
            status: todo.get_status().get_value().to_owned(),
            updated_at: todo_updated_at.copied(),
        });
        Ok(())
    }
}
