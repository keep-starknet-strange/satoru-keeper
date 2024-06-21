use crate::events::event::{Event, GenericEvent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct PoolAmountUpdated {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub market: Option<String>,
    pub token: Option<String>,
    pub delta_mag: Option<i64>,
    pub delta_sign: Option<bool>,
    pub next_value: Option<i64>,
}

#[async_trait]
impl Event for PoolAmountUpdated {
    fn event_key() -> &'static str {
        "03534d650a9b8eb67820f87038b8e8b36b741c6f7eb14d1a7ac5027e80fd4a82"
    }

    fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> =
            event.data.split(',').map(|s| Some(s.to_string())).collect();
        PoolAmountUpdated {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: event.key,
            market: data_parts.get(0).cloned().unwrap_or(None),
            token: data_parts.get(1).cloned().unwrap_or(None),
            delta_mag: data_parts
                .get(2)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            delta_sign: data_parts
                .get(3)
                .and_then(|s| s.as_ref().map(|v| v.parse::<bool>().ok()).flatten()),
            next_value: data_parts
                .get(4)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
        }
    }

    async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO pool_amount_updated (
                block_number, transaction_hash, key, market, token, delta_mag, delta_sign, next_value
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )",
                self.block_number, self.transaction_hash, self.key, self.market, self.token,
                self.delta_mag, self.delta_sign, self.next_value
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
