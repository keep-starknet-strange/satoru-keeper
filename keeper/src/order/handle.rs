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

use crate::{
    price::utils::{get_pragma_price, PathParams, QueryParams},
    types::SatoruAction,
    utils::get_token_name_from_address,
};

abigen!(
    OrderHandler,
    "./resources/satoru_OrderHandler.contract_class.json",
);

abigen!(
    DataStore,
    "./resources/satoru_DataStore.contract_class.json",
    type_aliases {
        satoru::order::order::OrderType as Order_;
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
    }
);

pub async fn handle_order(
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
    order: SatoruAction,
) {
    let set_price_call = get_set_primary_price_call(order.clone(), account.clone()).await;

    let execute_order_call = get_execute_order_call(order, account.clone());

    let order_execution_multicall = account
        .execute(vec![set_price_call, execute_order_call])
        .send()
        .await
        .expect("Order execution multicall failed");
    // TODO: poll transaction status
}

async fn get_set_primary_price_call(
    order: SatoruAction,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Call {
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
            FieldElement::from_hex_be(&order.key).expect("Cannot convert string to felt"),
        ))
        .call()
        .await
        .expect("Could not get market");

    let price = price_setup(order.block_number, market.clone()).await; // TODO use timestamp instead of block number

    oracle.set_primary_price_getcall(&market.long_token, &price)
}

async fn price_setup(timestamp: u64, market: Market) -> U256 {
    let path = PathParams {
        base: get_token_name_from_address(market.long_token).to_owned(),
        quote: "usd".to_owned(),
        timestamp: timestamp,
        interval: "1min".to_owned(),
    };

    let query = QueryParams {
        routing: false,
        aggregation: "median".to_owned(),
    };

    let price_info = get_pragma_price(path, query)
        .await
        .expect("Price did not get returned");

    let price_uint = u128::from_str_radix(price_info.price.as_str().trim_start_matches("0x"), 16)
        .expect("Could not convert hex price to uint");

    U256 {
        low: price_uint,
        high: 0,
    }
}

fn get_execute_order_call(
    order: SatoruAction,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Call {
    let order_handler_address =
        env::var("ORDER_HANDLER").expect("ORDER_HANDLER env variable not set");
    let order_handler = OrderHandler::new(
        FieldElement::from_hex_be(&order_handler_address)
            .expect("Conversion error: order_handler_address"),
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

    order_handler.execute_order_getcall(
        &FieldElement::from_hex_be(&order.key).expect("Cannot convert string to felt"),
        &set_prices_params,
    )
}

#[cfg(test)]
mod tests {
    use crate::price::error::PragmaAPIError;

    use super::*;

    #[tokio::test]
    async fn test_price_setup() {
        let api_key = env::var("PRAGMA_API_KEY").or_else(|e| Err(PragmaAPIError::APIKeyNotSet()));
        match api_key {
            Ok(_) => {
                let market = Market {
                    market_token: ContractAddress::from(
                        FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
                    ),
                    index_token: ContractAddress::from(
                        FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
                    ),
                    long_token: ContractAddress::from(
                        FieldElement::from_hex_be(
                            "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                        )
                        .expect("Cannot convert string to felt"),
                    ),
                    short_token: ContractAddress::from(
                        FieldElement::from_hex_be(
                            "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
                        )
                        .expect("Cannot convert string to felt"),
                    ),
                };

                let price = price_setup(1711110660, market).await;

                assert!(price > U256 { low: 3000, high: 0 })
            }
            Err(_) => {}
        }
    }
}
