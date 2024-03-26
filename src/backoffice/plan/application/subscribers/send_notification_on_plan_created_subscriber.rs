use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use crate::backoffice::plan::domain::events::plan_created_domain_event::PlanCreatedDomainEvent;
use crate::backoffice::plan::domain::services::notification_service::{
    NotificationService,
    PlanCreatedNotificationRequest,
};
use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::events::domain_event_subscriber::DomainEventSubscriber;

pub struct SendNotificationOnPlanCreatedSubscriber {
    notification_service: Arc<dyn NotificationService>,
}

impl SendNotificationOnPlanCreatedSubscriber {
    pub fn new(notification_service: Arc<dyn NotificationService>) -> Box<Self> {
        Box::new(Self { notification_service })
    }
}

#[async_trait]
impl DomainEventSubscriber for SendNotificationOnPlanCreatedSubscriber {
    fn subscribed_to(&self) -> String {
        String::from(PlanCreatedDomainEvent::name())
    }

    async fn on(&self, domain_event: &dyn DomainEvent) -> Result<(), Box<dyn Error>> {
        let plan_created_option = domain_event.as_any().downcast_ref::<PlanCreatedDomainEvent>();
        if let Some(plan_created) = plan_created_option {
            let request = PlanCreatedNotificationRequest {
                plan_id: plan_created.get_aggregate_root_id().to_string(),
                plan_name: plan_created.get_plan_name().to_string(),
                plan_created_at: plan_created.get_plan_created_at().to_owned(),
            };
            self.notification_service.notify_plan_created(request).await?;
            return Ok(());
        }
        Err(Box::from(format!("Invalid domain event type with name {}", domain_event.get_name())))
    }
}
