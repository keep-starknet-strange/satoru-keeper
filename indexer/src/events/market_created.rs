use crate::events::event::{Event, GenericEvent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketCreated {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub creator: Option<String>,
    pub market_token: Option<String>,
    pub index_token: Option<String>,
    pub long_token: Option<String>,
    pub short_token: Option<String>,
    pub market_type: Option<String>,
}

#[async_trait]
impl Event for MarketCreated {
    fn event_key() -> &'static str {
        "015d762f1fc581b3e684cf095d93d3a2c10754f60124b09bec8bf3d76473baaf"
    }

    fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> =
            event.data.split(',').map(|s| Some(s.to_string())).collect();
        MarketCreated {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: event.key,
            creator: data_parts.get(0).cloned().unwrap_or(None),
            market_token: data_parts.get(1).cloned().unwrap_or(None),
            index_token: data_parts.get(2).cloned().unwrap_or(None),
            long_token: data_parts.get(3).cloned().unwrap_or(None),
            short_token: data_parts.get(4).cloned().unwrap_or(None),
            market_type: data_parts.get(5).cloned().unwrap_or(None),
        }
    }

    async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO market_created (
                block_number, transaction_hash, key, creator, market_token, index_token,
                long_token, short_token, market_type
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9
            )",
            self.block_number,
            self.transaction_hash,
            self.key,
            self.creator,
            self.market_token,
            self.index_token,
            self.long_token,
            self.short_token,
            self.market_type
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
