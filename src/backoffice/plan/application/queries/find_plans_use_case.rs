use std::time::SystemTime;
use async_trait::async_trait;
use crate::{
    backoffice::plan::domain::repositories::{
        criteria::find_plans_criteria::{ FindPlansCriteria, PlanFieldEnum },
        find_plans_repository::FindPlansRepository,
    },
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::repositories::criteria::{ Criteria, Filter, FilterOperator, FilterValue },
    },
};

pub struct FindPlansRequestModel {
    pub name: Option<String>,
    pub offset: u16,
    pub limit: u16,
}

pub struct PlanReadModel {
    pub id: String,
    pub name: String,
    pub todo_count: usize,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

pub struct FindPlansResponseModel {
    pub plans: Vec<PlanReadModel>,
}

pub struct FindPlansUseCase<'a> {
    repository: &'a dyn FindPlansRepository<PlanReadModel>,
    output_port: &'a dyn UseCaseOutputPort<FindPlansResponseModel>,
}

impl<'a> FindPlansUseCase<'a> {
    pub fn new(
        repository: &'a dyn FindPlansRepository<PlanReadModel>,
        output_port: &'a dyn UseCaseOutputPort<FindPlansResponseModel>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<FindPlansRequestModel> for FindPlansUseCase<'a> {
    async fn interact(&self, request_model: FindPlansRequestModel) {
        let mut filters: Vec<Filter<PlanFieldEnum>> = vec![];
        if let Some(name) = request_model.name {
            filters.push(Filter {
                field: PlanFieldEnum::Name,
                operator: FilterOperator::Contains,
                value: FilterValue::Str(name),
            });
        }
        let criteria = FindPlansCriteria::new(filters)
            .with_limit(request_model.limit)
            .with_offset(request_model.offset);
        match self.repository.find(&criteria).await {
            Ok(plans) => {
                self.output_port.success(FindPlansResponseModel { plans }).await;
            }
            Err(error) => {
                self.output_port.failure(error.into()).await;
            }
        }
    }
}
