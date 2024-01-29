use async_trait::async_trait;
use chrono::{ DateTime, Local };
use std::{ collections::HashMap, sync::RwLock };
use crate::{
    backoffice::commands::domain::aggregates::create_todo::entities::todo::Todo,
    backoffice::commands::domain::aggregates::create_todo::repositories::create_todo_repository::{
        CreateTodoRepository,
        CreateTodoRepositoryError,
    },
    core::domain::entities::{ entity::Entity, value_object::ValueObject },
};

struct TodoModel {
    pub name: String,
    pub created_at: DateTime<Local>,
}

pub struct InMemoryCreateTodoRepository {
    todos: RwLock<HashMap<String, TodoModel>>,
}

impl InMemoryCreateTodoRepository {
    pub fn new() -> Self {
        Self {
            todos: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl CreateTodoRepository for InMemoryCreateTodoRepository {
    async fn save(&self, todo: &Todo) -> Result<(), CreateTodoRepositoryError> {
        self.todos.write().unwrap().insert(todo.get_id().get_value().to_string(), TodoModel {
            name: todo.get_name().to_string(),
            created_at: todo.get_created_at().to_owned(),
        });
        Ok(())
    }

    async fn generate_id(&self) -> Result<String, CreateTodoRepositoryError> {
        Ok("MyFirstTodoID".to_string())
    }
}
