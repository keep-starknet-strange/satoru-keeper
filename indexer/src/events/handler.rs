use crate::blockchain::head_chain::HeadChain;
use crate::config::get_contract_address;
use crate::events::event::{Event, GenericEvent};
use sqlx::postgres::PgPool;
use starknet::core::types::{BlockId, BlockTag, EventFilter, FieldElement, MaybePendingBlockWithTxHashes};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use std::collections::HashMap;

pub struct EventIndexer<'a> {
    provider: &'a JsonRpcClient<HttpTransport>,
    pool: &'a PgPool,
    event_processors: HashMap<&'static str, Box<dyn EventProcessor + Send + Sync>>,
    head_chain: HeadChain,
}

impl<'a> EventIndexer<'a> {
    pub fn new(
        provider: &'a JsonRpcClient<HttpTransport>,
        pool: &'a PgPool,
        event_processors: HashMap<&'static str, Box<dyn EventProcessor + Send + Sync>>,
        head_chain: HeadChain,
    ) -> Self {
        EventIndexer {
            provider,
            pool,
            event_processors,
            head_chain,
        }
    }

    pub async fn fetch_and_process_events(&self, from_block: u64) -> Result<(), sqlx::Error> {
        let keys: Vec<FieldElement> = self
            .event_processors
            .keys()
            .map(|key| FieldElement::from_hex_be(key).unwrap())
            .collect();

        let contract_address = FieldElement::from_hex_be(&get_contract_address()).unwrap();
        let event_filter = EventFilter {
            from_block: Some(BlockId::Number(from_block)),
            to_block: Some(BlockId::Tag(BlockTag::Latest)),
            address: Some(contract_address),
            keys: Some(vec![keys]),
        };

        let events_page = self
            .provider
            .get_events(event_filter, None, 100)
            .await
            .map_err(|e| sqlx::Error::Protocol(format!("{:?}", e)))?;

        for event in events_page.events {
            let block = self.provider.get_block_with_tx_hashes(BlockId::Number(event.block_number)).await.unwrap();
            let timestamp = match block {
                MaybePendingBlockWithTxHashes::Block(block) => Some(block.timestamp.to_string()),
                MaybePendingBlockWithTxHashes::PendingBlock(block) => Some(block.timestamp.to_string()),
            };
            let generic_event = GenericEvent {
                block_number: event.block_number as i64,
                timestamp,
                transaction_hash: hex::encode(event.transaction_hash.to_bytes_be()),
                key: event.keys.first().map(|k| hex::encode(k.to_bytes_be())),
                data: event.data.iter().map(|fe| hex::encode(fe.to_bytes_be())).collect::<Vec<_>>().join(","),
            };
            if let Some(key) = event.keys.first() {
                let key_str = hex::encode(key.to_bytes_be());
                if let Some(processor) = self.event_processors.get(&key_str.as_str()) {
                    processor.process_event(generic_event.clone(), &self.pool).await?;
                }
                self.head_chain.update_last_block_indexed(event.block_number as i64).await?;
            }
        }

        Ok(())
    }

    pub async fn fetch_pending_events(&self) -> Result<(), sqlx::Error> {
        let event_filter = EventFilter {
            from_block: Some(BlockId::Tag(BlockTag::Pending)),
            to_block: Some(BlockId::Tag(BlockTag::Pending)),
            address: Some(
                FieldElement::from_hex_be(
                    "0x2cf721c0387704095d6b2205b46e17d7768fa55c2f1a1087425b877b72937db",
                )
                .unwrap(),
            ),
            keys: Some(vec![self
                .event_processors
                .keys()
                .map(|key| FieldElement::from_hex_be(key).unwrap())
                .collect()]),
        };

        let events_page = self
            .provider
            .get_events(event_filter, None, 100)
            .await
            .map_err(|e| sqlx::Error::Protocol(format!("{:?}", e)))?;

        for event in events_page.events {
            let block = self.provider.get_block_with_tx_hashes(BlockId::Number(event.block_number)).await.unwrap();
            let timestamp = match block {
                MaybePendingBlockWithTxHashes::Block(block) => Some(block.timestamp.to_string()),
                MaybePendingBlockWithTxHashes::PendingBlock(block) => Some(block.timestamp.to_string()),
            };
            let generic_event = GenericEvent {
                block_number: event.block_number as i64,
                timestamp,
                transaction_hash: hex::encode(event.transaction_hash.to_bytes_be()),
                key: event.keys.first().map(|k| hex::encode(k.to_bytes_be())),
                data: event.data.iter().map(|fe| hex::encode(fe.to_bytes_be())).collect::<Vec<_>>().join(","),
            };
            if let Some(key) = event.keys.first() {
                let key_str = hex::encode(key.to_bytes_be());
                if let Some(processor) = self.event_processors.get(&key_str.as_str()) {
                    processor.process_event(generic_event.clone(), &self.pool).await?;
                }
                self.head_chain.update_last_block_indexed(event.block_number as i64).await?;
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait EventProcessor {
    async fn process_event(&self, event: GenericEvent, pool: &PgPool) -> Result<(), sqlx::Error>;
}

pub struct GenericEventProcessor<T: Event + Send + Sync> {
    pub _marker: std::marker::PhantomData<T>,
}

#[async_trait::async_trait]
impl<T: Event + Send + Sync> EventProcessor for GenericEventProcessor<T> {
    async fn process_event(&self, event: GenericEvent, pool: &PgPool) -> Result<(), sqlx::Error> {
        let specific_event = T::from_generic_event(event);
        println!("Inserting event: {:?}", specific_event);
        specific_event.insert(pool).await
    }
}
