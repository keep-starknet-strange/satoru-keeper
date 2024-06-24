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
    pub order_type: Option<OrderType>, // Use OrderType enum
    pub decrease_position_swap_type: Option<DecreasePositionSwapType>, // Use DecreasePositionSwapType enum
    pub account: Option<String>,
    pub receiver: Option<String>,
    pub callback_contract: Option<String>,
    pub ui_fee_receiver: Option<String>,
    pub market: Option<String>,
    pub initial_collateral_token: Option<String>,
    pub swap_path: Option<Vec<String>>, // Changed to Option<Vec<String>>
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    MarketSwap,
    LimitSwap,
    MarketIncrease,
    LimitIncrease,
    MarketDecrease,
    LimitDecrease,
    StopLossDecrease,
    Liquidation,
}

impl std::str::FromStr for OrderType {
    type Err = ();

    fn from_str(input: &str) -> Result<OrderType, Self::Err> {
        match input {
            "4" => Ok(OrderType::MarketSwap),
            "5" => Ok(OrderType::LimitSwap),
            "6" => Ok(OrderType::MarketIncrease),
            "7" => Ok(OrderType::LimitIncrease),
            "8" => Ok(OrderType::MarketDecrease),
            "9" => Ok(OrderType::LimitDecrease),
            "10" => Ok(OrderType::StopLossDecrease),
            "11" => Ok(OrderType::Liquidation),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DecreasePositionSwapType {
    NoSwap,
    SwapPnlTokenToCollateralToken,
    SwapCollateralTokenToPnlToken,
}

impl std::str::FromStr for DecreasePositionSwapType {
    type Err = ();

    fn from_str(input: &str) -> Result<DecreasePositionSwapType, Self::Err> {
        match input {
            "0" => Ok(DecreasePositionSwapType::NoSwap),
            "1" => Ok(DecreasePositionSwapType::SwapPnlTokenToCollateralToken),
            "2" => Ok(DecreasePositionSwapType::SwapCollateralTokenToPnlToken),
            _ => Err(()),
        }
    }
}

#[async_trait]
impl Event for Order {
    fn event_key() -> &'static str {
        "03427759bfd3b941f14e687e129519da3c9b0046c5b9aaa290bb1dede63753b3"
    }

    fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> =
            event.data.split(',').map(|s| Some(s.to_string())).collect();

        let swap_path_len = data_parts.get(10)
            .and_then(|s| s.as_ref().map(|v| v.parse::<usize>().ok()).flatten())
            .unwrap_or(0);
        let swap_path: Vec<String> = (0..swap_path_len)
            .filter_map(|i| data_parts.get(11 + i).cloned().unwrap_or(None))
            .collect();

        Order {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: data_parts.get(0).cloned().unwrap_or(None),
            order_type: data_parts.get(2).and_then(|s| s.as_ref().and_then(|v| v.parse::<OrderType>().ok())),
            decrease_position_swap_type: data_parts.get(3).and_then(|s| s.as_ref().and_then(|v| v.parse::<DecreasePositionSwapType>().ok())),
            account: data_parts.get(4).cloned().unwrap_or(None),
            receiver: data_parts.get(5).cloned().unwrap_or(None),
            callback_contract: data_parts.get(6).cloned().unwrap_or(None),
            ui_fee_receiver: data_parts.get(7).cloned().unwrap_or(None),
            market: data_parts.get(8).cloned().unwrap_or(None),
            initial_collateral_token: data_parts.get(9).cloned().unwrap_or(None),
            swap_path: Some(swap_path),
            size_delta_usd: data_parts.get(11 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| u64::from_str_radix(v, 16).ok().map(|x| x as i64))),
            initial_collateral_delta_amount: data_parts.get(13 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| u64::from_str_radix(v, 16).ok().map(|x| x as i64))),
            trigger_price: data_parts.get(15 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| u64::from_str_radix(v, 16).ok().map(|x| x as i64))),
            acceptable_price: data_parts.get(17 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| u64::from_str_radix(v, 16).ok().map(|x| x as i64))),
            execution_fee: data_parts.get(19 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| u64::from_str_radix(v, 16).ok().map(|x| x as i64))),
            callback_gas_limit: data_parts.get(21 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| u64::from_str_radix(v, 16).ok().map(|x| x as i64))),
            min_output_amount: data_parts.get(23 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| u64::from_str_radix(v, 16).ok().map(|x| x as i64))),
            updated_at_block: data_parts.get(25 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| u64::from_str_radix(v, 16).ok().map(|x| x as i64))),
            is_long: data_parts.get(26 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| v.parse::<bool>().ok())),
            is_frozen: data_parts.get(27 + swap_path_len).and_then(|s| s.as_ref().and_then(|v| v.parse::<bool>().ok())),
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
            self.block_number,
            self.transaction_hash,
            self.key,
            self.order_type.as_ref().map(|ot| format!("{:?}", ot)),
            self.decrease_position_swap_type.as_ref().map(|dt| format!("{:?}", dt)),
            self.account,
            self.receiver,
            self.callback_contract,
            self.ui_fee_receiver,
            self.market,
            self.initial_collateral_token,
            self.swap_path.as_ref().map(|sp| sp.join(",")),
            self.size_delta_usd,
            self.initial_collateral_delta_amount,
            self.trigger_price,
            self.acceptable_price,
            self.execution_fee,
            self.callback_gas_limit,
            self.min_output_amount,
            self.updated_at_block,
            self.is_long,
            self.is_frozen
            )
            .execute(pool)
            .await?;
        Ok(())
    }
}
