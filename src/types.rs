use serde::{Deserialize, Serialize};

// An enum representing the types of database actions.
#[derive(Deserialize, Debug)]
pub enum ActionType {
    INSERT,
    UPDATE,
}

// A struct that contains all fields of differents user actions (Order, Deposit, Withdrawal).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SatoruAction {
    // Shared.
    pub block_number: u64,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_withdrawal() {
        let json_data = r#"
        {
            "block_number": 100,
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
