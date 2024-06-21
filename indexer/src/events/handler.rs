use crate::events::event::{Event, GenericEvent};
use sqlx::postgres::PgPool;
use starknet::core::types::{BlockId, BlockTag, EmittedEvent, EventFilter, FieldElement};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use std::collections::HashMap;

pub struct EventIndexer<'a> {
    provider: &'a JsonRpcClient<HttpTransport>,
    pool: &'a PgPool,
    event_processors: HashMap<&'static str, Box<dyn EventProcessor + Send + Sync>>,
}

impl<'a> EventIndexer<'a> {
    pub fn new(
        provider: &'a JsonRpcClient<HttpTransport>,
        pool: &'a PgPool,
        event_processors: HashMap<&'static str, Box<dyn EventProcessor + Send + Sync>>,
    ) -> Self {
        EventIndexer {
            provider,
            pool,
            event_processors,
        }
    }

    pub async fn fetch_and_process_events(&self) -> Result<(), sqlx::Error> {
        let keys: Vec<FieldElement> = self
            .event_processors
            .keys()
            .map(|key| FieldElement::from_hex_be(key).unwrap())
            .collect();

        let event_filter = EventFilter {
            from_block: Some(BlockId::Number(64539)),
            to_block: Some(BlockId::Tag(BlockTag::Latest)),
            address: Some(
                FieldElement::from_hex_be(
                    "0x2cf721c0387704095d6b2205b46e17d7768fa55c2f1a1087425b877b72937db",
                )
                .unwrap(),
            ),
            keys: Some(vec![keys]),
        };

        let events_page = self
            .provider
            .get_events(event_filter, None, 100)
            .await
            .map_err(|e| sqlx::Error::Protocol(format!("{:?}", e)))?;

        for event in events_page.events {
            if let Some(key) = event.keys.first() {
                let key_str = hex::encode(key.to_bytes_be());
                if let Some(processor) = self.event_processors.get(&key_str.as_str()) {
                    processor.process_event(event, &self.pool).await?;
                }
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait EventProcessor {
    async fn process_event(&self, event: EmittedEvent, pool: &PgPool) -> Result<(), sqlx::Error>;
}

pub struct GenericEventProcessor<T: Event + Send + Sync> {
    pub _marker: std::marker::PhantomData<T>,
}

#[async_trait::async_trait]
impl<T: Event + Send + Sync> EventProcessor for GenericEventProcessor<T> {
    async fn process_event(&self, event: EmittedEvent, pool: &PgPool) -> Result<(), sqlx::Error> {
        let block_number = event.block_number as i64;
        let transaction_hash = hex::encode(event.transaction_hash.to_bytes_be());
        let key = event.keys.first().map(|k| hex::encode(k.to_bytes_be()));
        let data = event
            .data
            .iter()
            .map(|fe| hex::encode(fe.to_bytes_be()))
            .collect::<Vec<_>>()
            .join(",");

        let generic_event = GenericEvent {
            block_number,
            transaction_hash,
            key,
            data,
        };

        let specific_event = T::from_generic_event(generic_event);
        println!("Inserting event: {:?}", specific_event);
        specific_event.insert(pool).await
    }
}