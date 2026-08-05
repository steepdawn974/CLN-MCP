use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ClnBackend: Send + Sync + 'static {
    async fn call(&self, method: &str, params: Value) -> Result<Value>;
}
