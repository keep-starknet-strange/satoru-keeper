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

use crate::{trade::{order::handle::DataStore, utils::{get_set_primary_price_call, price_setup}}, types::SatoruAction};

use super::error::WithdrawalError;

abigen!(
    WithdrawalHandler,
    "./resources/satoru_WithdrawalHandler.contract_class.json",
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

pub async fn handle_withdrawal(
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
    withdrawal: SatoruAction,
) {
    match get_execute_withdrawal_call(withdrawal, account.clone()).await {
        Ok(execute_withdrawal_call) => {
            let withdrawal_execution_multicall = account
                .execute(vec![execute_withdrawal_call])
                .send()
                .await;

            match withdrawal_execution_multicall {
                Ok(multicall) => {
                    // TODO: poll transaction status
                }
                Err(e) => {
                    error!("Withdrawal execution multicall failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to get execute withdrawal call: {:?}", e);
        }
    }
}

async fn get_execute_withdrawal_call(
    withdrawal: SatoruAction,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Result<Call, WithdrawalError> {
    let withdrawal_handler_address = env::var("WITHDRAWAL_HANDLER")
        .map_err(|e| WithdrawalError::EnvVarNotSet("WITHDRAWAL_HANDLER".to_owned()))?;
    let withdrawal_handler = WithdrawalHandler::new(
        FieldElement::from_hex_be(&withdrawal_handler_address).map_err(|e| {
            WithdrawalError::ConversionError(format!("withdrawal_handler_address: {}", e))
        })?,
        account.clone(),
    );

    let data_store_address =
        env::var("DATA_STORE").map_err(|e| WithdrawalError::EnvVarNotSet("DATA_STORE".to_owned()))?;

    let data_store_felt = FieldElement::from_hex_be(&data_store_address)
        .map_err(|e| WithdrawalError::ConversionError(format!("data_store_address: {}", e)))?;

    let data_store = DataStore::new(data_store_felt, account.clone());

    let market_key_felt = FieldElement::from_hex_be(&withdrawal.key)
        .map_err(|e| WithdrawalError::ConversionError(format!("withdrawal.key: {}", e)))?;

    let market = data_store
        .get_market(&ContractAddress::from(market_key_felt))
        .call()
        .await
        .map_err(|e| WithdrawalError::SmartContractError(format!("Could not get market: {}", e)))?;

    let price = price_setup(withdrawal.time_stamp, market.clone()).await.map_err(|e| WithdrawalError::PriceError(e.to_string()))?;

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
                FieldElement::from_hex_be("0x")
                    .map_err(|e| WithdrawalError::ConversionError("Cannot convert string to felt".to_owned()))?,
                FieldElement::from_hex_be("0x")
                    .map_err(|e| WithdrawalError::ConversionError("Cannot convert string to felt".to_owned()))?,
            ],
            vec![
                FieldElement::from_hex_be("0x")
                    .map_err(|e| WithdrawalError::ConversionError("Cannot convert string to felt".to_owned()))?,
                FieldElement::from_hex_be("0x")
                    .map_err(|e| WithdrawalError::ConversionError("Cannot convert string to felt".to_owned()))?,
            ],
        ],
        price_feed_tokens: vec![],
    };

    Ok(withdrawal_handler.execute_withdrawal_getcall(
        &FieldElement::from_hex_be(&withdrawal.key).map_err(|e| {
            WithdrawalError::ConversionError("Cannot convert string to felt".to_owned())
        })?,
        &set_prices_params,
    ))
}
