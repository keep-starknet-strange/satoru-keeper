use std::{env, sync::Arc, vec};

use cainome::{
    cairo_serde::{ContractAddress, U256},
    rs::abigen,
};
use starknet::{
    accounts::{Account, SingleOwnerAccount},
    core::types::FieldElement,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};

use crate::{
    price::utils::{get_pragma_price, PathParams, QueryParams},
    types::SatoruAction,
    utils::get_token_name_from_address,
};

abigen!(
    OrderHandler,
    "./resources/satoru_DepositHandler.contract_class.json",
);

abigen!(
    DataStore,
    "./resources/satoru_DataStore.contract_class.json",
    type_aliases {
        satoru::deposit::deposit::OrderType as Order_;
        satoru::data::data_store::DataStore::Event as Event_;
        satoru::deposit::deposit::DecreasePositionSwapType as Decrease_;
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

pub async fn handle_deposit(
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
    deposit: SatoruAction,
) {
    let deposit_handler_address =
        env::var("DEPOSIT_HANDLER").expect("DEPOSIT_HANDLER env variable not set");
    let deposit_handler = OrderHandler::new(
        FieldElement::from_hex_be(&deposit_handler_address)
            .expect("Conversion error: deposit_handler_address"),
        account.clone(),
    );

    let data_store_address = env::var("DATA_STORE").expect("DATA_STORE env variable not set");
    let data_store = DataStore::new(
        FieldElement::from_hex_be(&data_store_address)
            .expect("Conversion error: data_store_address"),
        account.clone(),
    );

    let oracle_address = env::var("ORACLE").expect("ORACLE env variable not set");
    let oracle = Oracle::new(
        FieldElement::from_hex_be(&oracle_address).expect("Conversion error: oracle_address"),
        account.clone(),
    );

    let market = data_store
        .get_market(&ContractAddress::from(
            FieldElement::from_hex_be(&deposit.key).expect("Cannot convert string to felt"),
        ))
        .call()
        .await
        .expect("Could not get market");

    let path = PathParams {
        base: get_token_name_from_address(market.long_token).to_owned(),
        quote: "usd".to_owned(),
        timestamp: deposit.block_number, // TODO replace by timestamp
        interval: "1min".to_owned(),
    };

    let query = QueryParams {
        routing: false,
        aggregation: "median".to_owned(),
    };

    let price_info = get_pragma_price(path, query)
        .await
        .expect("Price did not get returned");

    // Set token price before tx.
    let price_uint = u128::from_str_radix(price_info.price.as_str(), 16)
        .expect("Could not convert hex price to uint");

    let set_price_call = oracle.set_primary_price_getcall(
        &market.long_token,
        &U256 {
            low: price_uint,
            high: 0,
        },
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

    let execute_deposit_call = deposit_handler.execute_deposit_getcall(
        &FieldElement::from_hex_be(&deposit.key).expect("Cannot convert string to felt"),
        &set_prices_params,
    );

    let deposit_execution_multicall = account
        .execute(vec![set_price_call, execute_deposit_call])
        .send()
        .await
        .expect("Order execution multicall failed");
    // TODO: poll transaction status
}
