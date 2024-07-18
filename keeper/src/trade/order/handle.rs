use std::{env, sync::Arc, vec};

use cainome::{
    cairo_serde::{ContractAddress, U256},
    rs::abigen,
};
use log::error;
use starknet::{
    accounts::{Account, Call, SingleOwnerAccount},
    core::types::FieldElement,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};

use crate::{trade::utils::price_setup, types::SatoruAction};

use super::error::OrderError;

abigen!(
    OrderHandler,
    "./resources/satoru_OrderHandler.contract_class.json",
);

abigen!(
    DataStore,
    "./resources/satoru_DataStore.contract_class.json",
    type_aliases {
        satoru::order::order::OrderType as OrderType_;
        satoru::data::data_store::DataStore::Event as Event_;
        satoru::order::order::DecreasePositionSwapType as Decrease_;
        satoru::utils::span32::Span32 as Span32_;
    }
);

abigen!(
    Oracle,
    "./resources/satoru_Oracle.contract_class.json",
    type_aliases {
        satoru::price::price::Price as Price_;
        satoru::oracle::oracle::Oracle::Event as Event__;
        satoru::oracle::oracle_utils::SetPricesParams as SetPricesParams_;
    }
);

pub async fn handle_order(
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
    order: SatoruAction,
) {
    match get_execute_order_call(order, account.clone()).await {
        Ok(execute_order_call) => {
            let order_execution_multicall = account.execute(vec![execute_order_call]).send().await;

            match order_execution_multicall {
                Ok(_multicall) => {
                    // TODO: poll transaction status
                }
                Err(e) => {
                    error!("Order execution multicall failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to get execute order call: {:?}", e);
        }
    }
}

async fn get_execute_order_call(
    order: SatoruAction,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Result<Call, OrderError> {
    let order_handler_address = env::var("ORDER_HANDLER")
        .map_err(|e| OrderError::EnvVarNotSet("ORDER_HANDLER".to_owned()))?;
    let order_handler = OrderHandler::new(
        FieldElement::from_hex_be(&order_handler_address)
            .map_err(|e| OrderError::ConversionError(format!("order_handler_address: {}", e)))?,
        account.clone(),
    );

    let data_store_address =
        env::var("DATA_STORE").map_err(|e| OrderError::EnvVarNotSet("DATA_STORE".to_owned()))?;

    let data_store_felt = FieldElement::from_hex_be(&data_store_address)
        .map_err(|e| OrderError::ConversionError(format!("data_store_address: {}", e)))?;

    let data_store = DataStore::new(data_store_felt, account.clone());

    let market_key_felt = if order.order_type == Some("MarketSwap".to_owned()) {
        // TODO set prices for all markets in swap_path
        let swap_paths = parse_hex_addresses(order.swap_path.unwrap())
            .map_err(|e| OrderError::ConversionError("Could not parse swap path".to_owned()))?;
        FieldElement::from_hex_be(swap_paths[0].as_str())
            .map_err(|e| OrderError::ConversionError(format!("order.key: {}", e)))?
    } else {
        FieldElement::from_hex_be(&order.market)
            .map_err(|e| OrderError::ConversionError(format!("order.key: {}", e)))?
    };

    let market = data_store
        .get_market(&ContractAddress::from(market_key_felt))
        .call()
        .await
        .map_err(|e| OrderError::SmartContractError(format!("Could not get market: {}", e)))?;

    let price = price_setup(order.time_stamp, market.clone())
        .await
        .map_err(|e| OrderError::PriceError(e.to_string()))?;

    let price_formated = U256 {
        low: keep_first_digits(price.low, 4),
        high: 0,
    };

    let set_prices_params: SetPricesParams = SetPricesParams {
        signer_info: U256 { low: 1, high: 0 },
        tokens: vec![market.long_token, market.short_token],
        compacted_min_oracle_block_numbers: vec![63970, 63970],
        compacted_max_oracle_block_numbers: vec![64901, 64901],
        compacted_oracle_timestamps: vec![171119803, 10],
        compacted_decimals: vec![U256 { low: 1, high: 0 }, U256 { low: 1, high: 0 }],
        compacted_min_prices: vec![U256 {
            low: 2147483648010000,
            high: 0,
        }],
        compacted_min_prices_indexes: vec![U256 { low: 0, high: 0 }],
        compacted_max_prices: vec![price_formated, U256 { low: 1, high: 0 }], // TODO replace 1 by real short token price
        compacted_max_prices_indexes: vec![U256 { low: 0, high: 0 }],
        signatures: vec![
            vec![
                FieldElement::from_hex_be("0x").map_err(|e| {
                    OrderError::ConversionError("Cannot convert string to felt".to_owned())
                })?,
                FieldElement::from_hex_be("0x").map_err(|e| {
                    OrderError::ConversionError("Cannot convert string to felt".to_owned())
                })?,
            ],
            vec![
                FieldElement::from_hex_be("0x").map_err(|e| {
                    OrderError::ConversionError("Cannot convert string to felt".to_owned())
                })?,
                FieldElement::from_hex_be("0x").map_err(|e| {
                    OrderError::ConversionError("Cannot convert string to felt".to_owned())
                })?,
            ],
        ],
        price_feed_tokens: vec![],
    };

    Ok(order_handler.execute_order_getcall(
        &FieldElement::from_hex_be(&order.key)
            .map_err(|e| OrderError::ConversionError("Cannot convert string to felt".to_owned()))?,
        &set_prices_params,
    ))
}

fn keep_first_digits(number: u128, number_of_digits: u64) -> u128 {
    let number_str = number.to_string(); // Convert number to string and handle negative numbers
    let first_four: String = number_str
        .chars()
        .take(number_of_digits.try_into().unwrap())
        .collect(); // Take the first four characters
    first_four.parse().unwrap() // Convert the first four characters back to an integer
}

fn parse_hex_addresses(hex_string: String) -> Result<Vec<String>, String> {
    hex_string
        .split(',')
        .map(|s| {
            s.parse::<String>()
                .map_err(|e| format!("Failed to parse '{}': {}", s, e))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_data() {
        let price = 3456001111;
        let new_price = keep_first_digits(price, 4);
        assert_eq!(new_price, 3456, "Price is not formated well.");
    }

    #[test]
    fn test_parse_swap_path() {
        let swap_path = "0x111,0x222".to_owned();
        let result = parse_hex_addresses(swap_path).unwrap();
        assert_eq!(
            result,
            vec!["0x111".to_owned(), "0x222".to_owned()],
            "Price is not formated well."
        );
    }
}
