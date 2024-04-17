use thiserror::Error;
use std::time::SystemTime;
use async_trait::async_trait;

pub struct PlanCreatedNotificationRequest {
    pub plan_id: String,
    pub plan_name: String,
    pub plan_created_at: SystemTime,
}

pub struct PlanCompletedNotificationRequest {
    pub plan_id: String,
    pub plan_name: String,
    pub plan_completed_at: SystemTime,
}

pub struct TodoAddedNotificationRequest {
    pub todo_id: String,
    pub todo_description: String,
    pub todo_created_at: SystemTime,
}

#[derive(Error, Debug)]
pub enum NotificationServiceError {
    #[error("Unable to notify Plan creation: {0}")] NotifyPlanCreatedError(String),
    #[error("Unable to notify Plan completion: {0}")] NotifyPlanCompletedError(String),
    #[error("Unable to send Todo details: {0}")] SendTodoDetailsError(String),
}

#[async_trait]
pub trait NotificationService: Sync + Send {
    async fn notify_plan_created(
        &self,
        request: PlanCreatedNotificationRequest
    ) -> Result<(), NotificationServiceError>;
    async fn notify_plan_completed(
        &self,
        request: PlanCompletedNotificationRequest
    ) -> Result<(), NotificationServiceError>;
    async fn send_new_todo_details(
        &self,
        request: TodoAddedNotificationRequest
    ) -> Result<(), NotificationServiceError>;
}
