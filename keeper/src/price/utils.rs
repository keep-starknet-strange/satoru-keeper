use cainome::cairo_serde::U256;
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use std::env;

use super::error::PragmaAPIError;

use crate::{
    liquidation::utils::{Market, MarketPrices, Price},
    trade::utils::get_token_name_from_address,
};

#[derive(Deserialize, Clone)]
pub struct PathParams {
    pub base: String,
    pub quote: String,
    pub timestamp: String,
    pub interval: String,
}

#[derive(Deserialize, Clone)]
pub struct QueryParams {
    pub routing: bool,
    pub aggregation: String,
}

// Price info returned from pragma API
#[derive(Deserialize, Debug)]
pub struct PriceInfo {
    pub decimals: u64,
    pub num_sources_aggregated: u64,
    pub pair_id: String,
    pub price: String,
    pub timestamp: u64,
}

// Get price from pragma api for a specific token
pub async fn get_pragma_price(
    path: PathParams,
    query: QueryParams,
) -> Result<PriceInfo, PragmaAPIError> {
    let api_url = format!(
        "https://api.dev.pragma.build/node/v1/data/{}/{}?interval={}&aggregation={}&timestamp={}",
        path.base, path.quote, path.interval, query.aggregation, path.timestamp
    );
    match fetch_data(&api_url).await {
        Ok(price_info) => Ok(price_info),
        Err(err) => Err(err),
    }
}

// Fetch prices to build MarketPrices from Market
pub async fn get_market_prices(
    market: Market,
    timestamp: String,
) -> Result<MarketPrices, PragmaAPIError> {
    let mut path = PathParams {
        base: get_token_name_from_address(market.long_token).to_owned(),
        quote: "usd".to_owned(),
        timestamp,
        interval: "1min".to_owned(),
    };

    let query = QueryParams {
        routing: false,
        aggregation: "median".to_owned(),
    };
    let long_token_price_info = get_pragma_price(path.clone(), query.clone())
        .await
        .expect("get_pragma_price failed on long_token");

    path.base = get_token_name_from_address(market.short_token);
    let short_token_price_info = get_pragma_price(path.clone(), query.clone())
        .await
        .expect("get_pragma_price failed on short_token");

    path.base = get_token_name_from_address(market.index_token);
    let index_token_price_info = get_pragma_price(path, query)
        .await
        .expect("get_pragma_price failed on index_token");

    let index_token_price = u128::from_str_radix(
        index_token_price_info
            .price
            .as_str()
            .trim_start_matches("0x"),
        16,
    )
    .expect("Could not convert index token hex price to uint");
    let long_token_price = u128::from_str_radix(
        long_token_price_info
            .price
            .as_str()
            .trim_start_matches("0x"),
        16,
    )
    .expect("Could not convert long token hex price to uint");
    let short_token_price = u128::from_str_radix(
        short_token_price_info
            .price
            .as_str()
            .trim_start_matches("0x"),
        16,
    )
    .expect("Could not convert short token hex price to uint");
    Ok(MarketPrices {
        index_token_price: Price {
            min: U256 {
                low: index_token_price,
                high: 0,
            },
            max: U256 {
                low: index_token_price,
                high: 0,
            },
        },
        long_token_price: Price {
            min: U256 {
                low: long_token_price,
                high: 0,
            },
            max: U256 {
                low: long_token_price,
                high: 0,
            },
        },
        short_token_price: Price {
            min: U256 {
                low: short_token_price,
                high: 0,
            },
            max: U256 {
                low: short_token_price,
                high: 0,
            },
        },
    })
}

async fn fetch_data(url: &str) -> Result<PriceInfo, PragmaAPIError> {
    dotenv().ok();

    let client = reqwest::Client::new();
    let api_key = env::var("PRAGMA_API_KEY").map_err(|_e| PragmaAPIError::APIKeyNotSet())?;
    if api_key.is_empty() {
        return Err(PragmaAPIError::APIKeyNotSet());
    }

    let response = client
        .get(url)
        .header("x-api-key", api_key)
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<PriceInfo>().await {
            Ok(parsed) => Ok(parsed),
            Err(err) => Err(PragmaAPIError::JsonParsing(err)),
        },
        reqwest::StatusCode::UNAUTHORIZED => Err(PragmaAPIError::UnauthorizedAccess()),
        other => Err(PragmaAPIError::Unknown(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_success() {
        let path = PathParams {
            base: "eth".to_owned(),
            quote: "usd".to_owned(),
            timestamp: "1711110660".to_owned(),
            interval: "1min".to_owned(),
        };
        let query = QueryParams {
            routing: false,
            aggregation: "median".to_owned(),
        };

        let price_info = get_pragma_price(path, query).await;
        match price_info {
            Ok(price_info) => {
                assert_eq!(price_info.decimals, 8);
                assert_eq!(price_info.pair_id, "ETH/USD");
                assert_eq!(price_info.price, "0x4f8b06508e");
                assert_eq!(price_info.timestamp, 1711110660000.to_owned());
                assert_eq!(price_info.num_sources_aggregated, 4);
            }
            Err(err) => match err {
                PragmaAPIError::APIKeyNotSet() => {}
                PragmaAPIError::Unknown(_) => {}
                _ => {
                    panic!("Handler failed.")
                }
            },
        }
    }
}
