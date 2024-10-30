use async_trait::async_trait;
use std::sync::Arc;
use crate::{
    backoffice::plan::{
        application::queries::get_plan_use_case::{ GetPlanReadModel, PlanTodoReadModel },
        domain::repositories::get_plan_repository::{ GetPlanRepository, GetPlanRepositoryError },
    },
    core::domain::models::{ identity_object::IdentityObject, value_object::ValueObject },
};
use super::in_memory_repository::InMemoryRepository;

pub struct InMemoryGetPlanRepository {
    in_memory_repository: Arc<InMemoryRepository>,
}

impl InMemoryGetPlanRepository {
    pub fn new(in_memory_repository: Arc<InMemoryRepository>) -> Self {
        Self { in_memory_repository }
    }
}

#[async_trait]
impl GetPlanRepository<GetPlanReadModel> for InMemoryGetPlanRepository {
    async fn get_by_id(
        &self,
        id: &IdentityObject
    ) -> Result<Option<GetPlanReadModel>, GetPlanRepositoryError> {
        let id_value = id.get_value();
        let result = self.in_memory_repository.read_plans.read().unwrap();
        // let plan_model = self.read_plans.read().unwrap().get(id.get_value());
        match result.get(id_value) {
            Some(plan_model) => {
                let todos = plan_model.todos
                    .iter()
                    .map(|todo| {
                        PlanTodoReadModel {
                            id: todo.id.to_string(),
                            status: todo.status.to_string(),
                            description: todo.description.to_string(),
                            created_at: todo.created_at,
                            updated_at: todo.updated_at,
                        }
                    })
                    .collect();
                Ok(
                    Some(GetPlanReadModel {
                        name: plan_model.name.to_string(),
                        todos,
                        created_at: plan_model.created_at,
                        updated_at: plan_model.updated_at,
                    })
                )
            }
            None => Ok(None),
        }
    }
}
