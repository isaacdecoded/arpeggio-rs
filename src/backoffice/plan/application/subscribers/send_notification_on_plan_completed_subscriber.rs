use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use crate::backoffice::plan::domain::events::plan_completed_domain_event::PlanCompletedDomainEvent;
use crate::backoffice::plan::domain::services::notification_service::{
    NotificationService,
    PlanCompletedNotificationRequest,
};
use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::events::domain_event_subscriber::DomainEventSubscriber;

pub struct SendNotificationOnPlanCompletedSubscriber {
    notification_service: Arc<dyn NotificationService>,
}

impl SendNotificationOnPlanCompletedSubscriber {
    pub fn new(notification_service: Arc<dyn NotificationService>) -> Box<Self> {
        Box::new(Self { notification_service })
    }
}

#[async_trait]
impl DomainEventSubscriber for SendNotificationOnPlanCompletedSubscriber {
    fn subscribed_to(&self) -> String {
        PlanCompletedDomainEvent::name()
    }

    async fn on(&self, domain_event: &dyn DomainEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        let plan_completed_option = domain_event
            .as_any()
            .downcast_ref::<PlanCompletedDomainEvent>();
        if let Some(plan_completed) = plan_completed_option {
            let request = PlanCompletedNotificationRequest {
                plan_id: plan_completed.get_aggregate_root_id().to_string(),
                plan_name: plan_completed.get_plan_name().to_string(),
                plan_completed_at: plan_completed.get_occurring_time().to_owned(),
            };
            self.notification_service.notify_plan_completed(request).await?;
            return Ok(());
        }
        Err(format!("Invalid domain event type with name {}", domain_event.get_name()).into())
    }
}
