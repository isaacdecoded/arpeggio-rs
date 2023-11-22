use async_trait::async_trait;

#[async_trait]
pub trait UseCaseInputPort<InputData>: Send {
    async fn interact(&self, input_data: InputData);
}
