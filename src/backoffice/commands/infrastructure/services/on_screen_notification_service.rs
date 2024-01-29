/*
import {
  NotificationService,
  NotificationRequest,
} from "../../domain/aggregates/create-todo/services"
import { EmailRecipientData } from "../../application/subscribers/SendNotificationOnTodoCreatedSubscriber"
*/
use async_trait::async_trait;
use crate::backoffice::commands::{
    domain::aggregates::create_todo::services::notification_service::{
        NotificationService,
        NotificationRequest,
        NotificationServiceError,
    },
    application::subscribers::send_notification_on_todo_created_subscriber::EmailRecipientData,
};

pub struct OnScreenNotificationService {}

impl OnScreenNotificationService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl NotificationService<EmailRecipientData> for OnScreenNotificationService {
    async fn send_new_todo_details(
        &self,
        request: NotificationRequest<EmailRecipientData>
    ) -> Result<(), NotificationServiceError> {
        let address = request.recipient_data.address;
        let name = request.recipient_data.name;
        let todo_id = request.todo_id;
        let todo_name = request.todo_name;
        let todo_created_at = request.todo_created_at;
        println!("----------------------");
        println!("OnScreenNotificationService");
        println!("\tRecipientAddress: {address}");
        println!("\tRecipientName: {name}");
        println!(
            "\tContent: `Todo <{todo_name}> has been created at <{todo_created_at}> with ID <{todo_id}>.`"
        );
        Ok(())
    }
}
