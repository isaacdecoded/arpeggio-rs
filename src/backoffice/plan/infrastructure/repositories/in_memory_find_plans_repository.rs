use async_trait::async_trait;
use std::sync::Arc;
use crate::{
    backoffice::plan::{
        application::queries::find_plans_use_case::PlanReadModel,
        domain::repositories::{
            criteria::find_plans_criteria::{ FindPlansCriteria, PlanFieldEnum },
            find_plans_repository::{ FindPlansRepository, FindPlansRepositoryError },
        },
    },
    core::domain::repositories::criteria::Criteria,
};
use super::in_memory_repository::InMemoryRepository;

pub struct InMemoryFindPlansRepository {
    in_memory_repository: Arc<InMemoryRepository>,
}

impl InMemoryFindPlansRepository {
    pub fn new(in_memory_repository: Arc<InMemoryRepository>) -> Self {
        Self { in_memory_repository }
    }
}

#[async_trait]
impl FindPlansRepository<PlanReadModel> for InMemoryFindPlansRepository {
    async fn find(
        &self,
        criteria: &FindPlansCriteria
    ) -> Result<Vec<PlanReadModel>, FindPlansRepositoryError> {
        let plans: Vec<_> = self.in_memory_repository.read_plans
            .read()
            .map_err(|e| FindPlansRepositoryError::FindError(e.to_string()))?
            .iter()
            .filter(|(_, plan_model)| {
                criteria
                    .get_filters()
                    .iter()
                    .all(|filter| {
                        match filter.field {
                            PlanFieldEnum::Name =>
                                plan_model.name.contains(&filter.value.to_string()),
                        }
                    })
            })
            .map(|(id, plan_model)| {
                PlanReadModel {
                    id: id.to_string(),
                    name: plan_model.name.clone(),
                    todo_count: plan_model.todos.len(),
                    created_at: plan_model.created_at,
                    updated_at: plan_model.updated_at,
                }
            })
            .collect();
        Ok(plans)
    }
}
