use std::sync::Arc;

use sqlx::PgPool;
use starknet::{
    accounts::{ConnectedAccount, SingleOwnerAccount},
    core::types::{BlockId, BlockTag, MaybePendingBlockWithTxHashes},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
    signers::LocalWallet,
};

use crate::{
    liquidation::utils::is_liquidatable_call,
    price::utils::get_market_prices,
    query::get_market,
    types::{MarketPrices, Position},
};

pub async fn get_liquidatable_positions(
    pool: &PgPool,
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
) -> Result<Vec<Position>, sqlx::Error> {
    let positions: Vec<Position> = sqlx::query_as("SELECT * FROM positions")
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

    let mut liquidatable_positions: Vec<Position> = Vec::new();
    for position in positions {
        let market = get_market(position.clone().market.0.to_string(), pool)
            .await
            .expect("Could not get market");
        let market_prices: MarketPrices = get_market_prices(market.clone(), timestamp.to_string())
            .await
            .expect(format!("Could not get market prices for market: :{:?}", market).as_str());
        let (is_liquidatable, _reason) = is_liquidatable_call(
            Arc::clone(&account),
            position.clone(),
            market,
            market_prices,
            true,
        )
        .await
        .expect("is_liquidatable_call fail");

        log::info!("Position {:?} is liquidatable", position.key);
        if is_liquidatable {
            liquidatable_positions.push(position);
        }
    }

    Ok(liquidatable_positions)
}
