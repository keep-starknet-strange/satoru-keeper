use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct Position {
    key: String,
    account: String,
    market: String,
    collateral_token: String,
    size_in_usd: Option<f64>,
    size_in_tokens: Option<f64>,
    collateral_amount: Option<f64>,
    borrowing_factor: Option<f64>,
    funding_fee_amount_per_size: Option<f64>,
    long_token_claimable_funding_amount_per_size: Option<f64>,
    short_token_claimable_funding_amount_per_size: Option<f64>,
    increased_at_block: Option<i64>,
    decreased_at_block: Option<i64>,
    is_long: Option<bool>,
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
        Ok(positions) => HttpResponse::Ok().json(positions),
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
