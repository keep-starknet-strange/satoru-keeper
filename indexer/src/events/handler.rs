use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, EventFilter, EmittedEvent},
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    providers::Provider,
};
use tokio_postgres::Client;
use hex;
use crate::events::{event::{GenericEvent, EventType}, order::Order, deposit::Deposit, withdrawal::Withdrawal,
 market_created::MarketCreated, swap_fees_collected::SwapFeesCollected, swap_info::SwapInfo, pool_amount_updated::PoolAmountUpdated};



pub struct Indexer<'a> {
    provider: &'a JsonRpcClient<HttpTransport>,
    client: &'a Client,
}

impl<'a> Indexer<'a> {
    pub fn new(provider: &'a JsonRpcClient<HttpTransport>, client: &'a Client) -> Self {
        Indexer { provider, client }
    }

    pub async fn fetch_and_process_events(&self) -> Result<(), tokio_postgres::Error> {
        let order_created_key = FieldElement::from_hex_be(Order::ORDER_KEY).unwrap();
        let deposit_created_key = FieldElement::from_hex_be(Deposit::DEPOSIT_KEY).unwrap();
        let withdrawal_created_key = FieldElement::from_hex_be(Withdrawal::WITHDRAWAL_KEY).unwrap();
        let market_created_key = FieldElement::from_hex_be(MarketCreated::MARKET_KEY).unwrap();
        let swap_fees_collected_key = FieldElement::from_hex_be(SwapFeesCollected::SWAP_FEES_COLLECTED_KEY).unwrap();
        let swap_info_key = FieldElement::from_hex_be(SwapInfo::SWAP_INFO_KEY).unwrap();
        let pool_amount_updated_key = FieldElement::from_hex_be(PoolAmountUpdated::POOL_AMOUNT_UPDATED_KEY).unwrap();


        let event_filter = self.create_event_filter(&[
            order_created_key, 
            deposit_created_key, 
            withdrawal_created_key,
            market_created_key,
            swap_fees_collected_key, 
            swap_info_key,
            pool_amount_updated_key
        ]);

        match self.provider.get_events(event_filter, None, 100).await {
            Ok(events_page) => {
                for event in events_page.events {
                    self.process_event(&event, order_created_key, deposit_created_key, withdrawal_created_key,
                         market_created_key, swap_fees_collected_key,swap_info_key,pool_amount_updated_key ).await?;
                }
            },
            Err(e) => {
                println!("Failed to fetch events: {:?}", e);
            }
        }

        Ok(())
    }

    fn create_event_filter(&self, keys: &[FieldElement]) -> EventFilter {
        EventFilter {
            from_block: Some(BlockId::Number(64406)),
            to_block: Some(BlockId::Tag(BlockTag::Latest)),
            address: FieldElement::from_hex_be("0x2cf721c0387704095d6b2205b46e17d7768fa55c2f1a1087425b877b72937db").ok(),
            keys: Some(vec![keys.to_vec()]),
        }
    }

    async fn process_event(
        &self,
        event: &EmittedEvent,
        order_created_key: FieldElement,
        deposit_created_key: FieldElement,
        withdrawal_created_key: FieldElement,
        market_created_key: FieldElement,
        swap_fees_collected_key: FieldElement,
        swap_info_key: FieldElement,
        pool_amount_updated_key: FieldElement
    ) -> Result<(), tokio_postgres::Error> {
        println!("Event found: {:?}", event);
        let block_number = event.block_number as i64;
        let transaction_hash = hex::encode(event.transaction_hash.to_bytes_be());
        let key = event.keys.first().map(|k| hex::encode(k.to_bytes_be()));
        let data = event.data.iter()
            .map(|fe| hex::encode(fe.to_bytes_be()))
            .collect::<Vec<_>>()
            .join(",");

        if event.keys.contains(&order_created_key) {
            let order_event = GenericEvent {
                block_number,
                transaction_hash: transaction_hash.clone(),
                key: key.clone(),
                data,
            };
            self.process_specific_event(EventType::Order(order_event)).await?;
        } else if event.keys.contains(&deposit_created_key) {
            let deposit_event = GenericEvent {
                block_number,
                transaction_hash: transaction_hash.clone(),
                key: key.clone(),
                data,
            };
            self.process_specific_event(EventType::Deposit(deposit_event)).await?;
        } else if event.keys.contains(&withdrawal_created_key) {
            let withdrawal_event = GenericEvent {
                block_number,
                transaction_hash: transaction_hash.clone(),
                key: key.clone(),
                data,
            };
            self.process_specific_event(EventType::Withdrawal(withdrawal_event)).await?;
        } else if event.keys.contains(&market_created_key) {
            let market_event = GenericEvent {
                block_number,
                transaction_hash: transaction_hash.clone(),
                key: key.clone(),
                data,
            };
            self.process_specific_event(EventType::MarketCreated(market_event)).await?;
        } else if event.keys.contains(&swap_fees_collected_key) {
            let swap_fees_event = GenericEvent {
                block_number,
                transaction_hash: transaction_hash.clone(),
                key: key.clone(),
                data,
            };
            self.process_specific_event(EventType::SwapFeesCollected(swap_fees_event)).await?;
        } else if event.keys.contains(&swap_info_key) {
            let swap_info_event = GenericEvent {
                block_number,
                transaction_hash: transaction_hash.clone(),
                key: key.clone(),
                data,
            };
            self.process_specific_event(EventType::SwapInfo(swap_info_event)).await?;
        } else if event.keys.contains(&pool_amount_updated_key) {
            let pool_amount_updated_event = GenericEvent {
                block_number,
                transaction_hash: transaction_hash.clone(),
                key: key.clone(),
                data,
            };
            self.process_specific_event(EventType::PoolAmountUpdated(pool_amount_updated_event)).await?;
        } else {
            println!("Unknown event type: {:?}", event);
        }

        Ok(())
    }

    async fn process_specific_event(&self, event_type: EventType) -> Result<(), tokio_postgres::Error> {
        match event_type {
            EventType::Order(event) => {
                let order = Order::from_generic_event(event);
                order.insert(&self.client).await?;
            },
            EventType::Deposit(event) => {
                let deposit = Deposit::from_generic_event(event);
                deposit.insert(&self.client).await?;
            },
            EventType::Withdrawal(event) => {
                let withdrawal = Withdrawal::from_generic_event(event);
                withdrawal.insert(&self.client).await?;
            },
            EventType::MarketCreated(event) => {
                let market = MarketCreated::from_generic_event(event);
                market.insert(&self.client).await?;
            },
            EventType::SwapFeesCollected(event) => {
                let swap_fees = SwapFeesCollected::from_generic_event(event);
                swap_fees.insert(&self.client).await?;
            },
            EventType::SwapInfo(event) => {
                let swap_info = SwapInfo::from_generic_event(event);
                swap_info.insert(&self.client).await?;
            },
            EventType::PoolAmountUpdated(event) => {
                let pool_amount_updated = PoolAmountUpdated::from_generic_event(event);
                pool_amount_updated.insert(&self.client).await?;
            },
        }
        Ok(())
    }
}