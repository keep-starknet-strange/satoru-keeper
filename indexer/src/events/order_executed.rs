use crate::events::event::{Event, GenericEvent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderExecuted {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub secondary_order_type: Option<String>,
}

#[async_trait]
impl Event for OrderExecuted {
    fn event_key() -> &'static str {
        "0392fd46c9dd1864ee8b38c8d7dd91cb8e1080856b554ce6d5560dae09b41181"
    }

    fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> =
            event.data.split(',').map(|s| Some(s.to_string())).collect();
        OrderExecuted {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: event.key,
            secondary_order_type: data_parts.get(0).cloned().unwrap_or(None),
        }
    }

    async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO order_executed (
                block_number, transaction_hash, key, secondary_order_type
            ) VALUES (
                $1, $2, $3, $4
            )",
            self.block_number,
            self.transaction_hash,
            self.key,
            self.secondary_order_type
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
