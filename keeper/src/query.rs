use sqlx::{Error, PgPool};

use crate::liquidation::utils::Market;

pub async fn get_market(market_token: String, pool: &PgPool) -> Result<Market, Error> {
    let market: Result<Market, Error> =
        sqlx::query_as("SELECT * FROM markets WHERE market_token = $1")
            .bind(market_token)
            .fetch_one(pool)
            .await;
    return market;
}
