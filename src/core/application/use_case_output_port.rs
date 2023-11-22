use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait UseCaseOutputPort<OutputData>: Send + Sync {
    async fn success(&self, output_data: OutputData);
    async fn failure(&self, error: Box<dyn Error + Send>);
}
