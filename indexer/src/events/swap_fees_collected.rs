use tokio_postgres::Client;
use crate::events::event::GenericEvent;

#[derive(Debug)]
pub struct SwapFeesCollected {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub market: Option<String>,
    pub token: Option<String>,
    pub token_price: Option<i64>,
    pub action: Option<String>,
    pub fee_receiver_amount: Option<i64>,
    pub fee_amount_for_pool: Option<i64>,
    pub amount_after_fees: Option<i64>,
    pub ui_fee_receiver: Option<String>,
    pub ui_fee_receiver_factor: Option<i64>,
    pub ui_fee_amount: Option<i64>,
}

impl SwapFeesCollected {
    pub const SWAP_FEES_COLLECTED_KEY: &'static str = "035c99b746450c623be607459294d15f458678f99d535718db6cfcbccb117c09";

    pub fn from_generic_event(event: GenericEvent) -> Self {
        let data_parts: Vec<Option<String>> = event.data.split(',').map(|s| Some(s.to_string())).collect();
        SwapFeesCollected {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            key: event.key,
            market: data_parts.get(0).cloned().unwrap_or(None),
            token: data_parts.get(1).cloned().unwrap_or(None),
            token_price: data_parts.get(2).and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            action: data_parts.get(3).cloned().unwrap_or(None),
            fee_receiver_amount: data_parts.get(4).and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            fee_amount_for_pool: data_parts.get(5).and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            amount_after_fees: data_parts.get(6).and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            ui_fee_receiver: data_parts.get(7).cloned().unwrap_or(None),
            ui_fee_receiver_factor: data_parts.get(8).and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
            ui_fee_amount: data_parts.get(9).and_then(|s| s.as_ref().map(|v| v.parse::<i64>().ok()).flatten()),
        }
    }

    pub async fn insert(&self, client: &Client) -> Result<u64, tokio_postgres::Error> {
        client.execute(
            "INSERT INTO swap_fees_collected (
                block_number, transaction_hash, key, market, token, token_price,
                action, fee_receiver_amount, fee_amount_for_pool, amount_after_fees,
                ui_fee_receiver, ui_fee_receiver_factor, ui_fee_amount
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10,
                $11, $12, $13
            )",
            &[
                &self.block_number, &self.transaction_hash, &self.key, &self.market, 
                &self.token, &self.token_price, &self.action, &self.fee_receiver_amount, 
                &self.fee_amount_for_pool, &self.amount_after_fees, &self.ui_fee_receiver, 
                &self.ui_fee_receiver_factor, &self.ui_fee_amount
            ],
        ).await
    }
}
