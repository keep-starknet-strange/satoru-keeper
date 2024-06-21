use crate::events::event::{Event, GenericEvent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapInfo {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub order_key: Option<String>,
    pub market: Option<String>,
    pub receiver: Option<String>,
    pub token_in: Option<String>,
    pub token_out: Option<String>,
    pub token_in_price: Option<i64>,
    pub token_out_price: Option<i64>,
    pub amount_in: Option<i64>,
    pub amount_in_after_fees: Option<i64>,
    pub amount_out: Option<i64>,
    pub price_impact_usd_mag: Option<i64>,
    pub price_impact_usd_sign: Option<bool>,
    pub price_impact_amount_mag: Option<i64>,
    pub price_impact_amount_sign: Option<bool>,
}

#[async_trait]
impl Event for SwapInfo {
    fn event_key() -> &'static str {
        "03534d650a9b8eb67820f87038b8e8b36b741c6f7eb14d1a7ac5027e80fd4a82"
    }

    fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> =
            event.data.split(',').map(|s| Some(s.to_string())).collect();
        SwapInfo {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: event.key,
            order_key: data_parts.get(0).cloned().unwrap_or(None),
            market: data_parts.get(1).cloned().unwrap_or(None),
            receiver: data_parts.get(2).cloned().unwrap_or(None),
            token_in: data_parts.get(3).cloned().unwrap_or(None),
            token_out: data_parts.get(4).cloned().unwrap_or(None),
            token_in_price: data_parts
                .get(5)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            token_out_price: data_parts
                .get(6)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            amount_in: data_parts
                .get(7)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            amount_in_after_fees: data_parts
                .get(8)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            amount_out: data_parts
                .get(9)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            price_impact_usd_mag: data_parts
                .get(10)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            price_impact_usd_sign: data_parts
                .get(11)
                .and_then(|s| s.as_ref().map(|v| v.parse::<bool>().ok()).flatten()),
            price_impact_amount_mag: data_parts
                .get(12)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            price_impact_amount_sign: data_parts
                .get(13)
                .and_then(|s| s.as_ref().map(|v| v.parse::<bool>().ok()).flatten()),
        }
    }

    async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO swap_info (
                block_number, transaction_hash, key, order_key, market, receiver,
                token_in, token_out, token_in_price, token_out_price, amount_in, amount_in_after_fees,
                amount_out, price_impact_usd_mag, price_impact_usd_sign, price_impact_amount_mag,
                price_impact_amount_sign
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )",
                self.block_number, self.transaction_hash, self.key, self.order_key,
                self.market, self.receiver, self.token_in, self.token_out,
                self.token_in_price, self.token_out_price, self.amount_in,
                self.amount_in_after_fees, self.amount_out, self.price_impact_usd_mag,
                self.price_impact_usd_sign, self.price_impact_amount_mag,
                self.price_impact_amount_sign
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
