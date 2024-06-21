use std::{env, sync::Arc, vec};

use cainome::{
    cairo_serde::{ContractAddress, U256},
    rs::abigen,
};
use starknet::{
    accounts::{Account, Call, SingleOwnerAccount},
    core::types::FieldElement,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};

use crate::{trade::utils::get_set_primary_price_call, types::SatoruAction};

abigen!(
    WithdrawalHandler,
    "./resources/satoru_WithdrawalHandler.contract_class.json",
);

abigen!(
    DataStore,
    "./resources/satoru_DataStore.contract_class.json",
    type_aliases {
        satoru::data::data_store::DataStore::Event as Event_;
        satoru::utils::span32::Span32 as Span32_;
    }
);

abigen!(
    Oracle,
    "./resources/satoru_Oracle.contract_class.json",
    type_aliases {
        satoru::price::price::Price as Price_;
        satoru::oracle::oracle::Oracle::Event as Event__;
    }
);

pub async fn handle_withdrawal(
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
    withdrawal: SatoruAction,
) {
    let set_price_call = get_set_primary_price_call(withdrawal.clone(), account.clone()).await;

    let execute_withdrawal_call = get_execute_withdrawal_call(withdrawal, account.clone());

    let _withdrawal_execution_multicall = account
        .execute(vec![set_price_call, execute_withdrawal_call])
        .send()
        .await
        .expect("Withdrawal execution multicall failed");
    // TODO: poll transaction status
}

fn get_execute_withdrawal_call(
    withdrawal: SatoruAction,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Call {
    let withdrawal_handler_address =
        env::var("WITHDRAWAL_HANDLER").expect("WITHDRAWAL_HANDLER env variable not set");
    let withdrawal_handler = WithdrawalHandler::new(
        FieldElement::from_hex_be(&withdrawal_handler_address)
            .expect("Conversion error: withdrawal_handler_address"),
        account.clone(),
    );

    let set_prices_params: SetPricesParams = SetPricesParams {
        signer_info: U256 { low: 1, high: 0 },
        tokens: vec![
            ContractAddress::from(
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
            ),
            ContractAddress::from(
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
            ),
        ],
        compacted_min_oracle_block_numbers: vec![63970, 63970],
        compacted_max_oracle_block_numbers: vec![64901, 64901],
        compacted_oracle_timestamps: vec![171119803, 10],
        compacted_decimals: vec![U256 { low: 1, high: 0 }, U256 { low: 1, high: 0 }],
        compacted_min_prices: vec![U256 {
            low: 2147483648010000,
            high: 0,
        }],
        compacted_min_prices_indexes: vec![U256 { low: 0, high: 0 }],
        compacted_max_prices: vec![U256 {
            low: 2147483648010000,
            high: 0,
        }],
        compacted_max_prices_indexes: vec![U256 { low: 0, high: 0 }],
        signatures: vec![
            vec![
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
            ],
            vec![
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
            ],
        ],
        price_feed_tokens: vec![],
    };

    withdrawal_handler.execute_withdrawal_getcall(
        &FieldElement::from_hex_be(&withdrawal.key).expect("Cannot convert string to felt"),
        &set_prices_params,
    )
}
