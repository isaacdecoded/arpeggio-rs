use async_trait::async_trait;
use chrono::{ DateTime, Local };
use std::{ collections::HashMap, sync::RwLock };
use crate::backoffice::queries::{
    domain::repositories::find_todos_repository::{ FindTodosRepository, FindTodosRepositoryError },
    application::use_cases::find_todos_use_case::{ FindTodosReadCriteria, FindTodosReadModel },
};

struct TodoModel {
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

pub struct InMemoryFindTodosRepository {
    todos: RwLock<HashMap<String, TodoModel>>,
}

impl InMemoryFindTodosRepository {
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
impl FindTodosRepository<FindTodosReadCriteria, Vec<FindTodosReadModel>>
for InMemoryFindTodosRepository {
    async fn find(
        &self,
        criteria: FindTodosReadCriteria
    ) -> Result<Vec<FindTodosReadModel>, FindTodosRepositoryError> {
        let todo_models = self.todos.read().unwrap();
        /*
        if todo_models.len() <= offset.into() {
            return Err(TodoRepositoryError {
                msg: "Offset value is bigger than stored TodoModels".to_string(),
            });
        }
        */
        let mut todos: Vec<FindTodosReadModel> = Vec::new();
        for (idx, (key, todo_model)) in todo_models.iter().enumerate() {
            todos.push(FindTodosReadModel {
                id: key.to_string(),
                name: todo_model.name.to_string(),
                created_at: todo_model.created_at,
            });
            /*
            if idx >= offset.into() && todos.len() <= limit.into() {
                todos.push(self.to_entity(key, todo_model));
            }
             */
        }
        Ok(todos)
    }
}
