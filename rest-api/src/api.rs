use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use bigdecimal::BigDecimal;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
struct Position {
    pub key: String,
    pub account: String,
    pub market: String,
    pub collateral_token: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub size_in_usd: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub size_in_tokens: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub collateral_amount: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub borrowing_factor: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub funding_fee_amount_per_size: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub long_token_claimable_funding_amount_per_size: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub short_token_claimable_funding_amount_per_size: Option<BigDecimal>,
    pub increased_at_block: Option<i64>,
    pub decreased_at_block: Option<i64>,
    pub is_long: Option<bool>,
}


#[post("/positions")]
async fn create_position(
    pool: web::Data<PgPool>,
    position: web::Json<Position>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO position_increase (
            key, account, market, collateral_token, size_in_usd, size_in_tokens,
            collateral_amount, borrowing_factor, funding_fee_amount_per_size,
            long_token_claimable_funding_amount_per_size, short_token_claimable_funding_amount_per_size,
            increased_at_block, decreased_at_block, is_long
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "#,
        position.key,
        position.account,
        position.market,
        position.collateral_token,
        position.size_in_usd,
        position.size_in_tokens,
        position.collateral_amount,
        position.borrowing_factor,
        position.funding_fee_amount_per_size,
        position.long_token_claimable_funding_amount_per_size,
        position.short_token_claimable_funding_amount_per_size,
        position.increased_at_block,
        position.decreased_at_block,
        position.is_long
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            println!("Failed to execute query: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/positions")]
async fn get_positions(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query!(
        r#"
        SELECT key, account, market, collateral_token, size_in_usd, size_in_tokens,
               collateral_amount, borrowing_factor, funding_fee_amount_per_size,
               long_token_claimable_funding_amount_per_size, short_token_claimable_funding_amount_per_size,
               increased_at_block, decreased_at_block, is_long
        FROM position_increase
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(records) => {
            let positions: Vec<Position> = records.iter().map(|record| Position {
                key: record.key.clone(),
                account: record.account.clone(),
                market: record.market.clone(),
                collateral_token: record.collateral_token.clone(),
                size_in_usd: record.size_in_usd.clone(),
                size_in_tokens: record.size_in_tokens.clone(),
                collateral_amount: record.collateral_amount.clone(),
                borrowing_factor: record.borrowing_factor.clone(),
                funding_fee_amount_per_size: record.funding_fee_amount_per_size.clone(),
                long_token_claimable_funding_amount_per_size: record.long_token_claimable_funding_amount_per_size.clone(),
                short_token_claimable_funding_amount_per_size: record.short_token_claimable_funding_amount_per_size.clone(),
                increased_at_block: record.increased_at_block,
                decreased_at_block: record.decreased_at_block,
                is_long: record.is_long,
            }).collect();

            HttpResponse::Ok().json(positions)
        },
        Err(err) => {
            println!("Failed to execute query: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_position);
    cfg.service(get_positions);
}
