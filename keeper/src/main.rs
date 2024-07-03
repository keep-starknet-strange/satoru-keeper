use dotenv::dotenv;
use std::{env, sync::Arc};

use keeper_satoru::{
    error::KeeperError,
    listen_db::start_listening,
    trade::{
        deposit::handle::handle_deposit, order::handle::handle_order,
        withdrawal::handle::handle_withdrawal,
    },
    types::{ActionType, Payload},
};
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, types::FieldElement},
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
    signers::{LocalWallet, SigningKey},
};
use tokio::task;
use url::Url;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    dotenv().ok();

    match args[1].as_str() {
        "liquidation" => {}
        "execution" => execution_mode().await,
        _ => {
            panic!("Wrong launch parameter")
        }
    }
}

async fn execution_mode() {
    let pool = sqlx::PgPool::connect("postgres://postgres:123@localhost:5432/zohal")
        .await
        .unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse(
            &env::var("RPC_URL")
                .map_err(|_e| KeeperError::RpcUrlNotSet())
                .unwrap(),
        )
        .map_err(|e| KeeperError::ProviderUrlError(format!("invalid rpc url: {}", e)))
        .unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            &env::var("PRIVATE_KEY")
                .map_err(|_e| KeeperError::PrivateKeyNotSet())
                .unwrap(),
        )
        .expect("Could not convert private key to felt."),
    ));

    let account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> =
        SingleOwnerAccount::new(
            provider,
            signer,
            FieldElement::from_hex_be(
                &env::var("PUBLIC_KEY")
                    .map_err(|_e| KeeperError::PublicKeyNotSet())
                    .unwrap(),
            )
            .expect("Could not convert private key to felt."),
            chain_id::SEPOLIA,
            ExecutionEncoding::New,
        );

    let account_ref = Arc::new(account);
    let channels: Vec<&str> = vec!["orders_update", "deposits_update", "withdrawals_update"];
    let call_back = |payload: Payload| {
        let account_ref = Arc::clone(&account_ref);
        task::spawn(async move {
            println!("{:?}", payload.row_data);
            let account_ref = Arc::clone(&account_ref);
            match payload.table.as_str() {
                "orders" => match payload.action_type {
                    ActionType::INSERT => {
                        handle_order(Arc::clone(&account_ref), payload.row_data).await;
                    }
                    ActionType::UPDATE => {}
                },
                "deposits" => match payload.action_type {
                    ActionType::INSERT => {
                        handle_deposit(Arc::clone(&account_ref), payload.row_data).await;
                    }
                    ActionType::UPDATE => {}
                },
                "withdrawals" => match payload.action_type {
                    ActionType::INSERT => {
                        handle_withdrawal(Arc::clone(&account_ref), payload.row_data).await;
                    }
                    ActionType::UPDATE => {}
                },
                &_ => {}
            }
        })
    };
    println!("Keeper connected to DB and listening...");

    let _ = start_listening(&pool, channels, call_back).await;
}
