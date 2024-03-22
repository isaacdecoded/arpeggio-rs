use uuid::Uuid;
use std::str::FromStr;
use async_trait::async_trait;
use std::sync::Arc;
use crate::{
    backoffice::plan::domain::{
        entities::{ plan::{ Plan, RecreatePlanProps }, todo::{ CreateTodoProps, Todo } },
        enums::todo_status::TodoStatus,
        repositories::plan_repository::{ PlanRepository, PlanRepositoryError },
        value_objects::{ plan_name::PlanName, todo_description::TodoDescription },
    },
    core::domain::models::{
        date_value_object::DateValueObject,
        entity::Entity,
        identity_object::IdentityObject,
        value_object::ValueObject,
    },
};

use super::in_memory_repository::{ InMemoryRepository, PlanWriteModel, TodoWriteModel };

pub struct InMemoryPlanRepository {
    in_memory_repository: Arc<InMemoryRepository>,
}

impl InMemoryPlanRepository {
    pub fn new(in_memory_repository: Arc<InMemoryRepository>) -> Self {
        Self { in_memory_repository }
    }

    fn plan_to_model(&self, plan: &Plan) -> PlanWriteModel {
        PlanWriteModel {
            name: plan.get_name().to_string(),
            created_at: *plan.get_created_at(),
            updated_at: plan.get_updated_at().copied(),
        }
    }

    fn todo_to_model(&self, plan_id: &IdentityObject, todo: &Todo) -> TodoWriteModel {
        TodoWriteModel {
            id: todo.get_id().to_string(),
            plan_id: plan_id.get_value().to_string(),
            description: todo.get_description().to_string(),
            status: todo.get_status().to_string(),
            created_at: *todo.get_created_at(),
            updated_at: todo.get_updated_at().copied(),
        }
    }

    fn plan_model_to_entity(&self, id: &str, model: &PlanWriteModel) -> Plan {
        let todo_models: Vec<TodoWriteModel> = match
            self.in_memory_repository.write_todos.read().unwrap().get(id)
        {
            Some(todos) => todos.to_vec(),
            None => Vec::new(),
        };
        Plan::recreate(RecreatePlanProps {
            id: IdentityObject::new(id.to_string()),
            name: PlanName::new(model.name.to_string()),
            todos: Some(
                todo_models
                    .iter()
                    .map(|todo| {
                        Todo::new(CreateTodoProps {
                            id: IdentityObject::new(todo.id.to_string()),
                            description: TodoDescription::new(todo.description.to_string()),
                            status: TodoStatus::from_str(&todo.status).unwrap(),
                            created_at: DateValueObject::new(todo.created_at),
                            updated_at: match todo.updated_at {
                                Some(updated_at) => Some(DateValueObject::new(updated_at)),
                                None => None,
                            },
                        })
                    })
                    .collect()
            ),
            created_at: DateValueObject::new(model.created_at),
            updated_at: match model.updated_at {
                Some(updated_at) => Some(DateValueObject::new(updated_at)),
                None => None,
            },
        })
    }
}

#[async_trait]
impl PlanRepository for InMemoryPlanRepository {
    async fn get_by_id(&self, id: &IdentityObject) -> Result<Option<Plan>, PlanRepositoryError> {
        let id_value = id.get_value();
        let result = self.in_memory_repository.write_plans.read().unwrap();
        match result.get(id_value) {
            Some(plan_model) => { Ok(Some(self.plan_model_to_entity(id_value, plan_model))) }
            None => Ok(None),
        }
    }

    async fn save(&self, plan: &Plan) -> Result<(), PlanRepositoryError> {
        self.in_memory_repository.write_plans
            .write()
            .unwrap()
            .insert(plan.get_id().to_string(), self.plan_to_model(plan));

        let todo_models: Vec<TodoWriteModel> = plan
            .get_todos()
            .iter()
            .map(|todo| self.todo_to_model(plan.get_id(), todo))
            .collect();

        self.in_memory_repository.write_todos
            .write()
            .unwrap()
            .insert(plan.get_id().to_string(), todo_models);
        self.in_memory_repository.sync_read_plans(plan);
        Ok(())
    }

    async fn generate_id(&self) -> Result<IdentityObject, PlanRepositoryError> {
        Ok(IdentityObject::new(Uuid::new_v4().to_string()))
    }
}
