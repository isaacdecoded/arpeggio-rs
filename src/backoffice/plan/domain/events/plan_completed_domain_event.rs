use crate::backoffice::plan::domain::entities::plan::Plan;
use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::models::entity::Entity;
use std::any::Any;
use std::time::SystemTime;

pub struct PlanCompletedDomainEvent {
    aggregate_root_id: String,
    occurring_time: SystemTime,
    plan_name: String,
}

impl PlanCompletedDomainEvent {
    pub fn new(plan: &Plan) -> Self {
        Self {
            aggregate_root_id: plan.get_id().to_string(),
            occurring_time: SystemTime::now(),
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

    fn get_occurring_time(&self) -> &SystemTime {
        &self.occurring_time
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
