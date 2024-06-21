use crate::events::event::{Event, GenericEvent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub order_type: Option<String>,
    pub decrease_position_swap_type: Option<String>,
    pub account: Option<String>,
    pub receiver: Option<String>,
    pub callback_contract: Option<String>,
    pub ui_fee_receiver: Option<String>,
    pub market: Option<String>,
    pub initial_collateral_token: Option<String>,
    pub swap_path: Option<String>,
    pub size_delta_usd: Option<i64>,
    pub initial_collateral_delta_amount: Option<i64>,
    pub trigger_price: Option<i64>,
    pub acceptable_price: Option<i64>,
    pub execution_fee: Option<i64>,
    pub callback_gas_limit: Option<i64>,
    pub min_output_amount: Option<i64>,
    pub updated_at_block: Option<i64>,
    pub is_long: Option<bool>,
    pub is_frozen: Option<bool>,
}

#[async_trait]
impl Event for Order {
    fn event_key() -> &'static str {
        "03427759bfd3b941f14e687e129519da3c9b0046c5b9aaa290bb1dede63753b3"
    }

    fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> =
            event.data.split(',').map(|s| Some(s.to_string())).collect();
        Order {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: event.key,
            order_type: data_parts.get(0).cloned().unwrap_or(None),
            decrease_position_swap_type: data_parts.get(1).cloned().unwrap_or(None),
            account: data_parts.get(2).cloned().unwrap_or(None),
            receiver: data_parts.get(3).cloned().unwrap_or(None),
            callback_contract: data_parts.get(4).cloned().unwrap_or(None),
            ui_fee_receiver: data_parts.get(5).cloned().unwrap_or(None),
            market: data_parts.get(6).cloned().unwrap_or(None),
            initial_collateral_token: data_parts.get(7).cloned().unwrap_or(None),
            swap_path: data_parts.get(8).cloned().unwrap_or(None),
            size_delta_usd: data_parts
                .get(9)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            initial_collateral_delta_amount: data_parts
                .get(10)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            trigger_price: data_parts
                .get(11)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            acceptable_price: data_parts
                .get(12)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            execution_fee: data_parts
                .get(13)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            callback_gas_limit: data_parts
                .get(14)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            min_output_amount: data_parts
                .get(15)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            updated_at_block: data_parts
                .get(16)
                .and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            is_long: data_parts
                .get(17)
                .and_then(|s| s.as_ref().map(|v| v.parse::<bool>().ok()).flatten()),
            is_frozen: data_parts
                .get(18)
                .and_then(|s| s.as_ref().map(|v| v.parse::<bool>().ok()).flatten()),
        }
    }

    async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO orders (
                block_number, transaction_hash, key, order_type, decrease_position_swap_type, account,
                receiver, callback_contract, ui_fee_receiver, market, initial_collateral_token, swap_path,
                size_delta_usd, initial_collateral_delta_amount, trigger_price, acceptable_price,
                execution_fee, callback_gas_limit, min_output_amount, updated_at_block, is_long, is_frozen
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12,
                $13, $14, $15, $16,
                $17, $18, $19, $20, $21, $22
            )",
                self.block_number, self.transaction_hash, self.key, self.order_type,
                self.decrease_position_swap_type, self.account, self.receiver,
                self.callback_contract, self.ui_fee_receiver, self.market,
                self.initial_collateral_token, self.swap_path, self.size_delta_usd,
                self.initial_collateral_delta_amount, self.trigger_price, self.acceptable_price,
                self.execution_fee, self.callback_gas_limit, self.min_output_amount,
                self.updated_at_block, self.is_long, self.is_frozen,
            )
            .execute(pool)
            .await?;
        Ok(())
    }
}
