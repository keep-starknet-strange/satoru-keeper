use tokio_postgres::Client;
use crate::events::event::GenericEvent;

#[derive(Debug)]
pub struct Deposit {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub account: Option<String>,
    pub receiver: Option<String>,
    pub callback_contract: Option<String>,
    pub market: Option<String>,
    pub initial_long_token: Option<String>,
    pub initial_short_token: Option<String>,
    pub long_token_swap_path: Option<String>,
    pub short_token_swap_path: Option<String>,
    pub initial_long_token_amount: Option<String>,
    pub initial_short_token_amount: Option<String>,
    pub min_market_tokens: Option<String>,
    pub updated_at_block: Option<String>,
    pub execution_fee: Option<String>,
    pub callback_gas_limit: Option<String>,
}


impl Deposit {
    pub const DEPOSIT_KEY: &'static str = "00ee02d31cafad9001fbdc4dd5cf4957e152a372530316a7d856401e4c5d74bd";

    pub fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> = event.data.split(',').map(|s| Some(s.to_string())).collect();
        Deposit {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: event.key,
            account: data_parts.get(0).cloned().unwrap_or(None),
            receiver: data_parts.get(1).cloned().unwrap_or(None),
            callback_contract: data_parts.get(2).cloned().unwrap_or(None),
            market: data_parts.get(3).cloned().unwrap_or(None),
            initial_long_token: data_parts.get(4).cloned().unwrap_or(None),
            initial_short_token: data_parts.get(5).cloned().unwrap_or(None),
            long_token_swap_path: data_parts.get(6).cloned().unwrap_or(None),
            short_token_swap_path: data_parts.get(7).cloned().unwrap_or(None),
            initial_long_token_amount: data_parts.get(8).cloned().unwrap_or(None),
            initial_short_token_amount: data_parts.get(9).cloned().unwrap_or(None),
            min_market_tokens: data_parts.get(10).cloned().unwrap_or(None),
            updated_at_block: data_parts.get(11).cloned().unwrap_or(None),
            execution_fee: data_parts.get(12).cloned().unwrap_or(None),
            callback_gas_limit: data_parts.get(13).cloned().unwrap_or(None),
        }
    }

    pub async fn insert(&self, client: &Client) -> Result<u64, tokio_postgres::Error> {
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
                &self.block_number, &self.transaction_hash, &self.key, &self.account, 
                &self.receiver, &self.callback_contract, &self.market, &self.initial_long_token, 
                &self.initial_short_token, &self.long_token_swap_path, &self.short_token_swap_path, 
                &self.initial_long_token_amount, &self.initial_short_token_amount, &self.min_market_tokens, 
                &self.updated_at_block, &self.execution_fee, &self.callback_gas_limit
            ],
        ).await
    }
}
