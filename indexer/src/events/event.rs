use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::fmt::Debug;

#[async_trait]
pub trait Event: Debug + Serialize + for<'de> Deserialize<'de> {
    fn event_key() -> &'static str;
    fn from_generic_event(event: GenericEvent) -> Self;
    async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericEvent {
    pub block_number: i64,
    pub timestamp: Option<String>,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub data: String,
}
