use async_trait::async_trait;
use chrono::{ DateTime, Local };

pub struct NotificationServiceError {}

pub struct NotificationRequest<RecipientData> {
    pub recipient_data: RecipientData,
    pub todo_id: String,
    pub todo_name: String,
    pub todo_created_at: DateTime<Local>,
}

#[async_trait]
pub trait NotificationService<RecipientData>: Sync {
    async fn send_new_todo_details(
        &self,
        request: NotificationRequest<RecipientData>
    ) -> Result<(), NotificationServiceError>;
}
