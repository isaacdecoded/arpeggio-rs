use std::any::Any;
use chrono::{ Local, DateTime };
use crate::core::domain::models::entity::Entity;
use crate::core::domain::events::domain_event::DomainEvent;
use crate::backoffice::plan::domain::entities::plan::Plan;

pub struct PlanCompletedDomainEvent {
    aggregate_root_id: String,
    occurring_time: DateTime<Local>,
    plan_name: String,
}

impl PlanCompletedDomainEvent {
    pub fn new(plan: &Plan) -> Self {
        Self {
            aggregate_root_id: plan.get_id().to_string(),
            occurring_time: Local::now(),
            plan_name: plan.get_name().to_string(),
        }
    }

    pub fn name() -> String {
        "PlanCompleted".to_string()
    }

    pub fn get_plan_name(&self) -> &str {
        &self.plan_name
    }
}

impl DomainEvent for PlanCompletedDomainEvent {
    fn get_name(&self) -> String {
        PlanCompletedDomainEvent::name()
    }

    fn get_aggregate_root_id(&self) -> &String {
        &self.aggregate_root_id
    }

    fn get_occurring_time(&self) -> &DateTime<Local> {
        &self.occurring_time
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
