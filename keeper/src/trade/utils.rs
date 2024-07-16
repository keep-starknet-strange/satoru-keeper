use std::{env, sync::Arc};

use cainome::cairo_serde::{ContractAddress, U256};
use starknet::{
    accounts::{Call, SingleOwnerAccount},
    core::types::FieldElement,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};

use crate::{
    price::utils::{get_pragma_price, PathParams, QueryParams},
    trade::order::handle::{DataStore, Market, Oracle},
    types::SatoruAction,
};

use super::{error::TradeError, order::handle::Price_};

pub fn get_token_name_from_address(token_address: ContractAddress) -> String {
    match token_address {
        x if x
            == ContractAddress::from(
                FieldElement::from_hex_be(
                    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .expect("Cannot convert string to felt"),
            ) =>
        {
            "eth".to_owned()
        }
        x if x
            == ContractAddress::from(
                FieldElement::from_hex_be(
                    "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
                )
                .expect("Cannot convert string to felt"),
            ) =>
        {
            "usdc".to_owned()
        }
        _ => "eth".to_owned(), // TODO throw error on this
    }
}

pub async fn get_set_primary_price_call(
    trade: SatoruAction,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Result<Call, TradeError> {
    let data_store_address =
        env::var("DATA_STORE").map_err(|_e| TradeError::EnvVarNotSet("DATA_STORE".to_owned()))?;
    let data_store = DataStore::new(
        FieldElement::from_hex_be(&data_store_address)
            .map_err(|e| TradeError::ConversionError("data_store_address".to_owned()))?,
        account.clone(),
    );

    let oracle_address = env::var("ORACLE").map_err(|_e| TradeError::EnvVarNotSet("ORACLE".to_owned()))?;
    let oracle = Oracle::new(
        FieldElement::from_hex_be(&oracle_address).map_err(|_e| TradeError::ConversionError("oracle_address".to_owned()))?,
        account.clone(),
    );

    let market = data_store
        .get_market(&ContractAddress::from(
            FieldElement::from_hex_be(&trade.key).expect("Cannot convert string to felt"),
        ))
        .call()
        .await
        .map_err(|_e| TradeError::SmartContractError("Could not get market".to_owned()))?;

    let price = price_setup(trade.time_stamp, market.clone()).await?;

    Ok(oracle.set_primary_price_getcall(
        &market.long_token,
        &Price_ {
            min: price,
            max: price,
        },
    ))
}

pub async fn price_setup(timestamp: String, market: Market) -> Result<U256, TradeError> {
    let path = PathParams {
        base: get_token_name_from_address(market.long_token).to_owned(),
        quote: "usd".to_owned(),
        timestamp,
        interval: "1min".to_owned(),
    };

    let query = QueryParams {
        routing: false,
        aggregation: "median".to_owned(),
    };

    let price_info = get_pragma_price(path, query)
        .await
        .map_err(|e| TradeError::PragmaAPIError(format!("Price did not get returned: {}", e)))?;

    let price_uint =
        u128::from_str_radix(price_info.price.as_str().trim_start_matches("0x"), 16)
            .map_err(|e| TradeError::ConversionError("Could not convert hex price to uint".to_owned()))?;

    Ok(U256 {
        low: price_uint,
        high: 0,
    })
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

                let price = price_setup("1711110660".to_owned(), market).await.unwrap();

                assert!(price > U256 { low: 3000, high: 0 })
            }
            Err(_) => {}
        }
    }
}
