use async_trait::async_trait;
use chrono::{ DateTime, Local };
use std::{ collections::HashMap, sync::RwLock };
use crate::{
    core::domain::entities::{ identity_object::IdentityObject, value_object::ValueObject },
    backoffice::queries::domain::repositories::get_todo_repository::{
        GetTodoRepository,
        GetTodoRepositoryError,
    },
    backoffice::queries::application::use_cases::get_todo_use_case::GetTodoReadModel,
};

struct TodoModel {
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

pub struct InMemoryGetTodoRepository {
    todos: RwLock<HashMap<String, TodoModel>>,
}

impl InMemoryGetTodoRepository {
    pub fn new() -> Self {
        let todos = RwLock::new(HashMap::new());
        todos.write().unwrap().insert("MyFirstTodoID".to_string(), TodoModel {
            name: "My First Todo".to_string(),
            created_at: Local::now(),
            updated_at: None,
        });
        Self { todos }
    }
}

#[async_trait]
impl GetTodoRepository<GetTodoReadModel> for InMemoryGetTodoRepository {
    async fn get_by_id(
        &self,
        id: IdentityObject
    ) -> Result<Option<GetTodoReadModel>, GetTodoRepositoryError> {
        let id_value = id.get_value();
        let result = self.todos.read().unwrap();
        match result.get(id_value) {
            Some(todo_model) =>
                Ok(
                    Some(GetTodoReadModel {
                        name: todo_model.name.to_string(),
                        created_at: todo_model.created_at,
                        updated_at: todo_model.updated_at,
                    })
                ),
            None => Ok(None),
        }
    }
}
