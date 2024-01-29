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
    backoffice::commands::domain::aggregates::update_todo::{
        entities::todo::{ Todo, RecreateTodoProps },
        value_objects::todo_name::TodoName,
    },
    backoffice::commands::domain::aggregates::update_todo::repositories::update_todo_repository::{
        UpdateTodoRepository,
        UpdateTodoRepositoryError,
    },
};

struct TodoModel {
    pub name: String,
    pub updated_at: Option<DateTime<Local>>,
}

pub struct InMemoryUpdateTodoRepository {
    todos: RwLock<HashMap<String, TodoModel>>,
}

impl InMemoryUpdateTodoRepository {
    pub fn new() -> Self {
        let todos = RwLock::new(HashMap::new());
        todos
            .write()
            .unwrap()
            .insert("MyFirstTodoID".to_string(), TodoModel {
                name: "My First Todo".to_string(),
                updated_at: Some(Local::now()),
            });
        Self { todos }
    }

    fn to_entity(&self, id: &String, model: &TodoModel) -> Todo {
        Todo::recreate(RecreateTodoProps {
            id: IdentityObject::new(id.to_string()),
            name: TodoName::new(model.name.to_string()),
            updated_at: match model.updated_at {
                Some(updated_at) => Some(DateValueObject::new(updated_at)),
                None => None,
            },
        })
    }
}

#[async_trait]
impl UpdateTodoRepository for InMemoryUpdateTodoRepository {
    async fn get_by_id(
        &self,
        id: &IdentityObject
    ) -> Result<Option<Todo>, UpdateTodoRepositoryError> {
        let id_value = id.get_value();
        let result = self.todos.read().unwrap();
        match result.get(id_value) {
            Some(todo_model) => Ok(Some(self.to_entity(&id_value, todo_model))),
            None => Ok(None),
        }
    }

    async fn save(&self, todo: &Todo) -> Result<(), UpdateTodoRepositoryError> {
        let mut todo_updated_at = None;
        match todo.get_updated_at() {
            Some(updated_at) => {
                todo_updated_at = Some(updated_at.get_value());
            }
            _ => (),
        }
        self.todos.write().unwrap().insert(todo.get_id().get_value().to_string(), TodoModel {
            name: todo.get_name().get_value().to_string(),
            updated_at: todo_updated_at.copied(),
        });
        Ok(())
    }
}
