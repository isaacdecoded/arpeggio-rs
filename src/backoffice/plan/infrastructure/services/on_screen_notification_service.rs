use async_trait::async_trait;
use crate::backoffice::plan::domain::services::notification_service::{
    NotificationService,
    PlanCreatedNotificationRequest,
    PlanCompletedNotificationRequest,
    TodoAddedNotificationRequest,
    NotificationServiceError,
};

pub struct OnScreenNotificationService;

#[async_trait]
impl NotificationService for OnScreenNotificationService {
    async fn notify_plan_created(
        &self,
        request: PlanCreatedNotificationRequest
    ) -> Result<(), NotificationServiceError> {
        println!("===");
        println!(
            "OnScreenNotificationService: [Notification] PLAN CREATED:\nContent: Plan <{}> has been created at <{:?}>.",
            request.plan_name,
            request.plan_created_at
        );
        println!("===");
        Ok(())
    }

    async fn notify_plan_completed(
        &self,
        request: PlanCompletedNotificationRequest
    ) -> Result<(), NotificationServiceError> {
        println!("===");
        println!(
            "OnScreenNotificationService: [Notification] PLAN COMPLETED:\nContent: Plan <{}> has been completed at <{:?}>.",
            request.plan_name,
            request.plan_completed_at
        );
        println!("===");
        Ok(())
    }

    async fn send_new_todo_details(
        &self,
        request: TodoAddedNotificationRequest
    ) -> Result<(), NotificationServiceError> {
        println!("===");
        println!(
            "OnScreenNotificationService: [Notification] TODO ADDED:\nContent: Todo <{}> has been added at <{:?}> with ID <{}>.",
            request.todo_description,
            request.todo_created_at,
            request.todo_id
        );
        println!("===");
        Ok(())
    }
}
