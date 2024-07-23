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
    #[serde_as(as = "DisplayFromStr")]
    pub size_in_usd: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub size_in_tokens: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub collateral_amount: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub borrowing_factor: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_fee_amount_per_size: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub long_token_claimable_funding_amount_per_size: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub short_token_claimable_funding_amount_per_size: BigDecimal,
    pub increased_at_block: i64,
    pub decreased_at_block: i64,
    pub is_long: bool,
}

#[post("/positions")]
async fn create_position(
    pool: web::Data<PgPool>,
    position: web::Json<Position>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO position (
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
        FROM position
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
                size_in_usd: record.size_in_usd.clone().unwrap(),
                size_in_tokens: record.size_in_tokens.clone().unwrap(),
                collateral_amount: record.collateral_amount.clone().unwrap(),
                borrowing_factor: record.borrowing_factor.clone().unwrap(),
                funding_fee_amount_per_size: record.funding_fee_amount_per_size.clone().unwrap(),
                long_token_claimable_funding_amount_per_size: record.long_token_claimable_funding_amount_per_size.clone().unwrap(),
                short_token_claimable_funding_amount_per_size: record.short_token_claimable_funding_amount_per_size.clone().unwrap(),
                increased_at_block: record.increased_at_block.unwrap(),
                decreased_at_block: record.decreased_at_block.unwrap(),
                is_long: record.is_long.unwrap(),
            }).collect();

            HttpResponse::Ok().json(positions)
        },
        Err(err) => {
            println!("Failed to execute query: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
struct Order {
    pub block_number: i64,
    pub timestamp: Option<String>,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub order_type: Option<String>,
    pub decrease_position_swap_type: Option<String>,
    pub account: Option<String>,
    pub receiver: Option<String>,
    pub callback_contract: Option<String>,
    pub ui_fee_receiver: Option<String>,
    pub market: Option<String>,
    pub initial_collateral_token: Option<String>,
    pub swap_path: Option<Vec<String>>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub size_delta_usd: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub initial_collateral_delta_amount: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub trigger_price: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub acceptable_price: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub execution_fee: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub callback_gas_limit: Option<BigDecimal>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub min_output_amount: Option<BigDecimal>,
    pub updated_at_block: Option<i64>,
    pub is_long: Option<bool>,
    pub is_frozen: Option<bool>,
}

#[post("/orders")]
async fn create_order(
    pool: web::Data<PgPool>,
    order: web::Json<Order>,
) -> impl Responder {
    let swap_path_json = order.swap_path.as_ref().map(|path| serde_json::to_string(path).unwrap());

    let result = sqlx::query!(
        r#"
        INSERT INTO orders (
            block_number, time_stamp, transaction_hash, key, order_type,
            decrease_position_swap_type, account, receiver, callback_contract,
            ui_fee_receiver, market, initial_collateral_token, swap_path,
            size_delta_usd, initial_collateral_delta_amount, trigger_price,
            acceptable_price, execution_fee, callback_gas_limit, min_output_amount,
            updated_at_block, is_long, is_frozen
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
        "#,
        order.block_number,
        order.timestamp,
        order.transaction_hash,
        order.key,
        order.order_type,
        order.decrease_position_swap_type,
        order.account,
        order.receiver,
        order.callback_contract,
        order.ui_fee_receiver,
        order.market,
        order.initial_collateral_token,
        swap_path_json,
        order.size_delta_usd,
        order.initial_collateral_delta_amount,
        order.trigger_price,
        order.acceptable_price,
        order.execution_fee,
        order.callback_gas_limit,
        order.min_output_amount,
        order.updated_at_block,
        order.is_long,
        order.is_frozen
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

#[get("/orders")]
async fn get_orders(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query!(
        r#"
        SELECT block_number, time_stamp, transaction_hash, key, order_type,
               decrease_position_swap_type, account, receiver, callback_contract,
               ui_fee_receiver, market, initial_collateral_token, swap_path,
               size_delta_usd, initial_collateral_delta_amount, trigger_price,
               acceptable_price, execution_fee, callback_gas_limit, min_output_amount,
               updated_at_block, is_long, is_frozen
        FROM orders
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(records) => {
            let orders: Vec<Order> = records.iter().map(|record| Order {
                block_number: record.block_number,
                timestamp: record.time_stamp.clone(),
                transaction_hash: record.transaction_hash.clone(),
                key: record.key.clone(),
                order_type: record.order_type.clone(),
                decrease_position_swap_type: record.decrease_position_swap_type.clone(),
                account: record.account.clone(),
                receiver: record.receiver.clone(),
                callback_contract: record.callback_contract.clone(),
                ui_fee_receiver: record.ui_fee_receiver.clone(),
                market: record.market.clone(),
                initial_collateral_token: record.initial_collateral_token.clone(),
                swap_path: record.swap_path.clone().map(|s| vec![s]),
                size_delta_usd: record.size_delta_usd.clone(),
                initial_collateral_delta_amount: record.initial_collateral_delta_amount.clone(),
                trigger_price: record.trigger_price.clone(),
                acceptable_price: record.acceptable_price.clone(),
                execution_fee: record.execution_fee.clone(),
                callback_gas_limit: record.callback_gas_limit.clone(),
                min_output_amount: record.min_output_amount.clone(),
                updated_at_block: record.updated_at_block,
                is_long: record.is_long,
                is_frozen: record.is_frozen,
            }).collect();

            HttpResponse::Ok().json(orders)
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
    cfg.service(create_order);
    cfg.service(get_orders);
}
