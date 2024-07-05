use cainome::cairo_serde::{ContractAddress, U256};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use starknet::core::types::FieldElement;

use crate::liquidation::utils::{Market, Position};

// An enum representing the types of database actions.
#[derive(Deserialize, Debug)]
pub enum ActionType {
    INSERT,
    UPDATE,
}

// A struct that contains all fields of differents user actions (Order, Deposit, Withdrawal).
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SatoruAction {
    // Shared.
    pub block_number: u64,
    pub time_stamp: String,
    pub transaction_hash: String,
    pub key: String,
    pub account: String,
    pub receiver: String,
    pub callback_contract: String,
    pub ui_fee_receiver: String,
    pub market: String,
    pub execution_fee: u128,
    pub callback_gas_limit: u128,
    pub updated_at_block: u64,

    // Order specific.
    pub order_type: Option<String>,
    pub decrease_position_swap_type: Option<String>,
    pub initial_collateral_token: Option<String>,
    pub swap_path: Option<String>,
    pub size_delta_usd: Option<u128>,
    pub initial_collateral_delta_amount: Option<u128>,
    pub trigger_price: Option<u128>,
    pub acceptable_price: Option<u128>,
    pub min_output_amount: Option<u128>,
    pub is_long: Option<bool>,
    pub is_frozen: Option<bool>,

    // Deposit specific.
    pub initial_long_token: Option<String>,
    pub initial_short_token: Option<String>,
    pub initial_long_token_amount: Option<u128>,
    pub initial_short_token_amount: Option<u128>,
    pub min_market_tokens: Option<u128>,

    // Withdrawal specific
    pub market_token_amount: Option<u128>,
    pub min_long_token_amount: Option<u128>,
    pub min_short_token_amount: Option<u128>,

    // Deposit & Withdrawal shared.
    pub long_token_swap_path: Option<String>,
    pub short_token_swap_path: Option<String>,
}

// A struct of a Satoru Position.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Position_ {
    pub key: String,
    pub account: String,
    pub market: String,
    pub collateral_token: String,
    pub size_in_usd: u128,
    pub size_in_tokens: u128,
    pub collateral_amount: u128,
    pub borrowing_factor: u128,
    pub funding_fee_amount_per_size: u128,
    pub long_token_claimable_funding_amount_per_size: u128,
    pub short_token_claimable_funding_amount_per_size: u128,
    pub increased_at_block: u64,
    pub decreased_at_block: u64,
    pub is_long: bool,
}

// A struct of a Satoru Market.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Market_ {
    pub market_token: String,
    pub index_token: String,
    pub long_token: String,
    pub short_token: String,
}

// A struct of a Satoru Market.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MarketPrices_ {
    pub index_token_price: String,
    pub long_token_price: String,
    pub short_token_price: String,
}

// A struct representing the payload of a notification.
// @table: Option<The table name in the database.
// @action_type: Option<The type of action (using the ActionType enum).
// @row_data: Option<The data of the affected row.
#[derive(Deserialize, Debug)]
pub struct Payload {
    pub table: String,
    pub action_type: ActionType,
    pub row_data: SatoruAction,
}

impl<'r> FromRow<'r, PgRow> for Position {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let key: &str = row.try_get("key").expect("Couldn't decode position");
        let account: &str = row.try_get("account").expect("Couldn't decode position");
        let market: &str = row.try_get("market").expect("Couldn't decode position");
        let collateral_token = row
            .try_get("collateral_token")
            .expect("Couldn't decode position");
        let size_in_usd: u128 = u128::from_str_radix(
            row.try_get::<'r, &str, _>("size_in_usd")
                .expect("Could not get size_in_usd"),
            10,
        )
        .expect("failed to convert string to u128");
        let size_in_tokens: u128 = u128::from_str_radix(
            row.try_get::<'r, &str, _>("size_in_tokens")
                .expect("Could not get size_in_tokens"),
            10,
        )
        .expect("failed to convert string to u128");
        let collateral_amount: u128 = u128::from_str_radix(
            row.try_get::<'r, &str, _>("collateral_amount")
                .expect("Could not get collateral_amount"),
            10,
        )
        .expect("failed to convert string to u128");
        let borrowing_factor: u128 = u128::from_str_radix(
            row.try_get::<'r, &str, _>("borrowing_factor")
                .expect("Could not get borrowing_factor"),
            10,
        )
        .expect("failed to convert string to u128");
        let funding_fee_amount_per_size: u128 = u128::from_str_radix(
            row.try_get::<'r, &str, _>("funding_fee_amount_per_size")
                .expect("Could not get funding_fee_amount_per_size"),
            10,
        )
        .expect("failed to convert string to u128");
        let long_token_claimable_funding_amount_per_size: u128 = u128::from_str_radix(
            row.try_get::<'r, &str, _>("long_token_claimable_funding_amount_per_size")
                .expect("Could not get long_token_claimable_funding_amount_per_size"),
            10,
        )
        .expect("failed to convert string to u128");
        let short_token_claimable_funding_amount_per_size: u128 = u128::from_str_radix(
            row.try_get::<'r, &str, _>("short_token_claimable_funding_amount_per_size")
                .expect("Could not get short_token_claimable_funding_amount_per_size"),
            10,
        )
        .expect("failed to convert string to u128");
        let increased_at_block: u64 = u64::from_str_radix(
            row.try_get::<'r, &str, _>("increased_at_block")
                .expect("Could not get increased_at_block"),
            10,
        )
        .expect("failed to convert string to u128");
        let decreased_at_block: u64 = u64::from_str_radix(
            row.try_get::<'r, &str, _>("decreased_at_block")
                .expect("Could not get decreased_at_block"),
            10,
        )
        .expect("failed to convert string to u128");
        Ok(Position {
            key: FieldElement::from_hex_be(key).expect("Could not convert key to felt"),
            account: ContractAddress::from(
                FieldElement::from_hex_be(account).expect("Could not convert account to felt"),
            ),
            market: ContractAddress::from(
                FieldElement::from_hex_be(market).expect("Could not convert market to felt"),
            ),
            collateral_token: ContractAddress::from(
                FieldElement::from_hex_be(collateral_token)
                    .expect("Could not convert collateral_token to felt"),
            ),
            size_in_usd: U256 {
                low: size_in_usd,
                high: 0,
            },
            size_in_tokens: U256 {
                low: size_in_tokens,
                high: 0,
            },
            collateral_amount: U256 {
                low: collateral_amount,
                high: 0,
            },
            borrowing_factor: U256 {
                low: borrowing_factor,
                high: 0,
            },
            funding_fee_amount_per_size: U256 {
                low: funding_fee_amount_per_size,
                high: 0,
            },
            long_token_claimable_funding_amount_per_size: U256 {
                low: long_token_claimable_funding_amount_per_size,
                high: 0,
            },
            short_token_claimable_funding_amount_per_size: U256 {
                low: short_token_claimable_funding_amount_per_size,
                high: 0,
            },
            increased_at_block,
            decreased_at_block,
            is_long: row
                .try_get::<'r, bool, _>("is_long")
                .expect("Could not get is_long"),
        })
    }
}

impl<'r> FromRow<'r, PgRow> for Market {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let market_token: &str = row
            .try_get("market_token")
            .expect("Couldn't decode market_token");
        let index_token: &str = row
            .try_get("index_token")
            .expect("Couldn't decode index_token");
        let long_token: &str = row
            .try_get("long_token")
            .expect("Couldn't decode long_token");
        let short_token: &str = row
            .try_get("short_token")
            .expect("Couldn't decode short_token");

        Ok(Market {
            market_token: ContractAddress::from(
                FieldElement::from_hex_be(market_token)
                    .expect("Could not convert market_token to felt"),
            ),
            index_token: ContractAddress::from(
                FieldElement::from_hex_be(index_token)
                    .expect("Could not convert index_token to felt"),
            ),
            long_token: ContractAddress::from(
                FieldElement::from_hex_be(long_token)
                    .expect("Could not convert long_token to felt"),
            ),
            short_token: ContractAddress::from(
                FieldElement::from_hex_be(short_token)
                    .expect("Could not convert short_token to felt"),
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_withdrawal() {
        let json_data = r#"
        {
            "block_number": 100,
            "time_stamp": "1708839",
            "transaction_hash": "0x1",
            "key": "0x2",
            "account": "0x3",
            "receiver": "0x4",
            "callback_contract": "0x5",
            "ui_fee_receiver": "0x6",
            "market": "0x7",
            "execution_fee": 10,
            "callback_gas_limit": 11,
            "updated_at_block": 12,
            "market_token_amount": 13,
            "min_long_token_amount": 14,
            "min_short_token_amount": 15
        }"#;

        let action: SatoruAction = serde_json::from_str(json_data).unwrap();
        assert_eq!(action.block_number, 100);
        assert_eq!(action.time_stamp, "1708839");
        assert_eq!(action.transaction_hash, "0x1");
        assert_eq!(action.key, "0x2");
        assert_eq!(action.account, "0x3");
        assert_eq!(action.receiver, "0x4");
        assert_eq!(action.callback_contract, "0x5");
        assert_eq!(action.ui_fee_receiver, "0x6");
        assert_eq!(action.market, "0x7");
        assert_eq!(action.execution_fee, 10);
        assert_eq!(action.callback_gas_limit, 11);
        assert_eq!(action.updated_at_block, 12);
        assert_eq!(action.market_token_amount, Some(13));
        assert_eq!(action.min_long_token_amount, Some(14));
        assert_eq!(action.min_short_token_amount, Some(15));
    }

    #[test]
    fn test_invalid_data() {
        let json_data = r#"
        {
            "some_other_id": "000",
            "amount": 400.0
        }"#;

        let result: Result<SatoruAction, _> = serde_json::from_str(json_data);
        assert!(result.is_err());
    }
}
