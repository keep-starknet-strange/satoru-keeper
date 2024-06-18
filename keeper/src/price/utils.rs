use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use std::env;

use super::error::PragmaAPIError;

#[derive(Deserialize)]
struct PathParams {
    base: String,
    quote: String,
    timestamp: i64,
    interval: String,
}

#[derive(Deserialize)]
struct QueryParams {
    routing: bool,
    aggregation: String,
}

#[derive(Deserialize, Debug)]
struct PriceInfo {
    decimals: u64,
    num_sources_aggregated: u64,
    pair_id: String,
    price: String,
    timestamp: u64,
}

pub async fn get_pragma_price(path: PathParams, query: QueryParams) -> Result<PriceInfo, PragmaAPIError> {
    let api_url = format!(
        "https://api.dev.pragma.build/node/v1/data/{}/{}?interval={}&aggregation={}&timestamp={}",
        path.base, path.quote, path.interval, query.aggregation, path.timestamp
    );
    match fetch_data(&api_url).await {
        Ok(price_info) => Ok(price_info),
        Err(err) => Err(err),
    }
}

async fn fetch_data(url: &str) -> Result<PriceInfo, PragmaAPIError> {
    dotenv().ok();

    let client = reqwest::Client::new();
    let api_key = env::var("PRAGMA_API_KEY").or_else(|e| Err(PragmaAPIError::APIKeyNotSet()))?;
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
            timestamp: 1711110660,
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
                assert_eq!(price_info.timestamp, 1711110660000);
                assert_eq!(price_info.num_sources_aggregated, 4);
            }
            Err(err) => {
                println!("{:?}", err);
                match err {
                    PragmaAPIError::APIKeyNotSet() => {}
                    PragmaAPIError::Unknown(_) => {}
                    _ => {
                        panic!("Handler failed.")
                    }
                }
            }
        }
    }
}
