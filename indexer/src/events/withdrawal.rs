use tokio_postgres::Client;
use crate::events::event::GenericEvent;

#[derive(Debug)]
pub struct Withdrawal {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub account: Option<String>,
    pub receiver: Option<String>,
    pub callback_contract: Option<String>,
    pub market: Option<String>,
    pub long_token_swap_path: Option<String>,
    pub short_token_swap_path: Option<String>,
    pub market_token_amount: Option<String>,
    pub min_long_token_amount: Option<String>,
    pub min_short_token_amount: Option<String>,
    pub updated_at_block: Option<String>,
    pub execution_fee: Option<String>,
    pub callback_gas_limit: Option<String>,
}


impl Withdrawal {
    pub const WITHDRAWAL_KEY: &'static str = "02021e2242f6c652ae824bc1428ee0fe7e8771a27295b9450792445dc456e37d";

    pub fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> = event.data.split(',').map(|s| Some(s.to_string())).collect();
        Withdrawal {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: event.key,
            account: data_parts.get(0).cloned().unwrap_or(None),
            receiver: data_parts.get(1).cloned().unwrap_or(None),
            callback_contract: data_parts.get(2).cloned().unwrap_or(None),
            market: data_parts.get(3).cloned().unwrap_or(None),
            long_token_swap_path: data_parts.get(4).cloned().unwrap_or(None),
            short_token_swap_path: data_parts.get(5).cloned().unwrap_or(None),
            market_token_amount: data_parts.get(6).cloned().unwrap_or(None),
            min_long_token_amount: data_parts.get(7).cloned().unwrap_or(None),
            min_short_token_amount: data_parts.get(8).cloned().unwrap_or(None),
            updated_at_block: data_parts.get(9).cloned().unwrap_or(None),
            execution_fee: data_parts.get(10).cloned().unwrap_or(None),
            callback_gas_limit: data_parts.get(11).cloned().unwrap_or(None),
        }
    }
    pub async fn insert(&self, client: &Client) -> Result<u64, tokio_postgres::Error> {
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
                &self.block_number, &self.transaction_hash, &self.key, &self.account, 
                &self.receiver, &self.callback_contract, &self.market, 
                &self.long_token_swap_path, &self.short_token_swap_path, &self.market_token_amount, 
                &self.min_long_token_amount, &self.min_short_token_amount, &self.updated_at_block, 
                &self.execution_fee, &self.callback_gas_limit
            ],
        ).await
    }
}