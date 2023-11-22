use async_trait::async_trait;

#[async_trait]
pub trait Controller<RequestModel> {
    async fn execute(&self, request_model: RequestModel);
}
