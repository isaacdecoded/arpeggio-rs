use std::fmt;
use std::error::Error;
use async_trait::async_trait;
use chrono::{ DateTime, Local };

pub struct PlanCompletedNotificationRequest {
    pub plan_id: String,
    pub plan_name: String,
    pub plan_completed_at: DateTime<Local>,
}

pub struct TodoAddedNotificationRequest {
    pub todo_id: String,
    pub todo_description: String,
    pub todo_created_at: DateTime<Local>,
}

#[derive(Debug)]
pub struct NotificationServiceError {
    pub msg: String,
}

impl fmt::Display for NotificationServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for NotificationServiceError {}

#[async_trait]
pub trait NotificationService: Sync + Send {
    async fn notify_plan_completed(
        &self,
        request: PlanCompletedNotificationRequest
    ) -> Result<(), NotificationServiceError>;
    async fn send_new_todo_details(
        &self,
        request: TodoAddedNotificationRequest
    ) -> Result<(), NotificationServiceError>;
}
