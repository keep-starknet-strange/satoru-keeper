use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, EventFilter},
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    providers::Provider,
};
use tokio_postgres::Client;
use hex;

use crate::events::types::{Order, Deposit, Withdrawal};

pub async fn fetch_and_process_events(
    provider: &JsonRpcClient<HttpTransport>,
    client: &Client,
) -> Result<(), tokio_postgres::Error> {
    let order_created_key = FieldElement::from_hex_be("03427759bfd3b941f14e687e129519da3c9b0046c5b9aaa290bb1dede63753b3").unwrap();
    let deposit_created_key = FieldElement::from_hex_be("00ee02d31cafad9001fbdc4dd5cf4957e152a372530316a7d856401e4c5d74bd").unwrap();
    let withdrawal_created_key = FieldElement::from_hex_be("02021e2242f6c652ae824bc1428ee0fe7e8771a27295b9450792445dc456e37d").unwrap();

    let event_filter = EventFilter {
        from_block: Some(BlockId::Number(64539)),
        to_block: Some(BlockId::Tag(BlockTag::Latest)),
        address: FieldElement::from_hex_be("0x2cf721c0387704095d6b2205b46e17d7768fa55c2f1a1087425b877b72937db").ok(),
        keys: Some(vec![vec![order_created_key, deposit_created_key, withdrawal_created_key]]),
    };

    match provider.get_events(event_filter, None, 100).await {
        Ok(events_page) => {
            for event in events_page.events {
                println!("Event found: {:?}", event);
                let block_number = event.block_number as i64;
                let transaction_hash_bytes = event.transaction_hash.to_bytes_be();
                let transaction_hash = hex::encode(transaction_hash_bytes);
                let key = event.keys.first().map(|k| hex::encode(k.to_bytes_be()));
                let data = event.data.iter()
                    .map(|fe| hex::encode(fe.to_bytes_be()))
                    .collect::<Vec<_>>()
                    .join(",");

                if event.keys.contains(&order_created_key) {
                    let order = Order {
                        block_number,
                        transaction_hash: transaction_hash.clone(),
                        key: key.clone(),
                        order_type: None,
                        decrease_position_swap_type: None,
                        account: None,
                        receiver: None,
                        callback_contract: None,
                        ui_fee_receiver: None,
                        market: None,
                        initial_collateral_token: None,
                        swap_path: None,
                        size_delta_usd: None,
                        initial_collateral_delta_amount: None,
                        trigger_price: None,
                        acceptable_price: None,
                        execution_fee: None,
                        callback_gas_limit: None,
                        min_output_amount: None,
                        updated_at_block: None,
                        is_long: None,
                        is_frozen: None,
                    };

                    insert_order(&client, &order).await?;
                } else if event.keys.contains(&deposit_created_key) {
                    let deposit = Deposit {
                        block_number,
                        transaction_hash: transaction_hash.clone(),
                        key: key.clone(),
                        account: None,
                        receiver: None,
                        callback_contract: None,
                        market: None,
                        initial_long_token: None,
                        initial_short_token: None,
                        long_token_swap_path: None,
                        short_token_swap_path: None,
                        initial_long_token_amount: None,
                        initial_short_token_amount: None,
                        min_market_tokens: None,
                        updated_at_block: None,
                        execution_fee: None,
                        callback_gas_limit: None,
                    };

                    insert_deposit(&client, &deposit).await?;
                } else if event.keys.contains(&withdrawal_created_key) {
                    let withdrawal = Withdrawal {
                        block_number,
                        transaction_hash: transaction_hash.clone(),
                        key: key.clone(),
                        account: None,
                        receiver: None,
                        callback_contract: None,
                        market: None,
                        long_token_swap_path: None,
                        short_token_swap_path: None,
                        market_token_amount: None,
                        min_long_token_amount: None,
                        min_short_token_amount: None,
                        updated_at_block: None,
                        execution_fee: None,
                        callback_gas_limit: None,
                    };

                    insert_withdrawal(&client, &withdrawal).await?;
                } else {
                    println!("Unknown event type: {:?}", event);
                }
            }
        },
        Err(e) => {
            println!("Failed to fetch events: {:?}", e);
        }
    }
    Ok(())
}

async fn insert_order(client: &Client, order: &Order) -> Result<u64, tokio_postgres::Error> {
    client.execute(
        "INSERT INTO orders (
            block_number, transaction_hash, key, order_type, decrease_position_swap_type, account,
            receiver, callback_contract, ui_fee_receiver, market, initial_collateral_token, swap_path,
            size_delta_usd, initial_collateral_delta_amount, trigger_price, acceptable_price,
            execution_fee, callback_gas_limit, min_output_amount, updated_at_block, is_long, is_frozen
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12,
            $13, $14, $15, $16,
            $17, $18, $19, $20, $21, $22
        )",
        &[
            &order.block_number, &order.transaction_hash, &order.key, &order.order_type, 
            &order.decrease_position_swap_type, &order.account, &order.receiver, 
            &order.callback_contract, &order.ui_fee_receiver, &order.market, 
            &order.initial_collateral_token, &order.swap_path, &order.size_delta_usd, 
            &order.initial_collateral_delta_amount, &order.trigger_price, &order.acceptable_price, 
            &order.execution_fee, &order.callback_gas_limit, &order.min_output_amount, 
            &order.updated_at_block, &order.is_long, &order.is_frozen
        ],
    ).await
}

async fn insert_deposit(client: &Client, deposit: &Deposit) -> Result<u64, tokio_postgres::Error> {
    client.execute(
        "INSERT INTO deposits (
            block_number, transaction_hash, key, account, receiver, callback_contract,
            market, initial_long_token, initial_short_token, long_token_swap_path, short_token_swap_path,
            initial_long_token_amount, initial_short_token_amount, min_market_tokens, updated_at_block,
            execution_fee, callback_gas_limit
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11,
            $12, $13, $14, $15,
            $16, $17
        )",
        &[
            &deposit.block_number, &deposit.transaction_hash, &deposit.key, &deposit.account, 
            &deposit.receiver, &deposit.callback_contract, &deposit.market, &deposit.initial_long_token, 
            &deposit.initial_short_token, &deposit.long_token_swap_path, &deposit.short_token_swap_path, 
            &deposit.initial_long_token_amount, &deposit.initial_short_token_amount, &deposit.min_market_tokens, 
            &deposit.updated_at_block, &deposit.execution_fee, &deposit.callback_gas_limit
        ],
    ).await
}

async fn insert_withdrawal(client: &Client, withdrawal: &Withdrawal) -> Result<u64, tokio_postgres::Error> {
    client.execute(
        "INSERT INTO withdrawals (
            block_number, transaction_hash, key, account, receiver, callback_contract,
            market, long_token_swap_path, short_token_swap_path, market_token_amount,
            min_long_token_amount, min_short_token_amount, updated_at_block, execution_fee,
            callback_gas_limit
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10,
            $11, $12, $13, $14,
            $15
        )",
        &[
            &withdrawal.block_number, &withdrawal.transaction_hash, &withdrawal.key, &withdrawal.account, 
            &withdrawal.receiver, &withdrawal.callback_contract, &withdrawal.market, 
            &withdrawal.long_token_swap_path, &withdrawal.short_token_swap_path, &withdrawal.market_token_amount, 
            &withdrawal.min_long_token_amount, &withdrawal.min_short_token_amount, &withdrawal.updated_at_block, 
            &withdrawal.execution_fee, &withdrawal.callback_gas_limit
        ],
    ).await
}
