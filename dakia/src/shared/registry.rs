use async_trait::async_trait;

#[async_trait]
pub trait Registry<I> {
    async fn register(&self, key: String, item: I) -> ();
    async fn get(&self, key: &str) -> Option<I>;
}
