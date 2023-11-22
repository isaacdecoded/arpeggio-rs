use async_trait::async_trait;

#[async_trait]
pub trait View<ViewModel>: Send + Sync {
    async fn transform(&self, view_model: ViewModel);
}
