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

use crate::{
    trade::utils::price_setup,
    types::{DataStore, Market, SatoruAction},
};

use super::error::DepositError;

abigen!(
    DepositHandler,
    "./resources/satoru_DepositHandler.contract_class.json",
);

pub async fn handle_deposit(
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
    deposit: SatoruAction,
) {
    match get_execute_deposit_call(deposit, account.clone()).await {
        Ok(execute_deposit_call) => {
            let deposit_execution_multicall =
                account.execute(vec![execute_deposit_call]).send().await;

            match deposit_execution_multicall {
                Ok(_multicall) => {
                    // TODO: poll transaction status
                }
                Err(e) => {
                    error!("Deposit execution multicall failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to get execute deposit call: {:?}", e);
        }
    }
}

async fn get_execute_deposit_call(
    deposit: SatoruAction,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Result<Call, DepositError> {
    let deposit_handler_address = env::var("DEPOSIT_HANDLER")
        .map_err(|_e| DepositError::EnvVarNotSet("DEPOSIT_HANDLER".to_owned()))?;
    let deposit_handler = DepositHandler::new(
        FieldElement::from_hex_be(&deposit_handler_address).map_err(|e| {
            DepositError::ConversionError(format!("deposit_handler_address: {}", e))
        })?,
        account.clone(),
    );

    let data_store_address =
        env::var("DATA_STORE").map_err(|_e| DepositError::EnvVarNotSet("DATA_STORE".to_owned()))?;

    let data_store_felt = FieldElement::from_hex_be(&data_store_address)
        .map_err(|e| DepositError::ConversionError(format!("data_store_address: {}", e)))?;

    let data_store = DataStore::new(data_store_felt, account.clone());

    let market_key_felt = FieldElement::from_hex_be(&deposit.market)
        .map_err(|e| DepositError::ConversionError(format!("deposit.key: {}", e)))?;

    let market_datastore = data_store
        .get_market(&ContractAddress::from(market_key_felt))
        .call()
        .await
        .map_err(|e| DepositError::SmartContractError(format!("Could not get market: {}", e)))?;
    let market: Market = Market {
        // TODO optimize
        long_token: market_datastore.long_token,
        market_token: market_datastore.market_token,
        index_token: market_datastore.index_token,
        short_token: market_datastore.short_token,
    };
    let price = price_setup(deposit.time_stamp, market.clone())
        .await
        .map_err(|e| DepositError::PriceError(e.to_string()))?;

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
        compacted_max_prices: vec![price, U256 { low: 1, high: 0 }], // TODO replace 1 by real short token price
        compacted_max_prices_indexes: vec![U256 { low: 0, high: 0 }],
        signatures: vec![
            vec![
                FieldElement::from_hex_be("0x").map_err(|_e| {
                    DepositError::ConversionError("Cannot convert string to felt".to_owned())
                })?,
                FieldElement::from_hex_be("0x").map_err(|_e| {
                    DepositError::ConversionError("Cannot convert string to felt".to_owned())
                })?,
            ],
            vec![
                FieldElement::from_hex_be("0x").map_err(|_e| {
                    DepositError::ConversionError("Cannot convert string to felt".to_owned())
                })?,
                FieldElement::from_hex_be("0x").map_err(|_e| {
                    DepositError::ConversionError("Cannot convert string to felt".to_owned())
                })?,
            ],
        ],
        price_feed_tokens: vec![],
    };

    Ok(deposit_handler.execute_deposit_getcall(
        &FieldElement::from_hex_be(&deposit.key).map_err(|_e| {
            DepositError::ConversionError("Cannot convert string to felt".to_owned())
        })?,
        &set_prices_params,
    ))
}
