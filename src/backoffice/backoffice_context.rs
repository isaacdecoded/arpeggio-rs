use crate::core::domain::events::domain_event_bus::DomainEventBus;
use crate::backoffice::plan::plan_aggregate::PlanAggregate;

pub struct BackofficeContext<'a> {
    pub plan_aggregate: PlanAggregate<'a>,
}

impl<'a> BackofficeContext<'a> {
    pub fn new(domain_event_bus: &'a dyn DomainEventBus) -> Self {
        Self {
            plan_aggregate: PlanAggregate::new(domain_event_bus),
        }
    }
}
