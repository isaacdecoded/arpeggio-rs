use chrono::{DateTime, Local};
use async_trait::async_trait;
use crate::{
    core::domain::entities::{
        string_value_object::StringValueObject,
        date_value_object::DateValueObject,
        entity::Entity,
    },
    core::domain::repositories::criteria::Criteria,
    backoffice::todo::domain::entities::todo::Todo,
    backoffice::todo::domain::value_objects::todo_name::TodoName,
    backoffice::todo::domain::repositories::todo_repository::{
        TodoRepository,
        TodoRepositoryError,
        TodoCriteria,
    },
};
use std::{
    collections::HashMap,
    sync::RwLock,
};

struct TodoModel {
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>
}

pub struct InMemoryTodoRepository {
    todos: RwLock<HashMap<String, TodoModel>>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self { todos: RwLock::new(HashMap::new()) }
    }
}

#[async_trait]
impl TodoRepository for InMemoryTodoRepository {
    async fn save(&self, todo: &Todo) -> Result<(), TodoRepositoryError> {
        self.todos
            .write()
            .unwrap()
            .insert(todo.id(), TodoModel {
                name: todo.name(),
                created_at: todo.created_at(),
                updated_at: todo.updated_at(),
            });
        Ok(())
    }

    async fn delete(&self, criteria: Criteria) -> Result<u32, TodoRepositoryError> {
        // Err(TodoRepositoryError{msg: "Error while generating ID".to_string()})
        let mut total_deleted = 0;
        for filter in criteria.filters {
            match filter.field.as_str() {
                "id" => {
                    unsafe {
                        let key = filter.value.s.as_str();
                        let removed_key = self.todos
                            .write()
                            .unwrap()
                            .remove(key);
                        if removed_key.is_some() {
                            total_deleted += 1;
                        }
                    }
                },
                invalid_field => return Err(TodoRepositoryError{
                    msg: format!("Invalid Filter field {invalid_field}")
                })
            }
        }
        Ok(total_deleted)
    }

    async fn generate_id(&self) -> Result<String, TodoRepositoryError> {
        Ok("MyFirstTodoID".to_string())
    }
}


/*
export class InMemoryTodoRepository implements TodoRepository {
  private todos: Map<string | number, TodoModel> = new Map()

  async generateId(): Promise<string> {
    return 'MyFirstTodoID'
  }

  async find(criteria?: Criteria<Todo>): Promise<Todo[]> {
    // TODO: implement Criteria behaviors
    const todos: Todo[] = []
    this.todos.forEach((value, key) => {
      todos.push(this.toEntity(key, {
        name: value.name,
        createdAt: value.createdAt,
        updatedAt: value.updatedAt,
      }))
    })
    return todos
  }

  async getById(id: IdentityObject): Promise<Todo|undefined> {
    const todoModel = this.todos.get(id.value)
    return todoModel
      ? this.toEntity(id.value, todoModel)
      : todoModel
  }

  async save(entity: Todo): Promise<void> {
    this.todos.set(entity.id.value, this.toModel(entity))
  }

  async delete(criteria?: Criteria<Todo>): Promise<number> {
    // TODO: implement Criteria behaviors
    const totalDeleted = this.todos.size
    this.todos.clear()
    return totalDeleted
  }

  private toEntity(id: string | number, model: TodoModel): Todo {
    return Todo.recreate({
      id: new IdentityObject(id),
      name: new TodoName(model.name),
      createdAt: new DateObject(model.createdAt),
      updatedAt: model.updatedAt ? new DateObject(model.updatedAt) : undefined,
    })
  }

  private toModel(entity: Todo): TodoModel {
    return {
      name: entity.name.value,
      createdAt: entity.createdAt.value,
      updatedAt: entity.updatedAt?.value,
    }
  }
}
*/