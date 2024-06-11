use keeper_satoru::{listen_db::start_listening, types::{ActionType, Payload}};

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect(
        "postgres://postgres:123@localhost:5432/zohal",
    )
    .await
    .unwrap();

    let channels: Vec<&str> = vec!["orders_update", "deposits_update", "withdrawals_update"];

    let call_back = |payload: Payload| {
        println!("{:?}", payload.row_data);
        match payload.table.as_str() {
            "orders" => {
                match payload.action_type {
                    ActionType::INSERT => {
                    }
                    ActionType::UPDATE => {
                    }
                }
            }
            "deposits" => {
                match payload.action_type {
                    ActionType::INSERT => {
                    }
                    ActionType::UPDATE => {
                    }
                }
            }
            "withdrawals" => {
                match payload.action_type {
                    ActionType::INSERT => {
                    }
                    ActionType::UPDATE => {
                    }
                }
            }
            &_ => {

            }
        }
    };
    println!("Keeper connected to DB and listening...");

    let _ = start_listening(&pool, channels, call_back).await;
}