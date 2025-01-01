use async_trait::async_trait;

use crate::error::DakiaResult;

#[async_trait]
pub trait Registry<I> {
    async fn register(&self, key: String, item: I) -> ();
    async fn get(&self, key: &str) -> DakiaResult<Option<I>>;
}
