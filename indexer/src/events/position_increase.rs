use crate::events::event::{Event, GenericEvent};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionIncrease {
    pub key: String,
    pub account: String,
    pub market: String,
    pub collateral_token: String,
    pub size_in_usd: BigDecimal,
    pub size_in_tokens: BigDecimal,
    pub collateral_amount: BigDecimal,
    pub borrowing_factor: BigDecimal,
    pub funding_fee_amount_per_size: BigDecimal,
    pub long_token_claimable_funding_amount_per_size: BigDecimal,
    pub short_token_claimable_funding_amount_per_size: BigDecimal,
    pub increased_at_block: i64,
    pub decreased_at_block: i64,
    pub is_long: bool,
}

#[async_trait]
impl Event for PositionIncrease {
    fn event_key() -> &'static str {
        "014196ccb31f81a3e67df18f2a62cbfb50009c80a7d3c728a3f542e3abc5cb63"
    }

    fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> = event.data.split(',').map(|s| Some(s.to_string())).collect();

        PositionIncrease {
            key: data_parts.get(0).cloned().unwrap_or(None).unwrap(),
            account: data_parts.get(1).cloned().unwrap_or(None).unwrap(),
            market: data_parts.get(2).cloned().unwrap_or(None).unwrap(),
            collateral_token: data_parts.get(3).cloned().unwrap_or(None).unwrap(),
            size_in_usd: BigDecimal::from_str(&data_parts.get(4).cloned().unwrap_or(None).unwrap()).unwrap(),
            size_in_tokens: BigDecimal::from_str(&data_parts.get(5).cloned().unwrap_or(None).unwrap()).unwrap(),
            collateral_amount: BigDecimal::from_str(&data_parts.get(6).cloned().unwrap_or(None).unwrap()).unwrap(),
            borrowing_factor: BigDecimal::from_str(&data_parts.get(7).cloned().unwrap_or(None).unwrap()).unwrap(),
            funding_fee_amount_per_size: BigDecimal::from_str(&data_parts.get(8).cloned().unwrap_or(None).unwrap()).unwrap(),
            long_token_claimable_funding_amount_per_size: BigDecimal::from_str(&data_parts.get(9).cloned().unwrap_or(None).unwrap()).unwrap(),
            short_token_claimable_funding_amount_per_size: BigDecimal::from_str(&data_parts.get(10).cloned().unwrap_or(None).unwrap()).unwrap(),
            increased_at_block: data_parts.get(11).cloned().unwrap_or(None).unwrap().parse().unwrap(),
            decreased_at_block: data_parts.get(12).cloned().unwrap_or(None).unwrap().parse().unwrap(),
            is_long: data_parts.get(13).cloned().unwrap_or(None).unwrap() == "1",
        }
    }

    async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO position_increase (
                key, account, market, collateral_token, size_in_usd, size_in_tokens, collateral_amount,
                borrowing_factor, funding_fee_amount_per_size, long_token_claimable_funding_amount_per_size,
                short_token_claimable_funding_amount_per_size, increased_at_block, decreased_at_block, is_long
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7,
                $8, $9, $10,
                $11, $12, $13, $14
            )",
            self.key,
            self.account,
            self.market,
            self.collateral_token,
            self.size_in_usd,
            self.size_in_tokens,
            self.collateral_amount,
            self.borrowing_factor,
            self.funding_fee_amount_per_size,
            self.long_token_claimable_funding_amount_per_size,
            self.short_token_claimable_funding_amount_per_size,
            self.increased_at_block,
            self.decreased_at_block,
            self.is_long
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
