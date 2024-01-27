use async_trait::async_trait;

#[async_trait]
pub trait Controller<RequestObject> {
    async fn execute(&self, request_object: RequestObject);
}
