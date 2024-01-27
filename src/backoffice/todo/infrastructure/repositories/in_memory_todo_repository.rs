use crate::{
    backoffice::todo::domain::entities::todo::{ RecreateTodo, Todo },
    backoffice::todo::domain::repositories::todo_repository::{
        TodoRepository,
        TodoRepositoryError,
    },
    backoffice::todo::domain::value_objects::todo_name::TodoName,
    core::domain::entities::{
        date_value_object::DateValueObject,
        entity::Recreable,
        string_value_object::StringValueObject,
    },
    core::domain::{ entities::value_object::ValueObject, repositories::criteria::Criteria },
};
use async_trait::async_trait;
use chrono::{ DateTime, Local };
use std::{ collections::HashMap, sync::RwLock };

struct TodoModel {
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

pub struct InMemoryTodoRepository {
    todos: RwLock<HashMap<String, TodoModel>>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            todos: RwLock::new(HashMap::new()),
        }
    }

    fn to_entity(&self, id: &String, model: &TodoModel) -> Todo {
        Todo::recreate(RecreateTodo {
            id: StringValueObject::new(id.to_string()),
            name: TodoName::new(model.name.to_string()),
            created_at: DateValueObject::new(model.created_at),
            updated_at: match model.updated_at {
                Some(updated_at) => Some(DateValueObject::new(updated_at)),
                None => None,
            },
        })
    }
}

#[async_trait]
impl TodoRepository for InMemoryTodoRepository {
    async fn find(&self, criteria: Option<Criteria>) -> Result<Vec<Todo>, TodoRepositoryError> {
        let mut limit: u16 = 10;
        let mut offset: u16 = 0;
        if let Some(criteria) = criteria {
            match criteria.limit {
                Some(l) => {
                    limit = l;
                }
                None => {
                    limit = 10;
                }
            }
            match criteria.offset {
                Some(o) => {
                    offset = o;
                }
                None => {
                    offset = 0;
                }
            }
        }
        let todo_models = self.todos.read().unwrap();
        if todo_models.len() <= offset.into() {
            return Err(TodoRepositoryError {
                msg: "Offset value is bigger than stored TodoModels".to_string(),
            });
        }
        let mut todos: Vec<Todo> = Vec::new();
        for (idx, (key, todo_model)) in todo_models.iter().enumerate() {
            if idx >= offset.into() && todos.len() <= limit.into() {
                todos.push(self.to_entity(key, todo_model));
            }
        }
        Ok(todos)
    }

    async fn get_by_id(&self, id: &StringValueObject) -> Result<Option<Todo>, TodoRepositoryError> {
        let id_value = id.value();
        let result = self.todos.read().unwrap();
        match result.get(&id_value) {
            Some(todo_model) => Ok(Some(self.to_entity(&id_value, todo_model))),
            None => Ok(None),
        }
    }

    async fn save(&self, todo: &Todo) -> Result<(), TodoRepositoryError> {
        self.todos.write().unwrap().insert(todo.id(), TodoModel {
            name: todo.name(),
            created_at: todo.created_at(),
            updated_at: todo.updated_at(),
        });
        Ok(())
    }

    async fn delete(&self, criteria: Criteria) -> Result<u32, TodoRepositoryError> {
        let mut total_deleted = 0;
        for filter in criteria.filters {
            match filter.field.as_str() {
                "id" => unsafe {
                    let key = filter.value.s.as_str();
                    let removed_key = self.todos.write().unwrap().remove(key);
                    if removed_key.is_some() {
                        total_deleted += 1;
                    }
                }
                invalid_field => {
                    return Err(TodoRepositoryError {
                        msg: format!("Invalid Filter field {invalid_field}"),
                    });
                }
            }
        }
        Ok(total_deleted)
    }

    async fn generate_id(&self) -> Result<String, TodoRepositoryError> {
        Ok("MyFirstTodoID".to_string())
    }
}
