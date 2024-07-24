use std::sync::Arc;

use cainome::cairo_serde::U256;
use sqlx::PgPool;
use starknet::{
    accounts::{ConnectedAccount, SingleOwnerAccount},
    core::types::{BlockId, BlockTag, MaybePendingBlockWithTxHashes},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
    signers::LocalWallet,
};

use crate::{
    price::utils::get_market_prices,
    query::get_market,
    types::{MarketPrices, SatoruAction},
};

pub async fn get_triggerable_orders(
    pool: &PgPool,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Result<Vec<SatoruAction>, sqlx::Error> {
    let orders: Vec<SatoruAction> = sqlx::query_as("SELECT * FROM orders")
        .fetch_all(pool)
        .await?;

    let block = Arc::clone(&account)
        .provider()
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .expect("Could not fetch latest block");
    let timestamp = match block {
        MaybePendingBlockWithTxHashes::Block(block) => block.timestamp,
        MaybePendingBlockWithTxHashes::PendingBlock(block) => block.timestamp,
    };

    let mut triggerable_positions: Vec<SatoruAction> = Vec::new();
    for order in orders {
        if is_limit_order(
            order
                .clone()
                .order_type
                .expect("Could not unwrap order_type"),
        ) {
            let market = get_market(order.clone().market, pool)
                .await
                .expect("Could not get market");
            let market_prices: MarketPrices =
                get_market_prices(market.clone(), timestamp.to_string())
                    .await
                    .expect(
                        format!("Could not get market prices for market: :{:?}", market).as_str(),
                    );
            if should_trigger(order.clone(), market_prices) {
                triggerable_positions.push(order.clone());
                log::info!("Position {:?} is liquidatable", order.clone().key);
            }
        }
    }

    Ok(triggerable_positions)
}

pub fn is_limit_order(order_type: String) -> bool {
    match order_type.as_str() {
        "LimitSwap" => true,
        "LimitIncrease" => true,
        "LimitDecrease" => true,
        "StopLossDecrease" => true,
        _ => false,
    }
}

pub fn should_trigger(order: SatoruAction, market_prices: MarketPrices) -> bool {
    let trigger_price = U256 {
        low: order.trigger_price.unwrap(),
        high: 0,
    };
    let is_long = order.is_long.unwrap();
    let min_price = market_prices.index_token_price.min;
    let max_price = market_prices.index_token_price.max;
    match order.order_type.unwrap().as_str() {
        "LimitSwap" => {
            // TODO
            true
        }
        "LimitIncrease" => {
            if is_long {
                max_price <= trigger_price
            } else {
                min_price >= trigger_price
            }
        }
        "LimitDecrease" => {
            if is_long {
                min_price >= trigger_price
            } else {
                max_price <= trigger_price
            }
        }
        "StopLossDecrease" => {
            if is_long {
                min_price <= trigger_price
            } else {
                max_price >= trigger_price
            }
        }
        _ => false,
    }
}
