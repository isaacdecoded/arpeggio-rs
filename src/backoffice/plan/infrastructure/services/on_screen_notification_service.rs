use crate::backoffice::plan::domain::services::notification_service::{
    NotificationService, NotificationServiceError, PlanCompletedNotificationRequest,
    PlanCreatedNotificationRequest, TodoAddedNotificationRequest,
};
use async_trait::async_trait;
use chrono::{DateTime, Local};

pub struct OnScreenNotificationService;

#[async_trait]
impl NotificationService for OnScreenNotificationService {
    async fn notify_plan_created(
        &self,
        request: PlanCreatedNotificationRequest,
    ) -> Result<(), NotificationServiceError> {
        let datetime: DateTime<Local> = request.plan_created_at.into();
        println!("===");
        println!(
            "OnScreenNotificationService: [Notification] PLAN CREATED:\nContent: Plan <{}> has been created at <{}>.",
            request.plan_name,
            datetime.format("%Y-%m-%d %H:%M:%S")
        );
        println!("===");
        Ok(())
    }

    async fn notify_plan_completed(
        &self,
        request: PlanCompletedNotificationRequest,
    ) -> Result<(), NotificationServiceError> {
        let datetime: DateTime<Local> = request.plan_completed_at.into();
        println!("===");
        println!(
            "OnScreenNotificationService: [Notification] PLAN COMPLETED:\nContent: Plan <{}> has been completed at <{}>.",
            request.plan_name,
            datetime.format("%Y-%m-%d %H:%M:%S")
        );
        println!("===");
        Ok(())
    }

    async fn send_new_todo_details(
        &self,
        request: TodoAddedNotificationRequest,
    ) -> Result<(), NotificationServiceError> {
        let datetime: DateTime<Local> = request.todo_created_at.into();
        println!("===");
        println!(
            "OnScreenNotificationService: [Notification] TODO ADDED:\nContent: Todo <{}> has been added at <{}> with ID <{}>.",
            request.todo_description,
            datetime.format("%Y-%m-%d %H:%M:%S"),
            request.todo_id
        );
        println!("===");
        Ok(())
    }
}
